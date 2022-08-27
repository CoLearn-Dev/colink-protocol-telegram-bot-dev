use colink_protocol_telegram_bot::get_msg::*;
use colink_protocol_telegram_bot::send_msg::*;

colink_sdk::protocol_start!(
    ("telegram_bot.send_msg:default", SendMsg),
    ("telegram_bot.get_msg:default", GetMsg)
);
