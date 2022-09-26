use colink_sdk::*;
use std::collections::HashMap;

const TG_API: &str = "https://api.telegram.org/bot";

pub struct SendWaitingTask;
#[colink_sdk::async_trait]
impl ProtocolEntry for SendWaitingTask {
    async fn start(
        &self,
        cl: CoLink,
        param: Vec<u8>,
        _participants: Vec<Participant>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        let task_id = String::from_utf8_lossy(&param);
        let bot_token = cl.read_entry("tg_bot:bot_token").await?;
        let bot_token = String::from_utf8_lossy(&bot_token);
        let chat_id = cl.read_entry("tg_bot:chat_id").await?;
        let chat_id = String::from_utf8_lossy(&chat_id);
        let msg_text = format!("Waiting task: {}", task_id);
        let mut inline_keyboard_entries: Vec<Vec<HashMap<&str, String>>> = vec![];
        let mut inline_keyboard_entry: Vec<HashMap<&str, String>> = vec![];
        for action in ["approve", "reject", "ignore"] {
            let mut map: HashMap<&str, String> = HashMap::new();
            map.insert("text", action.to_string());
            map.insert("callback_data", format!("2 {} {}", task_id, action));
            inline_keyboard_entry.push(map);
        }
        inline_keyboard_entries.push(inline_keyboard_entry);
        let reply_markup = &format!(
            "{{\"inline_keyboard\":{}}}",
            serde_json::to_string(&inline_keyboard_entries)?
        );
        println!("{}", reply_markup);
        let mut payload: HashMap<&str, &str> = HashMap::new();
        payload.insert("chat_id", &chat_id);
        payload.insert("text", &msg_text);
        payload.insert("reply_markup", reply_markup);
        let http_client = reqwest::Client::new();
        let resp = http_client
            .post(TG_API.to_string() + &bot_token + "/sendMessage")
            .json(&payload)
            .send()
            .await?;
        println!("{:?}", resp.status());
        cl.create_entry(
            &format!("tasks:{}:output", cl.get_task_id()?),
            task_id.as_bytes(),
        )
        .await?;
        Ok(())
    }
}
