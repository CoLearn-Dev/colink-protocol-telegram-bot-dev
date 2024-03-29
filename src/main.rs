use colink_protocol_telegram_bot::edit_msg::*;
use colink_protocol_telegram_bot::init::*;
use colink_protocol_telegram_bot::send_msg::*;
use colink_protocol_telegram_bot::send_msg_with_parse_mode::*;
use colink_protocol_telegram_bot::send_msg_with_reply_markup::*;
use colink_protocol_telegram_bot::send_multi_select_question::*;
use colink_protocol_telegram_bot::send_waiting_task::*;
use colink_protocol_telegram_bot::telegram_bot::*;

colink::protocol_start!(
    ("telegram_bot:@init", Init),
    ("telegram_bot:default", TelegramBot),
    ("telegram_bot.send_msg:default", SendMsg),
    (
        "telegram_bot.send_multi_select_question:default",
        SendMultiSelectQuestion
    ),
    (
        "telegram_bot.send_msg_with_reply_markup:default",
        SendMsgWithReplyMarkup
    ),
    (
        "telegram_bot.send_msg_with_parse_mode:default",
        SendMsgWithParseMode
    ),
    ("telegram_bot.send_waiting_task:default", SendWaitingTask),
    ("telegram_bot.edit_msg:default", EditMsg)
);
