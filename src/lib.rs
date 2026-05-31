//! # AINPCEngine Rust SDK
//!
//! AI-powered NPCs with persistent memory for Rust game engines.
//!
//! ## Quick Start
//!
//! ```no_run
//! use ainpc_sdk::{AINPCClient, GameEvent, GameContext};
//!
//! #[tokio::main]
//! async fn main() {
//!     let client = AINPCClient::new("http://localhost:16745", "your-api-key", "my-game");
//!
//!     // Health check
//!     let health = client.health().await.unwrap();
//!     println!("Status: {}", health.status);
//!
//!     // Talk to an NPC
//!     let result = client.say("npc_1", "player_1", "Hello there!", GameContext {
//!         location: Some("village_square".into()),
//!         time_of_day: Some("morning".into()),
//!         weather: Some("sunny".into()),
//!         player_reputation: Some(75),
//!         player_level: Some(12),
//!         nearby_npcs: None,
//!         active_quests: None,
//!         custom: None,
//!     }).await.unwrap();
//!
//!     println!("NPC says: {}", result.response.dialogue);
//! }
//! ```

mod client;
mod models;

#[cfg(feature = "native")]
mod websocket;

pub use client::{AINPCClient, AINPCError, Result};
pub use models::*;

#[cfg(feature = "native")]
pub use websocket::NPCWebSocket;
