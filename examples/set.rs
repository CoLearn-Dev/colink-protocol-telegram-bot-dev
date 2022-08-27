use colink_sdk::CoLink;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let args = env::args().skip(1).collect::<Vec<_>>();
    let addr = &args[0];
    let jwt_a = &args[1];
    let bot_token = &args[2];
    let chat_id = &args[3];

    let cl = CoLink::new(addr, jwt_a);
    cl.update_entry("tg_bot:bot_token", bot_token.as_bytes())
        .await?;
    cl.update_entry("tg_bot:chat_id", chat_id.as_bytes())
        .await?;

    Ok(())
}
