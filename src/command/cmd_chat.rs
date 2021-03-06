use std::{
    env,
    io::{self as sio, Read, Write, BufRead, BufReader as stdBufReader},
    error::Error,
    path::Path,
    str::SplitWhitespace,
    fs::{File as stdFile, OpenOptions}
};
use futures::{
    executor::block_on,
    StreamExt,
};
use libp2p::{
    core::upgrade,
    floodsub::{self, Floodsub, FloodsubEvent},
    identity,
    mdns::{Mdns, MdnsEvent},
    mplex,
    noise,
    swarm::{dial_opts::DialOpts, NetworkBehaviourEventProcess, SwarmBuilder, SwarmEvent},
    // `TokioTcpConfig` is available through the `tcp-tokio` feature.
    tcp::TokioTcpConfig,
    Multiaddr,
    NetworkBehaviour,
    PeerId,
    Transport,
};
use tokio::{
    io::{
        self,
        AsyncRead,
        AsyncBufReadExt,
        AsyncWriteExt,
        BufWriter
    },
};
use log::{info, debug};
use serde::{Serialize, Deserialize};

use super::cmd::Cmd;
use super::consts::CHAT_USER_DB;  //"./user_db.json"

pub struct CmdChat {
    pub name: String,
}

impl CmdChat {
    // inner execute function for asynchronous programming
    async fn _execute(&self) {
        match execute_chat().await {
            // TODO: should show a more detail
            Ok(_) => println!("Good"),
            Err(e) => println!("{}: {}", self.name, e),
        }
    }
}

impl Cmd for CmdChat {
    type Error = io::Error;

    fn execute(&self, _args: &SplitWhitespace) {
        block_on(self._execute());
    }
    
    fn error_handling(&self, _err: Self::Error) {
        let cmd = self.get_cmd_name();
        // TODO: classify errors below using err value
        println!("{}: {}", cmd, "nothing");
    }

    fn get_cmd_name(&self) -> &str {
        self.name.as_ref()
    }
}

// We create a custom network behaviour that combines floodsub and mDNS.
// The derive generates a delegating `NetworkBehaviour` impl which in turn
// requires the implementations of `NetworkBehaviourEventProcess` for
// the events of each behaviour.
#[derive(NetworkBehaviour)]
#[behaviour(event_process = true)]
struct MyBehaviour {
    floodsub: Floodsub,
    mdns: Mdns,
}

impl NetworkBehaviourEventProcess<FloodsubEvent> for MyBehaviour {
    // Called when `floodsub` produces an event.
    fn inject_event(&mut self, message: FloodsubEvent) {
        if let FloodsubEvent::Message(message) = message {
            let data = String::from_utf8_lossy(&message.data);
            info!(
                "Received: '{:?}' from {:?}",
                data,
                message.source
            );

            if let Some(nick) = 
                check_user_existence(message.source.to_base58().as_ref())
                .unwrap()
            {
                println!("{}: {}", nick, data);
            } else {
                println!("{}: {}", message.source, data);
            }
        }
    }
}

