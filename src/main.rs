use teloxide::prelude2::*;

mod commands;
mod logging;
mod message_transformer;
mod parameters;
mod webhook;

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    logging::init_logger();

    log::info!("Starting True Patriot Telegram bot");

    let parameters = std::sync::Arc::new(parameters::Parameters::new());

    let bot = Bot::from_env().auto_send();

    let handler = Update::filter_message()
        .branch(
            dptree::entry()
                .filter_command::<commands::Command>()
                .endpoint(commands::command_handler),
        )
        .branch(
            dptree::filter(|msg: Message| msg.chat.is_private() && msg.text().is_some()).endpoint(
                |msg: Message, bot: AutoSend<Bot>| async move {
                    log::debug!("Message in private chat message handler received");
                    let processed_message = message_transformer::make_message_patriotic(
                        msg.text().unwrap().to_string(),
                    );
                    bot.send_message(msg.chat.id, processed_message).await?;
                    respond(())
                },
            ),
        );

    if !parameters.is_webhook_mode_enabled {
        bot.delete_webhook()
            .send()
            .await
            .expect("Cannot delete a webhook");
    }

    let mut bot_dispatcher = Dispatcher::builder(bot.clone(), handler)
        .default_handler(|_| async move {})
        .error_handler(LoggingErrorHandler::with_custom_text(
            "An error has occurred in the dispatcher",
        ))
        .build();

    if parameters.is_webhook_mode_enabled {
        log::info!("Webhook mode activated");
        let rx = webhook::webhook(bot);
        bot_dispatcher
            .setup_ctrlc_handler()
            .dispatch_with_listener(
                rx.await,
                LoggingErrorHandler::with_custom_text("An error from the update listener"),
            )
            .await;
    } else {
        log::info!("Long polling mode activated");
        bot_dispatcher.setup_ctrlc_handler().dispatch().await;
    }
}
