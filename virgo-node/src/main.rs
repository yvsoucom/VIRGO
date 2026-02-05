// -----------------------------------------------------------------------------
// VIRGO Node - Rust Implementation
// -----------------------------------------------------------------------------
// Copyright (c) 2026 Lican Huang
// Conception & design: Lican Huang
// Implementation: Lican Huang
//
// Licensed under the MIT License
// -----------------------------------------------------------------------------

use anyhow::Result;
use rand::Rng;
use serde::{Serialize, Deserialize};
use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::Arc,
    time::Duration,
};
use tokio::{
    net::UdpSocket,
    sync::RwLock,
    time::interval,
};

/// PathID represents a canonical or partial path
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct PathID(pub String);

/// CanonicalPath: full path + role path
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CanonicalPath {
    pub full_path: PathID,
    pub role_path: PathID, // leaf -> upper
}

/// Node with multiple canonical identities
#[derive(Clone, Debug)]
pub struct Node {
    pub node_id: String,
    pub identities: Vec<CanonicalPath>,
}

/// Endpoint types
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum EndpointType {
    Public,
    Private,
    Tunnel,
}

/// Endpoint info
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Endpoint {
    pub node_id: String,
    pub addr: String, // ip:port
    pub ep_type: EndpointType,
    pub score: u32,
}

/// Routing table: role_path -> endpoints
#[derive(Debug, Default)]
pub struct RouteTable {
    table: HashMap<PathID, Vec<Endpoint>>,
}

impl RouteTable {
    pub fn new() -> Self { Self { table: HashMap::new() } }

    pub fn add_entry(&mut self, role: PathID, ep: Endpoint) {
        self.table.entry(role).or_default().push(ep);
    }

    pub fn get_all(&self, role: &PathID) -> Option<&Vec<Endpoint>> {
        self.table.get(role)
    }

    /// Sort endpoints by score descending
    pub fn get_best_endpoints(&self, role: &PathID) -> Option<Vec<Endpoint>> {
        let endpoints = self.table.get(role)?.clone();
        let mut sorted = endpoints;
        sorted.sort_by(|a, b| b.score.cmp(&a.score));
        Some(sorted)
    }

    /// Update endpoint scores dynamically
    pub fn update_scores(&mut self) {
        for eps in self.table.values_mut() {
            for ep in eps.iter_mut() {
                let base = match ep.ep_type {
                    EndpointType::Public => 150,
                    EndpointType::Private => 80,
                    EndpointType::Tunnel => 50,
                };
                let fluct: i32 = rand::thread_rng().gen_range(-10..=10);
                ep.score = ((base as i32 + fluct).max(0)) as u32;
            }
        }
    }
}

/// Heartbeat message
#[derive(Serialize, Deserialize, Debug)]
pub struct Heartbeat {
    pub node_id: String,
    pub role_path: String,
}

/// Coordinator info
#[derive(Clone, Debug)]
pub struct Coordinator {
    pub node_id: String,
    pub endpoint: Endpoint,
    pub last_seen: tokio::time::Instant,
}

/// Check if endpoint is eligible for role (member or child coordinator)
fn is_eligible(_ep: &Endpoint, _role: &PathID, _coordinators: &HashMap<PathID, Coordinator>) -> bool {
    // In practice, implement membership/child check
    true
}

