use crate::models::*;
use reqwest::Client;

#[derive(Debug, thiserror::Error)]
pub enum AINPCError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("API error: {0}")]
    Api(String),
}

pub type Result<T> = std::result::Result<T, AINPCError>;

/// AINPCEngine REST API client.
///
/// # Example
/// ```no_run
/// use ainpc_sdk::AINPCClient;
///
/// #[tokio::main]
/// async fn main() {
///     let client = AINPCClient::new("http://localhost:16745", "your-api-key", "my-game");
///     let health = client.health().await.unwrap();
///     println!("Status: {}", health.status);
/// }
/// ```
pub struct AINPCClient {
    base_url: String,
    api_key: String,
    game_id: String,
    http: Client,
}

impl AINPCClient {
    pub fn new(base_url: &str, api_key: &str, game_id: &str) -> Self {
        Self {
            base_url: base_url.trim_end_matches('/').to_string(),
            api_key: api_key.to_string(),
            game_id: game_id.to_string(),
            http: Client::new(),
        }
    }

    async fn get<T: serde::de::DeserializeOwned>(&self, path: &str) -> Result<T> {
        let res = self.http.get(format!("{}{}", self.base_url, path))
            .header("x-api-key", &self.api_key)
            .header("x-game-id", &self.game_id)
            .send().await?;
        if !res.status().is_success() {
            let text = res.text().await.unwrap_or_default();
            return Err(AINPCError::Api(text));
        }
        Ok(res.json().await?)
    }

    async fn post<B: serde::Serialize, T: serde::de::DeserializeOwned>(&self, path: &str, body: &B) -> Result<T> {
        let res = self.http.post(format!("{}{}", self.base_url, path))
            .header("x-api-key", &self.api_key)
            .header("x-game-id", &self.game_id)
            .json(body)
            .send().await?;
        if !res.status().is_success() {
            let text = res.text().await.unwrap_or_default();
            return Err(AINPCError::Api(text));
        }
        Ok(res.json().await?)
    }

    async fn delete(&self, path: &str) -> Result<()> {
        let res = self.http.delete(format!("{}{}", self.base_url, path))
            .header("x-api-key", &self.api_key)
            .header("x-game-id", &self.game_id)
            .send().await?;
        if !res.status().is_success() && res.status().as_u16() != 404 {
            let text = res.text().await.unwrap_or_default();
            return Err(AINPCError::Api(text));
        }
        Ok(())
    }

    // ── Health ──────────────────────────────────────────────────────

    pub async fn health(&self) -> Result<HealthResponse> {
        self.get("/api/health").await
    }

    // ── NPC CRUD ────────────────────────────────────────────────────

    pub async fn create_npc(&self, req: &CreateNPCRequest) -> Result<NPC> {
        self.post("/api/npcs", req).await
    }

    pub async fn get_npc(&self, npc_id: &str) -> Result<NPC> {
        self.get(&format!("/api/npcs/{}", npc_id)).await
    }

    pub async fn list_npcs(&self) -> Result<Vec<NPC>> {
        self.get("/api/npcs").await
    }

    pub async fn delete_npc(&self, npc_id: &str) -> Result<()> {
        self.delete(&format!("/api/npcs/{}", npc_id)).await
    }

    // ── Events ──────────────────────────────────────────────────────

    pub async fn send_event(&self, npc_id: &str, event: &GameEvent) -> Result<EventResult> {
        self.post(&format!("/api/npcs/{}/event", npc_id), event).await
    }

    pub async fn say(&self, npc_id: &str, player_id: &str, message: &str, context: GameContext) -> Result<EventResult> {
        let event = GameEvent {
            event_type: "player_dialogue".into(),
            player_id: Some(player_id.into()),
            message: Some(message.into()),
            context,
        };
        self.send_event(npc_id, &event).await
    }

    pub async fn approach(&self, npc_id: &str, player_id: &str, context: GameContext) -> Result<EventResult> {
        let event = GameEvent {
            event_type: "player_approached".into(),
            player_id: Some(player_id.into()),
            message: None,
            context,
        };
        self.send_event(npc_id, &event).await
    }

    pub async fn request_trade(&self, npc_id: &str, player_id: &str, message: &str, context: GameContext) -> Result<EventResult> {
        let event = GameEvent {
            event_type: "trade_requested".into(),
            player_id: Some(player_id.into()),
            message: Some(message.into()),
            context,
        };
        self.send_event(npc_id, &event).await
    }

    pub async fn start_combat(&self, npc_id: &str, player_id: &str, context: GameContext) -> Result<EventResult> {
        let event = GameEvent {
            event_type: "combat_started".into(),
            player_id: Some(player_id.into()),
            message: None,
            context,
        };
        self.send_event(npc_id, &event).await
    }

    pub async fn player_left(&self, npc_id: &str, player_id: &str, context: GameContext) -> Result<EventResult> {
        let event = GameEvent {
            event_type: "player_left".into(),
            player_id: Some(player_id.into()),
            message: None,
            context,
        };
        self.send_event(npc_id, &event).await
    }

    // ── Batch ───────────────────────────────────────────────────────

    pub async fn send_batch_events(&self, events: &[serde_json::Value]) -> Result<BatchEventResult> {
        #[derive(serde::Serialize)]
        struct BatchBody { events: Vec<serde_json::Value> }
        self.post("/api/events", &BatchBody { events: events.to_vec() }).await
    }

    // ── Generation ──────────────────────────────────────────────────

    pub async fn generate_npc(&self, role: &str, name: &str) -> Result<NPC> {
        self.post("/api/npcs/generate", &GenerateNPCRequest {
            role: Some(role.into()),
            name: Some(name.into()),
        }).await
    }

    pub async fn generate_batch(&self, count: u32, role: &str) -> Result<GenerateBatchResponse> {
        #[derive(serde::Serialize)]
        struct Req { count: u32, role: String }
        self.post("/api/npcs/generate-batch", &Req { count, role: role.into() }).await
    }
}