impl NetworkBehaviourEventProcess<MdnsEvent> for MyBehaviour {
    // Called when `mdns` produces an event.
    fn inject_event(&mut self, event: MdnsEvent) {
        match event {
            MdnsEvent::Discovered(list) => {
                for (peer, _) in list {
                    self.floodsub.add_node_to_partial_view(peer);
                }
            }
            MdnsEvent::Expired(list) => {
                for (peer, _) in list {
                    if !self.mdns.has_node(&peer) {
                        self.floodsub.remove_node_from_partial_view(&peer);
                    }
                }
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct User {
    peer_id: String,
    nickname: Vec<u8>,
}

// Copyright 2018 Parity Technologies (UK) Ltd.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
// OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.
//---------------------------------------------------------------------------
// An example from the offical repository
// https://github.com/libp2p/rust-libp2p/blob/master/examples/chat-tokio.rs
//
// I, Hyunmin Shin, fixed the code slightly for convinence.
// [The parts I touched]
// - move pre-defined struct, MyBehavior, to outside
// - printing messages and infos as neat
// - participants have a nickname
// - (not yet) choose a chat channel
// - (not yet) when a session is expired, ask the user to get in again
async fn execute_chat() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();

    // Create a random PeerId
    let id_keys = identity::Keypair::generate_ed25519();
    let peer_id = PeerId::from(id_keys.public());
    info!("Local peer id: {:?}", peer_id);

    // Create a keypair for authenticated encryption of the transport.
    let noise_keys = noise::Keypair::<noise::X25519Spec>::new()
        .into_authentic(&id_keys)
        .expect("Signing libp2p-noise static DH keypair failed.");

    // Create a tokio-based TCP transport use noise for authenticated
    // encryption and Mplex for multiplexing of substreams on a TCP stream.
    let transport = TokioTcpConfig::new()
        .nodelay(true)
        .upgrade(upgrade::Version::V1)
        .authenticate(noise::NoiseConfig::xx(noise_keys).into_authenticated())
        .multiplex(mplex::MplexConfig::new())
        .boxed();

    // Create a Floodsub topic
    let floodsub_topic = floodsub::Topic::new("chat");

    // CUSTOM: user can take his own nickname
    let user: User = init_user(peer_id.to_base58()).await?;
    let my_nickname = String::from_utf8_lossy(&user.nickname).into_owned();

    // Create a Swarm to manage peers and events.
    let mut swarm = {
        let mdns = Mdns::new(Default::default()).await?;
        let mut behaviour = MyBehaviour {
            floodsub: Floodsub::new(peer_id.clone()),
            mdns,
        };

        behaviour.floodsub.subscribe(floodsub_topic.clone());

        SwarmBuilder::new(transport, behaviour, peer_id)
            // We want the connection background tasks to be spawned
            // onto the tokio runtime.
            .executor(Box::new(|fut| {
                tokio::spawn(fut);
            }))
            .build()
    };

    // Reach out to another node if specified
    if let Some(to_dial) = std::env::args().nth(1) {
        let addr: Multiaddr = to_dial.parse()?;
        swarm.dial(addr)?;
        println!("Dialed {:?}", to_dial);
    }

    // Read full lines from stdin
    let mut stdin = io::BufReader::new(io::stdin()).lines();

    // Listen on all interfaces and whatever port the OS assigns
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    // Kick it off
    loop {
        tokio::select! {
            line = stdin.next_line() => {
                let line = line?.expect("stdin closed");
                swarm.behaviour_mut().floodsub.publish(floodsub_topic.clone(), line.as_bytes());
            }
            event = swarm.select_next_some() => {
                if let SwarmEvent::NewListenAddr { address, .. } = event {
                    println!("Listening on {:?}", address);
                }
            }
        }
    }
}

async fn init_user(peer_id: String) -> sio::Result<User> {
    let mut nick = String::new();
    
    loop {
        print!("Nickname you want to show other: ");
        sio::stdout().flush().unwrap();

        let length: usize = sio::stdin().read_line(&mut nick).unwrap();
        if length <= 0 { 
            println!("[init_user] nickname should be larger than zero.");
        } else {
            break;
        }
    }

    let user = User {
        peer_id: peer_id,   // continue to move the ownership
        nickname: nick.trim().as_bytes().to_vec(),
    };

    if let Ok(()) = write_json_file(CHAT_USER_DB, &user) {
        debug!("Successfully appended to the file");
    }

    Ok(user)
}

fn check_user_existence(target_id: &str) -> Result<Option<String>, Box<dyn Error>> {
    let reader = read_json_file(CHAT_USER_DB)?;

    // check existence of target's id
    for line in reader.lines() {
        let user: User = serde_json::from_str(line?.as_ref())?;
        info!("deserialized content: {:?}", user);
        
        if target_id == user.peer_id {
            let s = String::from_utf8_lossy(&user.nickname).into_owned();  
            return Ok(Some(s))
        }
    }

    Ok(None)
}

fn read_json_file<P>(path: P) -> Result<stdBufReader<std::fs::File>, Box<dyn Error>>
    where P: AsRef<Path>,
{
    // TODO: /tmp ??????????????? ?????????
    // Create a temporary file.
    // let temp_directory = env::temp_dir();
    // let temp_file = temp_directory.join("file");

    // tokio::fs::File, not std::fs::File
    let file = OpenOptions::new()
                            .read(true)
                            .open(path)
                            .expect("could not open file");

    // change tokio file to std file for iterator
    let reader = stdBufReader::new(file);

    Ok(reader)
}

fn write_json_file<P, C>(path: P, content: &C) -> Result<(), Box<dyn Error>>
    where P: AsRef<Path>,
          C: ?Sized + Serialize,
{
    // TODO: /tmp ??????????????? ?????????
    // Create a temporary file.
    // let temp_directory = env::temp_dir();
    // let temp_file = temp_directory.join("file");

    let mut file = OpenOptions::new()
                            .append(true)
                            .open(path)
                            .expect("could not open file");
    let serialized = serde_json::to_string(content)?;
    debug!("serialized content: {}", serialized);

    // TODO: ?????? ????????? ?????? ?????? + ????????? ????????????, ????????? ?????? ??????
    // TODO: ?????? ?????? ?????????(?????? json ????????? ?????? ????????? ?????? ??????), ?????? ???????????? ???????
    if let Err(e) = writeln!(file, "{}", serialized) {
        eprintln!("Couldn't write to file: {}", e);
    }

    Ok(())
}
