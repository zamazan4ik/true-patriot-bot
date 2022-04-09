use teloxide::{prelude2::*, utils::command::BotCommand};

#[derive(Clone, teloxide::utils::command::BotCommand)]
#[command(rename = "lowercase", description = "These commands are supported:")]
pub(crate) enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "show generic information about the bot.")]
    About,
    #[command(description = "patriotify the message.")]
    Patriotify(String),
}

#[allow(unused_assignments)]
pub(crate) async fn command_handler(
    msg: Message,
    bot: AutoSend<Bot>,
    command: Command,
) -> Result<(), teloxide::RequestError> {
    static HELP_TEXT: &str = "Команды:
        (инлайн-режим) - пиши и я сделаю тебя Истинным Патриотом
        /help - показать это сообщение";
    static ABOUT_TEXT: &str = "Репозиторий бота: https://github.com/zamazan4ik/true-patriot-bot .\
        Там вы можете получить более подробную справку, оставить отчёт о проблеме или внести \
        какое-либо предложение.";

    match command {
        Command::Help => {
            bot.send_message(msg.chat.id, HELP_TEXT)
                .reply_to_message_id(msg.id)
                .await?;
        }
        Command::About => {
            bot.send_message(msg.chat.id, ABOUT_TEXT)
                .reply_to_message_id(msg.id)
                .await?;
        }
        Command::Patriotify(message) => {
            let processed_message =
                crate::message_transformer::make_message_patriotic(message.to_string());
            bot.send_message(msg.chat.id, processed_message)
                .reply_to_message_id(msg.id)
                .await?;
        }
    };

    Ok(())
}
