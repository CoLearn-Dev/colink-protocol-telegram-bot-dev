use colink_protocol_telegram_bot::edit_msg::*;
use colink_protocol_telegram_bot::send_msg::*;
use colink_protocol_telegram_bot::send_msg_with_reply_markup::*;
use colink_protocol_telegram_bot::telegram_bot::*;

colink_sdk::protocol_start!(
    ("telegram_bot:default", TelegramBot),
    ("telegram_bot.send_msg:default", SendMsg),
    (
        "telegram_bot.send_msg_with_reply_markup:default",
        SendMsgWithReplyMarkup
    ),
    ("telegram_bot.edit_msg:default", EditMsg)
);
