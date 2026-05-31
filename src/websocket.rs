use crate::models::EventResult;
use futures_util::{SinkExt, StreamExt};
use tokio::sync::mpsc;
use tokio_tungstenite::{connect_async, tungstenite::Message};
use url::Url;

/// Real-time WebSocket client for NPC event streaming.
///
/// # Example
/// ```no_run
/// use ainpc_sdk::NPCWebSocket;
///
/// #[tokio::main]
/// async fn main() {
///     let (ws, mut rx) = NPCWebSocket::connect(
///         "ws://localhost:16745/ws",
///         "your-api-key",
///         "my-game",
///     ).await.unwrap();
///
///     // Listen for NPC responses
///     tokio::spawn(async move {
///         while let Some(event) = rx.recv().await {
///             println!("NPC says: {}", event.response.dialogue);
///         }
///     });
///
///     // Subscribe to NPC updates
///     ws.subscribe(&["npc_1", "npc_2"]).await.unwrap();
///
///     // Send an event
///     ws.send_event("npc_1", serde_json::json!({
///         "type": "player_dialogue",
///         "playerId": "player_1",
///         "message": "Hello!",
///         "context": { "location": "village_square" }
///     })).await.unwrap();
/// }
/// ```
pub struct NPCWebSocket {
    tx: mpsc::Sender<String>,
}

impl NPCWebSocket {
    /// Connect to the AINPCEngine WebSocket server.
    ///
    /// Returns a handle for sending messages and a receiver for NPC responses.
    pub async fn connect(
        base_url: &str,
        api_key: &str,
        game_id: &str,
    ) -> Result<(Self, mpsc::Receiver<EventResult>), Box<dyn std::error::Error + Send + Sync>> {
        let mut url = Url::parse(base_url)?;
        url.query_pairs_mut()
            .append_pair("apiKey", api_key)
            .append_pair("gameId", game_id);

        let (ws_stream, _) = connect_async(url.as_str()).await?;
        let (mut write, mut read) = ws_stream.split();

        // Channel for outbound messages (commands → WebSocket)
        let (cmd_tx, mut cmd_rx) = mpsc::channel::<String>(64);

        // Channel for inbound NPC responses (WebSocket → caller)
        let (event_tx, event_rx) = mpsc::channel::<EventResult>(64);

        // Writer task
        tokio::spawn(async move {
            while let Some(msg) = cmd_rx.recv().await {
                if write.send(Message::Text(msg.into())).await.is_err() {
                    break;
                }
            }
        });

        // Reader task
        tokio::spawn(async move {
            while let Some(Ok(msg)) = read.next().await {
                if let Message::Text(text) = msg {
                    if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&text) {
                        if parsed.get("type").and_then(|t| t.as_str()) == Some("response") {
                            if let Some(data) = parsed.get("data") {
                                if let Ok(result) = serde_json::from_value::<EventResult>(data.clone()) {
                                    let _ = event_tx.send(result).await;
                                }
                            }
                        }
                    }
                }
            }
        });

        Ok((Self { tx: cmd_tx }, event_rx))
    }

    /// Send a game event to a specific NPC.
    pub async fn send_event(
        &self,
        npc_id: &str,
        event_data: serde_json::Value,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let msg = serde_json::json!({
            "action": "event",
            "data": {
                "npcId": npc_id,
                "type": event_data.get("type").and_then(|t| t.as_str()).unwrap_or("player_dialogue"),
                "playerId": event_data.get("playerId"),
                "message": event_data.get("message"),
                "context": event_data.get("context").cloned().unwrap_or(serde_json::json!({})),
            }
        });
        self.tx.send(msg.to_string()).await?;
        Ok(())
    }

    /// Subscribe to real-time updates for specific NPCs.
    pub async fn subscribe(
        &self,
        npc_ids: &[&str],
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let msg = serde_json::json!({
            "action": "subscribe",
            "npcIds": npc_ids,
        });
        self.tx.send(msg.to_string()).await?;
        Ok(())
    }

    /// Unsubscribe from NPC updates.
    pub async fn unsubscribe(
        &self,
        npc_ids: &[&str],
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let msg = serde_json::json!({
            "action": "unsubscribe",
            "npcIds": npc_ids,
        });
        self.tx.send(msg.to_string()).await?;
        Ok(())
    }

    /// Disconnect from the server.
    pub async fn disconnect(self) {
        drop(self.tx);
    }
}
