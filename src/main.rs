use teloxide::{prelude::*, utils::command::BotCommands};

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Available commands:")]
enum Command {
    #[command(description = "Start the bot")]
    Start,
    #[command(description = "Get information about VPN")]
    Info,
    #[command(description = "Get your VPN status")]
    Status,
    #[command(description = "Display this help message")]
    Help,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    
    pretty_env_logger::init();
    log::info!("Starting VPN bot...");

    let bot = Bot::from_env();

    let handler = Update::filter_message()
        .filter_command::<Command>()
        .endpoint(answer);

    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Start => {
            bot.send_message(
                msg.chat.id,
                "👋 Welcome to VPN Bot!\n\nUse /help to see available commands."
            ).await?;
        }
        Command::Info => {
            bot.send_message(
                msg.chat.id,
                "ℹ️ This is a VPN service bot.\n\n\
                We provide secure VPN connections using Outline.\n\
                To get started, contact our administrator."
            ).await?;
        }
        Command::Status => {
            bot.send_message(
                msg.chat.id,
                "🔄 Your VPN status:\n\n\
                Currently this is a placeholder. Real status checking will be implemented soon!"
            ).await?;
        }
        Command::Help => {
            bot.send_message(
                msg.chat.id,
                Command::descriptions().to_string()
            ).await?;
        }
    };

    Ok(())
}
