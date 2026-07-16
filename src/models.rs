use serde::{Deserialize, Serialize};

// ── NPC Core ────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NPC {
    pub id: String,
    pub name: String,
    pub personality: NPCPersonality,
    pub role: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub faction: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inventory: Option<Vec<InventoryItem>>,
    pub mood: NPCMood,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "gameId")]
    pub game_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persona: Option<GeneratedPersona>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<NPCSchedule>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub psychology: Option<NPCPsychology>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NPCPersonality {
    pub traits: Vec<String>,
    #[serde(rename = "speechStyle")]
    pub speech_style: String,
    pub backstory: String,
    pub values: Vec<String>,
    #[serde(default)]
    pub quirks: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NPCMood {
    pub emotion: String,
    pub intensity: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryItem {
    pub item: String,
    pub price: i32,
    pub quantity: i32,
}

// ── Game Events ─────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameEvent {
    #[serde(rename = "type")]
    pub event_type: String,
    #[serde(rename = "playerId", skip_serializing_if = "Option::is_none")]
    pub player_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    pub context: GameContext,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameContext {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(rename = "timeOfDay", skip_serializing_if = "Option::is_none")]
    pub time_of_day: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weather: Option<String>,
    #[serde(rename = "nearbyNPCs", default)]
    pub nearby_npcs: Vec<String>,
    #[serde(rename = "playerReputation", default)]
    pub player_reputation: i32,
    #[serde(rename = "playerLevel", default)]
    pub player_level: i32,
    #[serde(rename = "activeQuests", default)]
    pub active_quests: Vec<String>,
    #[serde(rename = "recentEvents", default)]
    pub recent_events: Vec<String>,
}

impl Default for GameContext {
    fn default() -> Self {
        Self {
            location: None,
            time_of_day: None,
            weather: None,
            nearby_npcs: vec![],
            player_reputation: 50,
            player_level: 1,
            active_quests: vec![],
            recent_events: vec![],
        }
    }
}

// ── Responses ───────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventResult {
    pub response: NPCResponse,
    #[serde(rename = "tokenUsage")]
    pub token_usage: TokenUsage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NPCResponse {
    #[serde(rename = "eventId")]
    pub event_id: String,
    #[serde(rename = "npcId")]
    pub npc_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dialogue: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emotion: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<NPCAction>,
    #[serde(rename = "tradeOffer", skip_serializing_if = "Option::is_none")]
    pub trade_offer: Option<TradeOffer>,
    #[serde(rename = "questUpdate", skip_serializing_if = "Option::is_none")]
    pub quest_update: Option<QuestUpdate>,
    #[serde(rename = "combatDecision", skip_serializing_if = "Option::is_none")]
    pub combat_decision: Option<CombatDecision>,
    #[serde(rename = "memoryUpdated", default)]
    pub memory_updated: bool,
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NPCAction {
    #[serde(rename = "type")]
    pub action_type: String,
    pub target: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeOffer {
    pub item: String,
    pub price: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestUpdate {
    #[serde(rename = "questId")]
    pub quest_id: String,
    pub status: String,
    #[serde(rename = "nextObjective", skip_serializing_if = "Option::is_none")]
    pub next_objective: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reward: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombatDecision {
    pub action: String,
    pub target: String,
    pub reasoning: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenUsage {
    pub input: i32,
    pub output: i32,
    pub calls: i32,
}

// ── Generation ──────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedPersona {
    pub age: i32,
    pub gender: String,
    pub openness: f32,
    pub conscientiousness: f32,
    pub extraversion: f32,
    pub agreeableness: f32,
    pub neuroticism: f32,
    #[serde(default)]
    pub fears: Vec<String>,
    #[serde(default)]
    pub ambitions: Vec<String>,
    #[serde(default)]
    pub vices: Vec<String>,
    #[serde(default)]
    pub virtues: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub income: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NPCSchedule {
    #[serde(rename = "npcId")]
    pub npc_id: String,
    pub slots: Vec<ScheduleSlot>,
    #[serde(default)]
    pub overrides: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduleSlot {
    pub activity: String,
    pub location: String,
    #[serde(rename = "startHour")]
    pub start_hour: i32,
    #[serde(rename = "endHour")]
    pub end_hour: i32,
    #[serde(default)]
    pub days: Vec<i32>,
    #[serde(default)]
    pub priority: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NPCPsychology {
    pub stress: f32,
    #[serde(default)]
    pub trauma: Vec<TraumaEntry>,
    #[serde(default)]
    pub goals: Vec<NPCGoal>,
    #[serde(default)]
    pub emotions: Vec<EmotionLayer>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraumaEntry {
    #[serde(rename = "traumaEvent", alias = "event")]
    pub event: String,
    pub severity: f32,
    pub triggers: Vec<String>,
    #[serde(rename = "healingRate")]
    pub healing_rate: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NPCGoal {
    pub id: String,
    pub description: String,
    pub priority: f32,
    pub progress: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionLayer {
    pub emotion: String,
    pub intensity: f32,
    pub cause: String,
}

// ── Requests ────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize)]
pub struct CreateNPCRequest {
    pub name: String,
    pub role: String,
    pub personality: NPCPersonality,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub faction: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inventory: Option<Vec<InventoryItem>>,
}

/// Partial NPC update — only fields set to `Some` are sent.
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateNPCRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personality: Option<NPCPersonality>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub faction: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inventory: Option<Vec<InventoryItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mood: Option<NPCMood>,
}

#[derive(Debug, Clone, Serialize)]
pub struct GenerateNPCRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub service: String,
    pub uptime: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchEventResult {
    pub results: Vec<EventResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateBatchResponse {
    pub npcs: Vec<NPC>,
    #[serde(rename = "socialLinks", default)]
    pub social_links: Vec<SocialLink>,
}

/// Response from `POST /api/npcs/:id/speak` (NPC voice synthesis).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeakResponse {
    /// Base64-encoded audio data.
    pub audio: String,
    /// Audio format (e.g. "wav").
    pub format: String,
    /// Voice used for synthesis.
    pub voice: String,
    /// TTS provider that produced the audio.
    pub provider: String,
    #[serde(rename = "npcId")]
    pub npc_id: String,
    #[serde(rename = "npcName")]
    pub npc_name: String,
    /// Number of characters synthesized (metered against the voice quota).
    #[serde(rename = "textLength")]
    pub text_length: i64,
}

/// Response from `GET /api/stats`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatsResponse {
    pub experts: Vec<ExpertInfo>,
    #[serde(rename = "totalExperts")]
    pub total_experts: i32,
    #[serde(rename = "cognitiveFeatures", default)]
    pub cognitive_features: Vec<String>,
    pub uptime: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpertInfo {
    pub id: String,
    pub name: String,
    pub domain: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialLink {
    #[serde(rename = "fromId")]
    pub from_id: String,
    #[serde(rename = "toId")]
    pub to_id: String,
    #[serde(rename = "type")]
    pub link_type: String,
    pub strength: f32,
}
