# AINPCEngine Rust SDK

Rust client library for integrating AI-powered NPCs into Rust game engines (Bevy, Macroquad, Fyrox, Eldiron, etc.).

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
ainpc-sdk = "0.1"
```

## Quick Start

```rust
use ainpc_sdk::{AINPCClient, GameContext};

#[tokio::main]
async fn main() {
    let client = AINPCClient::new(
        "http://localhost:16745",  // API URL
        "your-api-key",            // API key
        "my-rpg-game",             // Game ID
    );

    let context = GameContext {
        location: Some("village_square".into()),
        time_of_day: Some("morning".into()),
        weather: Some("sunny".into()),
        player_reputation: Some(75),
        player_level: Some(12),
        nearby_npcs: None,
        active_quests: None,
        custom: None,
    };

    let result = client.say("npc_1", "player_1", "Hello there!", context).await.unwrap();

    println!("NPC says: {}", result.response.dialogue);
    println!("Emotion: {}", result.response.emotion);
}
```

## WebSocket (Real-time)

```rust
use ainpc_sdk::NPCWebSocket;

#[tokio::main]
async fn main() {
    let (ws, mut rx) = NPCWebSocket::connect(
        "ws://localhost:16745/ws",
        "your-api-key",
        "my-game",
    ).await.unwrap();

    // Listen for responses in background
    tokio::spawn(async move {
        while let Some(event) = rx.recv().await {
            println!("NPC: {}", event.response.dialogue);
        }
    });

    // Subscribe to NPC updates
    ws.subscribe(&["npc_1", "npc_2"]).await.unwrap();

    // Send event
    ws.send_event("npc_1", serde_json::json!({
        "type": "player_dialogue",
        "playerId": "player_1",
        "message": "What wares do you have?",
        "context": { "location": "market_stall" }
    })).await.unwrap();
}
```

## API Methods

### REST Client (`AINPCClient`)

| Method | Description |
|--------|-------------|
| `health()` | API health check |
| `create_npc(request)` | Create a new NPC |
| `get_npc(id)` | Get NPC details |
| `list_npcs()` | List all NPCs |
| `delete_npc(id)` | Remove an NPC |
| `send_event(npc_id, event)` | Send any game event |
| `say(npc_id, player_id, message, context)` | Dialogue shortcut |
| `approach(npc_id, player_id, context)` | Player approached |
| `request_trade(npc_id, player_id, message, context)` | Trade request |
| `start_combat(npc_id, player_id, context)` | Combat started |
| `player_left(npc_id, player_id, context)` | Player left area |
| `send_batch_events(events)` | Batch multiple events |
| `generate_npc(role, name)` | AI-generate an NPC persona |
| `generate_batch(count, role)` | Batch generate NPCs |

### WebSocket Client (`NPCWebSocket`)

| Method | Description |
|--------|-------------|
| `connect(url, api_key, game_id)` | Connect to server (returns handle + receiver) |
| `send_event(npc_id, event_data)` | Send real-time event |
| `subscribe(npc_ids)` | Subscribe to NPC updates |
| `unsubscribe(npc_ids)` | Unsubscribe from updates |
| `disconnect()` | Close connection |

## Event Types

| Event | Description |
|-------|-------------|
| `player_dialogue` | Player speaks to NPC |
| `player_approached` | Player enters NPC area |
| `trade_requested` | Player wants to trade |
| `combat_started` | Combat initiated |
| `player_left` | Player leaves area |
| `quest_completed` | Quest finished |
| `item_given` | Item transferred to NPC |
| `environment_changed` | World state change |

## License

MIT License - Tyga.Cloud Ltd.
