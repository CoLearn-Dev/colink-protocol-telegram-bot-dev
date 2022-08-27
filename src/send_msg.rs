use colink_sdk::*;
use std::collections::HashMap;

const TG_API: &str = "https://api.telegram.org/bot";

pub struct SendMsg;
#[colink_sdk::async_trait]
impl ProtocolEntry for SendMsg {
    async fn start(
        &self,
        cl: CoLink,
        param: Vec<u8>,
        _participants: Vec<Participant>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        let msg = String::from_utf8_lossy(&param);
        let bot_token = cl.read_entry("tg_bot:bot_token").await?;
        let bot_token = String::from_utf8_lossy(&bot_token);
        let chat_id = cl.read_entry("tg_bot:chat_id").await?;
        let chat_id = String::from_utf8_lossy(&chat_id);
        let mut payload = HashMap::new();
        payload.insert("chat_id", &chat_id);
        payload.insert("text", &msg);
        let http_client = reqwest::Client::new();
        http_client
            .post(TG_API.to_string() + &bot_token + "/sendMessage")
            .json(&payload)
            .send()
            .await?;
        Ok(())
    }
}
