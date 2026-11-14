use crate::models::*;
use reqwest::Client;

/// Well-known values of [`AINPCError::Api::code`].
///
/// - `payment_required` (HTTP 402) — no active subscription (signup not
///   finished, or subscription cancelled). Finish signing up or resubscribe
///   at the `payment_url`.
/// - `npc_limit` (HTTP 402) — plan NPC limit reached; upgrade to create more.
/// - `feature_not_in_plan` (HTTP 402) — feature not included in current plan.
/// - `interaction_cap` (HTTP 429) — monthly included interactions used up.
pub mod error_codes {
    pub const PAYMENT_REQUIRED: &str = "payment_required";
    pub const NPC_LIMIT: &str = "npc_limit";
    pub const FEATURE_NOT_IN_PLAN: &str = "feature_not_in_plan";
    pub const INTERACTION_CAP: &str = "interaction_cap";
}

#[derive(Debug, thiserror::Error)]
pub enum AINPCError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    /// Structured API error (non-2xx response with a JSON body).
    #[error("API error {status}{code}: {message}", code = .code.as_deref().map(|c| format!(" [{c}]")).unwrap_or_default())]
    Api {
        /// HTTP status code (402, 404, 429, ...).
        status: u16,
        /// Machine-readable error code, e.g. `payment_required`,
        /// `npc_limit`, `feature_not_in_plan`, `interaction_cap`.
        code: Option<String>,
        /// Human-readable error message.
        message: String,
        /// Where to subscribe / upgrade. Set for `payment_required`
        /// (no active subscription) and for plan-limit errors (upgrade URL).
        payment_url: Option<String>,
    },
}

impl AINPCError {
    /// HTTP status of an API error, if this is an [`AINPCError::Api`].
    pub fn status(&self) -> Option<u16> {
        match self {
            AINPCError::Api { status, .. } => Some(*status),
            _ => None,
        }
    }

    /// Machine-readable error code (`payment_required`, `npc_limit`,
    /// `feature_not_in_plan`, `interaction_cap`, ...).
    pub fn code(&self) -> Option<&str> {
        match self {
            AINPCError::Api { code, .. } => code.as_deref(),
            _ => None,
        }
    }

    /// URL where the studio can subscribe or upgrade its plan.
    pub fn payment_url(&self) -> Option<&str> {
        match self {
            AINPCError::Api { payment_url, .. } => payment_url.as_deref(),
            _ => None,
        }
    }

    /// True when the account has no active subscription (signup not
    /// finished, or the subscription was cancelled). Every account is on a
    /// paid plan; direct the user to [`AINPCError::payment_url`] to finish
    /// signing up or resubscribe.
    pub fn is_payment_required(&self) -> bool {
        self.code() == Some(error_codes::PAYMENT_REQUIRED)
    }
}

/// Error body shape returned by the API on non-2xx responses.
#[derive(serde::Deserialize)]
struct ApiErrorBody {
    #[serde(default)]
    error: Option<String>,
    #[serde(default)]
    code: Option<String>,
    #[serde(rename = "paymentUrl", default)]
    payment_url: Option<String>,
    #[serde(rename = "upgradeUrl", default)]
    upgrade_url: Option<String>,
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
///     let client = AINPCClient::new("https://ainpcengine.com", "your-api-key", "my-game");
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

    /// Turn a non-2xx response into a structured [`AINPCError::Api`],
    /// parsing the JSON error body for `code` / `paymentUrl` / `upgradeUrl`.
    async fn api_error(res: reqwest::Response) -> AINPCError {
        let status = res.status().as_u16();
        let text = res.text().await.unwrap_or_default();
        match serde_json::from_str::<ApiErrorBody>(&text) {
            Ok(body) => AINPCError::Api {
                status,
                code: body.code,
                message: body.error.unwrap_or(text),
                payment_url: body.payment_url.or(body.upgrade_url),
            },
            Err(_) => AINPCError::Api {
                status,
                code: None,
                message: text,
                payment_url: None,
            },
        }
    }

    async fn get<T: serde::de::DeserializeOwned>(&self, path: &str) -> Result<T> {
        let res = self.http.get(format!("{}{}", self.base_url, path))
            .header("x-api-key", &self.api_key)
            .header("x-game-id", &self.game_id)
            .send().await?;
        if !res.status().is_success() {
            return Err(Self::api_error(res).await);
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
            return Err(Self::api_error(res).await);
        }
        Ok(res.json().await?)
    }

    async fn patch<B: serde::Serialize, T: serde::de::DeserializeOwned>(&self, path: &str, body: &B) -> Result<T> {
        let res = self.http.patch(format!("{}{}", self.base_url, path))
            .header("x-api-key", &self.api_key)
            .header("x-game-id", &self.game_id)
            .json(body)
            .send().await?;
        if !res.status().is_success() {
            return Err(Self::api_error(res).await);
        }
        Ok(res.json().await?)
    }

    async fn delete(&self, path: &str) -> Result<()> {
        let res = self.http.delete(format!("{}{}", self.base_url, path))
            .header("x-api-key", &self.api_key)
            .header("x-game-id", &self.game_id)
            .send().await?;
        if !res.status().is_success() && res.status().as_u16() != 404 {
            return Err(Self::api_error(res).await);
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

    /// Update an existing NPC (partial update). Only the fields set in
    /// `updates` are changed; the updated NPC is returned.
    pub async fn update_npc(&self, npc_id: &str, updates: &UpdateNPCRequest) -> Result<NPC> {
        self.patch(&format!("/api/npcs/{}", npc_id), updates).await
    }

    pub async fn delete_npc(&self, npc_id: &str) -> Result<()> {
        self.delete(&format!("/api/npcs/{}", npc_id)).await
    }

    // ── Voice (TTS) ─────────────────────────────────────────────────

    /// Synthesize NPC speech for the given text. Returns base64-encoded
    /// audio with the voice matched to the NPC's personality.
    /// Voice is not currently offered; this returns 403 `voice_unavailable`.
    pub async fn speak(&self, npc_id: &str, text: &str) -> Result<SpeakResponse> {
        #[derive(serde::Serialize)]
        struct Body<'a> { text: &'a str }
        self.post(&format!("/api/npcs/{}/speak", npc_id), &Body { text }).await
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

    // ── Stats ───────────────────────────────────────────────────────

    /// Engine stats: registered experts, cognitive features, uptime.
    pub async fn stats(&self) -> Result<StatsResponse> {
        self.get("/api/stats").await
    }
}
