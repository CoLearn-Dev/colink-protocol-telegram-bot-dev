use colink_sdk::*;
use std::collections::HashMap;

const TG_API: &str = "https://api.telegram.org/bot";

pub struct TelegramBot;
#[colink_sdk::async_trait]
impl ProtocolEntry for TelegramBot {
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
                cl.update_entry(
                    "tg_bot:msg_offset",
                    &(res["update_id"].as_i64().unwrap() as i32 + 1).to_le_bytes(),
                )
                .await?;
                if res.get("message") != None
                    && res["message"]["chat"]["id"].as_i64().unwrap().to_string() == chat_id
                {
                    let msg_id = res["message"]["message_id"].as_i64().unwrap().to_string();
                    let error_catch = async {
                        let msg = res["message"]["text"].as_str().unwrap();
                        println!("{}", res["message"]["text"].as_str().unwrap());
                        if msg.starts_with("/create_entry ") {
                            let args: Vec<&str> = msg.splitn(3, ' ').collect();
                            let res = cl.create_entry(args[1], args[2].as_bytes()).await?;
                            send_msg(&bot_token, &chat_id, &res, &msg_id).await?;
                        } else if msg.starts_with("/read_entry ") {
                            let args: Vec<&str> = msg.splitn(2, ' ').collect();
                            let res = cl.read_entry(args[1]).await?;
                            let res = String::from_utf8_lossy(&res);
                            send_msg(&bot_token, &chat_id, &res, &msg_id).await?;
                        } else if msg.starts_with("/update_entry ") {
                            let args: Vec<&str> = msg.splitn(3, ' ').collect();
                            let res = cl.update_entry(args[1], args[2].as_bytes()).await?;
                            send_msg(&bot_token, &chat_id, &res, &msg_id).await?;
                        } else if msg.starts_with("/delete_entry ") {
                            let args: Vec<&str> = msg.splitn(2, ' ').collect();
                            let res = cl.delete_entry(args[1]).await?;
                            send_msg(&bot_token, &chat_id, &res, &msg_id).await?;
                        } else if msg.starts_with("/run_local_task ") {
                            let args: Vec<&str> = msg.splitn(3, ' ').collect();
                            let res = cl
                                .run_task(
                                    args[1],
                                    args[2].as_bytes(),
                                    &[Participant {
                                        user_id: cl.get_user_id()?,
                                        role: "default".to_string(),
                                    }],
                                    false,
                                )
                                .await?;
                            send_msg(&bot_token, &chat_id, &res, &msg_id).await?;
                        }
                        Ok::<(), Box<dyn std::error::Error + Send + Sync + 'static>>(())
                    }
                    .await;
                    if let Err(errmsg) = error_catch {
                        let _ = send_msg(&bot_token, &chat_id, &errmsg.to_string(), &msg_id).await;
                    }
                } else if res.get("callback_query") != None
                    && res["callback_query"]["message"]["chat"]["id"]
                        .as_i64()
                        .unwrap()
                        .to_string()
                        == chat_id
                {
                    let callback_query_id = res["callback_query"]["id"].as_str().unwrap();
                    let error_catch = async {
                        let msg_id = res["callback_query"]["message"]["message_id"]
                            .as_i64()
                            .unwrap()
                            .to_string();
                        let args: Vec<&str> = res["callback_query"]["data"]
                            .as_str()
                            .unwrap()
                            .splitn(2, ' ')
                            .collect();
                        if args[0] == "1" {
                            // callback 1: create callback entry
                            let args: Vec<&str> = args[1].splitn(2, ' ').collect();
                            cl.create_entry(
                                &format!("tg_bot:callback:{}", args[0]),
                                args[1].as_bytes(),
                            )
                            .await?;
                            edit_msg(
                                &bot_token,
                                &chat_id,
                                &msg_id,
                                &format!("Your choice: {}", args[1]),
                            )
                            .await?;
                            answer_callback_query(&bot_token, callback_query_id, args[1]).await?;
                        } else if args[0] == "2" {
                            // callback 2: confirm task
                            let args: Vec<&str> = args[1].splitn(2, ' ').collect();
                            if args[1] == "approve" {
                                cl.confirm_task(args[0], true, false, "").await?;
                            } else if args[1] == "reject" {
                                cl.confirm_task(args[0], false, true, "").await?;
                            } else if args[1] == "ignore" {
                                cl.confirm_task(args[0], false, false, "").await?;
                            }
                            edit_msg(
                                &bot_token,
                                &chat_id,
                                &msg_id,
                                &format!("Confirmed: {}", args[1]),
                            )
                            .await?;
                            answer_callback_query(&bot_token, callback_query_id, args[1]).await?;
                        }
                        Ok::<(), Box<dyn std::error::Error + Send + Sync + 'static>>(())
                    }
                    .await;
                    if let Err(errmsg) = error_catch {
                        let _ = answer_callback_query(
                            &bot_token,
                            callback_query_id,
                            &errmsg.to_string()[..180],
                        )
                        .await;
                    }
                }
            }
        }
    }
}

async fn send_msg(
    bot_token: &str,
    chat_id: &str,
    text: &str,
    reply_to_message_id: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let mut payload = HashMap::new();
    payload.insert("chat_id", chat_id);
    payload.insert("text", text);
    if !reply_to_message_id.is_empty() {
        payload.insert("reply_to_message_id", reply_to_message_id);
    }
    let http_client = reqwest::Client::new();
    let resp = http_client
        .post(TG_API.to_string() + bot_token + "/sendMessage")
        .json(&payload)
        .send()
        .await?;
    println!("{:?}", resp.status());
    Ok(())
}

async fn edit_msg(
    bot_token: &str,
    chat_id: &str,
    message_id: &str,
    text: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let mut payload = HashMap::new();
    payload.insert("chat_id", chat_id);
    payload.insert("message_id", message_id);
    payload.insert("text", text);
    let http_client = reqwest::Client::new();
    let resp = http_client
        .post(TG_API.to_string() + bot_token + "/editMessageText")
        .json(&payload)
        .send()
        .await?;
    println!("{:?}", resp.status());
    Ok(())
}

async fn answer_callback_query(
    bot_token: &str,
    callback_query_id: &str,
    text: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let mut payload = HashMap::new();
    payload.insert("callback_query_id", callback_query_id);
    payload.insert("text", text);
    let http_client = reqwest::Client::new();
    let resp = http_client
        .post(TG_API.to_string() + bot_token + "/answerCallbackQuery")
        .json(&payload)
        .send()
        .await?;
    println!("{:?}", resp.status());
    Ok(())
}
