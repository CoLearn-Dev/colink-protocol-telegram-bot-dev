use colink::*;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let args = env::args().skip(1).collect::<Vec<_>>();
    let addr = &args[0];
    let jwt = &args[1];

    let cl = CoLink::new(addr, jwt);
    let participants = vec![Participant {
        user_id: cl.get_user_id()?,
        role: "default".to_string(),
    }];
    cl.run_task(
        "telegram_bot.send_waiting_task",
        Default::default(),
        &participants,
        false,
    )
    .await?;

    Ok(())
}
