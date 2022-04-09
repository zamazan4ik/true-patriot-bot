pub struct Parameters {
    pub bot_name: String,
    pub is_webhook_mode_enabled: bool,
}

impl Parameters {
    pub fn new() -> Self {
        let bot_name = std::env::var("BOT_NAME").expect("BOT_NAME env var is not specified");

        let is_webhook_mode_enabled: bool = std::env::var("WEBHOOK_MODE")
            .unwrap_or_else(|_| "false".to_string())
            .parse()
            .expect(
                "Cannot convert WEBHOOK_MODE to bool. Applicable values are only \"true\" or \"false\"",
            );

        Self {
            bot_name,
            is_webhook_mode_enabled,
        }
    }
}
