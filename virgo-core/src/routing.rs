// -----------------------------------------------------------------------------
// VIRGO Node - Rust Implementation
// -----------------------------------------------------------------------------
// Copyright (c) 2026 Lican Huang
// Conception & design: Lican Huang
// Implementation: Lican Huang
//
// Licensed under the MIT License
// -----------------------------------------------------------------------------

use std::net::SocketAddr;
use std::time::{SystemTime, UNIX_EPOCH};

pub type NodeID = u64;
pub type PathID = String;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EndpointKind {
    Public,
    Private,
    Tunnel,
}

#[derive(Debug, Clone)]
pub struct Endpoint {
    pub kind: EndpointKind,
    pub addr: SocketAddr,

    pub last_ok: u64,
    pub last_fail: u64,
    pub score: i32, // higher = better
}

impl Endpoint {
    pub fn new(kind: EndpointKind, addr: SocketAddr) -> Self {
        Self {
            kind,
            addr,
            last_ok: 0,
            last_fail: 0,
            score: kind.base_score(),
        }
    }

    pub fn report_ok(&mut self) {
        self.last_ok = now();
        self.score += 10;
    }

    pub fn report_fail(&mut self) {
        self.last_fail = now();
        self.score -= 20;
    }
}

impl EndpointKind {
    fn base_score(self) -> i32 {
        match self {
            EndpointKind::Public => 100,
            EndpointKind::Private => 80,
            EndpointKind::Tunnel => 50,
        }
    }
}

#[derive(Debug)]
pub struct PathEntry {
    pub path_id: PathID,
    pub node_id: NodeID,
    pub endpoints: Vec<Endpoint>,
}

impl PathEntry {
    /// Select best endpoint dynamically
    pub fn select_endpoint(&mut self) -> Option<&mut Endpoint> {
        self.endpoints
            .sort_by(|a, b| b.score.cmp(&a.score));
        self.endpoints.first_mut()
    }
}

fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