#[tokio::main]
async fn main() -> Result<()> {
    // --- Example node ---
    let node = Node {
        node_id: "claerk".to_string(),
        identities: vec![
            CanonicalPath {
                full_path: PathID("all.science.cs.ai.001::claerk".into()),
                role_path: PathID("cs.ai.001".into()),
            },
            CanonicalPath {
                full_path: PathID("all.science.ai.002::claerk".into()),
                role_path: PathID("ai.002".into()),
            },
        ],
    };

    // --- Routing table ---
    let route_table = Arc::new(RwLock::new(RouteTable::new()));
    {
        let mut rt = route_table.write().await;
        // cs.ai.001 endpoints
        rt.add_entry(PathID("cs.ai.001".into()), Endpoint { node_id: "A".into(), addr: "203.0.113.10:4000".into(), ep_type: EndpointType::Public, score: 150 });
        rt.add_entry(PathID("cs.ai.001".into()), Endpoint { node_id: "B".into(), addr: "10.0.0.12:4000".into(), ep_type: EndpointType::Private, score: 80 });
        rt.add_entry(PathID("cs.ai.001".into()), Endpoint { node_id: "C".into(), addr: "100.64.0.5:4000".into(), ep_type: EndpointType::Tunnel, score: 50 });
        // ai.002 endpoints
        rt.add_entry(PathID("ai.002".into()), Endpoint { node_id: "C".into(), addr: "203.0.113.20:5000".into(), ep_type: EndpointType::Public, score: 150 });
        rt.add_entry(PathID("ai.002".into()), Endpoint { node_id: "D".into(), addr: "10.0.0.22:5000".into(), ep_type: EndpointType::Private, score: 80 });
    }

    // --- Coordinator map ---
    let coordinators = Arc::new(RwLock::new(HashMap::<PathID, Coordinator>::new()));

    // --- Bind UDP socket ---
    let socket = Arc::new(UdpSocket::bind("0.0.0.0:0").await?);
    println!("Node {} listening on {}", node.node_id, socket.local_addr()?);

    let socket_send = socket.clone();
    let rt_clone = route_table.clone();
    let coor_clone = coordinators.clone();
    let node_clone = node.clone();

    // --- Heartbeat + coordinator election task ---
    tokio::spawn(async move {
        let mut ticker = interval(Duration::from_secs(5));
        loop {
            ticker.tick().await;

            let mut rt = rt_clone.write().await;
            rt.update_scores();

            let mut coor_map = coor_clone.write().await;

            // bottom-to-top election
            for identity in &node_clone.identities {
                let role = &identity.role_path;
                let current = coor_map.get(role);
                let need_reselect = match current {
                    Some(coord) => coord.last_seen.elapsed() > Duration::from_secs(15),
                    None => true,
                };
                if !need_reselect { continue; }

                if let Some(endpoints) = rt.get_best_endpoints(role) {
                    // only eligible nodes
                    let eligible: Vec<_> = endpoints.iter()
                        .filter(|ep| is_eligible(ep, role, &coor_map))
                        .collect();
                    if let Some(best_ep) = eligible.first() {
                        coor_map.insert(role.clone(), Coordinator {
                            node_id: best_ep.node_id.clone(),
                            endpoint: (*best_ep).clone(),
                            last_seen: tokio::time::Instant::now(),
                        });
                        println!("New coordinator for {}: {} ({:?}, score {})",
                            role.0, best_ep.node_id, best_ep.ep_type, best_ep.score);
                    }
                }
            }

            // Send heartbeat to coordinators
            for identity in &node_clone.identities {
                let role = &identity.role_path;
                let hb = Heartbeat {
                    node_id: node_clone.node_id.clone(),
                    role_path: role.0.clone(),
                };
                let payload = serde_json::to_vec(&hb).unwrap();
                if let Some(coord) = coor_map.get(role) {
                    if let Ok(addr) = coord.endpoint.addr.parse::<SocketAddr>() {
                        let _ = socket_send.send_to(&payload, addr).await;
                        println!("Sent heartbeat to {} for role {}", coord.endpoint.addr, role.0);
                    }
                }
            }
        }
    });

    // --- Receive messages ---
    let mut buf = vec![0u8; 1024];
    loop {
        let (len, addr) = socket.recv_from(&mut buf).await?;
        if let Ok(hb) = serde_json::from_slice::<Heartbeat>(&buf[..len]) {
            println!("Received heartbeat from {} (role {}) at {}", hb.node_id, hb.role_path, addr);
            // update last_seen if coordinator
            let mut coor_map = coordinators.write().await;
            if let Some(coord) = coor_map.get_mut(&PathID(hb.role_path.clone())) {
                if coord.node_id == hb.node_id {
                    coord.last_seen = tokio::time::Instant::now();
                }
            }
        }
    }
}
