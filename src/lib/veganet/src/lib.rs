use anyhow::Result;
use libp2p::{Swarm, swarm::SwarmEvent, ping, noise, yamux, tcp, Multiaddr};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub listen_addr: String,
}

pub struct VegaNet {
    swarm: Swarm<ping::Behaviour>,
}

impl VegaNet {
    pub fn new(config: &Config) -> Result<Self> {
        let keypair = libp2p::identity::Keypair::generate_ed25519();
        let peer_id = keypair.public().to_peer_id();
        let transport = tcp::tokio::Transport::new(tcp::Config::default())
            .upgrade(libp2p::core::upgrade::Version::V1)
            .authenticate(noise::Config::new(&keypair)?)
            .multiplex(yamux::Config::default())
            .boxed();
        let behaviour = ping::Behaviour::new(ping::Config::new());
        let mut swarm = Swarm::new(transport, behaviour, peer_id);
        let addr: Multiaddr = config.listen_addr.parse()?;
        swarm.listen_on(addr)?;
        Ok(VegaNet { swarm })
    }

    pub async fn run(&mut self) -> Result<()> {
        loop {
            match self.swarm.select_next_some().await {
                SwarmEvent::NewListenAddr { address, .. } => println!("Listening on {}", address),
                SwarmEvent::Behaviour(ping::Event { peer, result, .. }) => println!("Ping to {}: {:?}", peer, result),
                _ => {}
            }
        }
    }
}
