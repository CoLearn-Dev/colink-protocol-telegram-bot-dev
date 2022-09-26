use colink::*;
use std::collections::HashMap;

const TG_API: &str = "https://api.telegram.org/bot";

pub struct SendWaitingTask;
#[colink::async_trait]
impl ProtocolEntry for SendWaitingTask {
    async fn start(
        &self,
        cl: CoLink,
        _param: Vec<u8>,
        _participants: Vec<Participant>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        let bot_token = cl.read_entry("tg_bot:bot_token").await?;
        let bot_token = String::from_utf8_lossy(&bot_token).to_string();
        let chat_id = cl.read_entry("tg_bot:chat_id").await?;
        let chat_id = String::from_utf8_lossy(&chat_id).to_string();
        let task_queue_name = cl
            .subscribe("_internal:tasks:status:waiting:latest", None)
            .await?;
        let mut subscriber = cl.new_subscriber(&task_queue_name).await?;
        loop {
            let data = subscriber.get_next().await?;
            let message: SubscriptionMessage = prost::Message::decode(&*data)?;
            if message.change_type != "delete" {
                let task_id: Task = prost::Message::decode(&*message.payload).unwrap();
                let task_id = task_id.task_id;
                println!("{}", task_id);
                let cl = cl.clone();
                let bot_token = bot_token.clone();
                let chat_id = chat_id.clone();
                tokio::spawn(async move {
                    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                    send_task(cl, &task_id, &bot_token, &chat_id).await?;
                    Ok::<(), Box<dyn std::error::Error + Send + Sync + 'static>>(())
                });
            }
        }
    }
}

async fn send_task(
    cl: CoLink,
    task_id: &str,
    bot_token: &str,
    chat_id: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let res = cl
        .read_entry(&format!("_internal:tasks:{}", task_id))
        .await?;
    let task: Task = prost::Message::decode(&*res).unwrap();
    if task.status == "waiting" {
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
        payload.insert("chat_id", chat_id);
        payload.insert("text", &msg_text);
        payload.insert("reply_markup", reply_markup);
        let http_client = reqwest::Client::new();
        let resp = http_client
            .post(TG_API.to_string() + bot_token + "/sendMessage")
            .json(&payload)
            .send()
            .await?;
        println!("{:?}", resp.status());
        cl.create_entry(
            &format!("tasks:{}:output", cl.get_task_id()?),
            task_id.as_bytes(),
        )
        .await?;
    }
    Ok(())
}
