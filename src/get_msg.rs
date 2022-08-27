use colink_sdk::*;
use std::collections::HashMap;

const TG_API: &str = "https://api.telegram.org/bot";

pub struct GetMsg;
#[colink_sdk::async_trait]
impl ProtocolEntry for GetMsg {
    async fn start(
        &self,
        cl: CoLink,
        _param: Vec<u8>,
        _participants: Vec<Participant>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        let bot_token = cl.read_entry("tg_bot:bot_token").await?;
        let bot_token = String::from_utf8_lossy(&bot_token);
        let chat_id = cl.read_entry("tg_bot:chat_id").await?;
        let chat_id = String::from_utf8_lossy(&chat_id);
        loop {
            let mut payload = HashMap::new();
            if let Ok(offset) = cl.read_entry("tg_bot:msg_offset").await {
                let offset = i32::from_le_bytes(<[u8; 4]>::try_from(offset).unwrap());
                payload.insert("offset", offset);
            }
            payload.insert("limit", 1);
            payload.insert("timeout", 300);
            let http_client = reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(600))
                .build()?;
            let resp = http_client
                .post(TG_API.to_string() + &bot_token + "/getUpdates")
                .json(&payload)
                .send()
                .await?;
            let resp = resp.json::<serde_json::Value>().await?;
            let res = resp["result"].as_array().unwrap();
            if !res.is_empty() {
                let res = &res[0];
                if res["message"]["chat"]["id"].as_i64().unwrap().to_string() == chat_id {
                    cl.create_entry(
                        &format!("tasks:{}:output", cl.get_task_id()?),
                        res["message"]["text"].as_str().unwrap().as_bytes(),
                    )
                    .await?;
                    println!("{}", res["message"]["text"].as_str().unwrap());
                    cl.update_entry(
                        "tg_bot:msg_offset",
                        &(res["update_id"].as_i64().unwrap() as i32 + 1).to_le_bytes(),
                    )
                    .await?;
                    return Ok(());
                }
            }
        }
    }
}
