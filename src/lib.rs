//! # AINPCEngine Rust SDK
//!
//! AI-powered NPCs with persistent memory for Rust game engines.
//!
//! ## Quick Start
//!
//! ```no_run
//! use ainpc_sdk::{AINPCClient, AINPCError, GameContext};
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
//!     let context = GameContext {
//!         location: Some("village_square".into()),
//!         time_of_day: Some("morning".into()),
//!         weather: Some("sunny".into()),
//!         player_reputation: 75,
//!         player_level: 12,
//!         ..Default::default()
//!     };
//!     match client.say("npc_1", "player_1", "Hello there!", context).await {
//!         Ok(result) => println!("NPC says: {}", result.response.dialogue.unwrap_or_default()),
//!         Err(err) if err.is_payment_required() => {
//!             // Trial paywall: add a card to start the 7-day free trial ($0 today)
//!             println!("Activate your account: {}", err.payment_url().unwrap_or_default());
//!         }
//!         Err(err) => eprintln!("Error: {err}"),
//!     }
//! }
//! ```

mod client;
mod models;

#[cfg(feature = "native")]
mod websocket;

pub use client::{error_codes, AINPCClient, AINPCError, Result};
pub use models::*;

#[cfg(feature = "native")]
pub use websocket::NPCWebSocket;
