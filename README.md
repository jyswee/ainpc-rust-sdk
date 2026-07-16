# AINPCEngine Rust SDK

Rust client library for integrating AI-powered NPCs into Rust game engines (Bevy, Macroquad, Fyrox, Eldiron, etc.).

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
ainpc-sdk = "0.2"
```

## Quick Start

```rust
use ainpc_sdk::{AINPCClient, GameContext};

#[tokio::main]
async fn main() {
    let client = AINPCClient::new(
        "https://api.ainpcengine.com",  // API URL
        "npk_your-api-key",             // API key
        "my-rpg-game",                  // Game ID
    );

    let context = GameContext {
        location: Some("village_square".into()),
        time_of_day: Some("morning".into()),
        weather: Some("sunny".into()),
        player_reputation: 75,
        player_level: 12,
        ..Default::default()
    };

    let result = client.say("npc_1", "player_1", "Hello there!", context).await.unwrap();

    println!("NPC says: {}", result.response.dialogue.unwrap_or_default());
    println!("Emotion: {}", result.response.emotion.unwrap_or_default());
}
```

## Updating NPCs

Partial updates — only the fields you set are changed:

```rust
use ainpc_sdk::UpdateNPCRequest;

let updated = client.update_npc("npc_1", &UpdateNPCRequest {
    location: Some("tavern".into()),
    faction: Some("merchants_guild".into()),
    ..Default::default()
}).await?;
```

## NPC Voice (TTS)

Give your NPCs a voice — audio is synthesized with a voice matched to the
NPC's personality. Requires a Voice add-on on your plan.

```rust
let speech = client.speak("npc_1", "Welcome to my shop, traveler!").await?;
// speech.audio is base64-encoded audio, speech.format is e.g. "wav"
println!("Voice: {} ({} chars)", speech.voice, speech.text_length);
```

## Engine Stats

```rust
let stats = client.stats().await?;
println!("{} experts online, uptime {:.0}s", stats.total_experts, stats.uptime);
for expert in &stats.experts {
    println!("  {} ({})", expert.name, expert.domain);
}
```

## Error Handling

Non-2xx API responses are returned as a structured `AINPCError::Api` with
the HTTP status, a machine-readable `code`, a human-readable message, and
(where relevant) a URL to resolve the issue:

```rust
use ainpc_sdk::{AINPCError, error_codes};

match client.say("npc_1", "player_1", "Hi!", Default::default()).await {
    Ok(result) => { /* ... */ }
    Err(err) if err.is_payment_required() => {
        // 402: account not activated yet (or subscription cancelled).
        // Add a card at the payment URL to start your 7-day free trial —
        // $0 today, full API access immediately.
        println!("Activate your account: {}", err.payment_url().unwrap_or_default());
    }
    Err(AINPCError::Api { status, code, message, payment_url }) => {
        match code.as_deref() {
            Some(error_codes::NPC_LIMIT) => {
                // 402: plan NPC limit reached — upgrade to create more
            }
            Some(error_codes::FEATURE_NOT_IN_PLAN) => {
                // 402: feature not included in current plan
            }
            Some(error_codes::INTERACTION_CAP) => {
                // 429: monthly included interactions used up
            }
            _ => eprintln!("API error {status}: {message}"),
        }
        if let Some(url) = payment_url {
            println!("Upgrade at: {url}");
        }
    }
    Err(err) => eprintln!("Network error: {err}"),
}
```

| Code | HTTP | Meaning |
|------|------|---------|
| `payment_required` | 402 | Add a card to start the 7-day free trial ($0 today) or resubscribe |
| `npc_limit` | 402 | Plan NPC limit reached — upgrade to create more |
| `feature_not_in_plan` | 402 | Feature not included in your plan |
| `interaction_cap` | 429 | Monthly included interactions used — upgrade for more |

## WebSocket (Real-time)

```rust
use ainpc_sdk::NPCWebSocket;

#[tokio::main]
async fn main() {
    let (ws, mut rx) = NPCWebSocket::connect(
        "wss://api.ainpcengine.com/ws",
        "npk_your-api-key",
        "my-game",
    ).await.unwrap();

    // Listen for responses in background
    tokio::spawn(async move {
        while let Some(event) = rx.recv().await {
            println!("NPC: {}", event.response.dialogue.unwrap_or_default());
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
| `update_npc(id, updates)` | Partially update an NPC |
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
| `speak(npc_id, text)` | Synthesize NPC voice audio (Voice add-on) |
| `stats()` | Engine stats: experts, features, uptime |

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

## Getting Started with AINPCEngine

Sign up at [ainpcengine.com](https://ainpcengine.com) — every plan starts
with a 7-day free trial ($0 today). Create an API key in your Studio
dashboard and you're ready to go.

## License

MIT License - Tyga.Cloud Ltd.
