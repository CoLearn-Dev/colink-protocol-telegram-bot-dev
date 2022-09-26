use colink::*;
use std::collections::HashMap;

const TG_API: &str = "https://api.telegram.org/bot";

pub struct EditMsg;
#[colink::async_trait]
impl ProtocolEntry for EditMsg {
    async fn start(
        &self,
        cl: CoLink,
        param: Vec<u8>,
        _participants: Vec<Participant>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        let params: HashMap<String, String> = serde_json::from_slice(&param)?;
        let msg = params.get("text").unwrap();
        let message_id = params.get("message_id").unwrap();
        let bot_token = cl.read_entry("tg_bot:bot_token").await?;
        let bot_token = String::from_utf8_lossy(&bot_token);
        let chat_id = cl.read_entry("tg_bot:chat_id").await?;
        let chat_id = String::from_utf8_lossy(&chat_id);
        let mut payload: HashMap<&str, &str> = HashMap::new();
        payload.insert("chat_id", &chat_id);
        payload.insert("text", msg);
        payload.insert("message_id", message_id);
        let http_client = reqwest::Client::new();
        let resp = http_client
            .post(TG_API.to_string() + &bot_token + "/editMessageText")
            .json(&payload)
            .send()
            .await?;
        println!("{:?}", resp.status());
        Ok(())
    }
}
