use colink::*;

pub struct Init;
#[colink::async_trait]
impl ProtocolEntry for Init {
    async fn start(
        &self,
        cl: CoLink,
        _param: Vec<u8>,
        _participants: Vec<Participant>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        let _ = cl.read_or_wait("tg_bot:bot_token").await?;
        let _ = cl.read_or_wait("tg_bot:chat_id").await?;
        let participants = vec![Participant {
            user_id: cl.get_user_id()?,
            role: "default".to_string(),
        }];
        cl.run_task("telegram_bot", Default::default(), &participants, false)
            .await?;
        cl.run_task(
            "telegram_bot.send_msg",
            "Welcome, you've successfully connected your Telegram bot!".as_bytes(),
            &participants,
            false,
        )
        .await?;
        Ok(())
    }
}
