pub use telegram_bot_raw as types;

use serde::{Deserialize, Serialize};

pub mod ops {
    pub const MESSAGE: &'static str = "WASMTG_BOT_MESSAGE";
    pub const UPDATE: &'static str = "WASMTG_BOT_UPDATE";
    pub const REQUEST: &'static str = "WASMTG_BOT_REQUEST";
}

pub const WASMBOT_CAPS: &'static str = "tgbot::core";

#[derive(Clone, Debug, Serialize, Deserialize, Copy)]
pub struct InitStruct;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum BotError {
    InternalError(String),
    MethodNotImplemented,
}

pub type BotResult = Result<(), BotError>;
