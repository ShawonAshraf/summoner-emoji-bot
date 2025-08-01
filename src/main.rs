mod bot;
mod clipboard;
mod emoji_generator;

use std::env;
use tracing::{info, error};

#[tokio::main]
async fn main() {
    // Initialize tracing subscriber with structured logging
    tracing_subscriber::fmt::init();

    info!("Starting Summoner Emoji Bot application");

    let args = env::args().collect::<Vec<String>>();
    
    // Check if the user provided a command-line argument
    if args.len() > 1 && args[1] == "bot" {
        // If the argument is "bot", run the bot
        info!("Starting Discord bot mode");
        bot::run().await;
        return;
    }
    else {
        // If no argument or a different argument is provided, run the emoji generator
        info!("Starting emoji generator mode");
        let generator = emoji_generator::EmojiGenerator::new();
        let unique_emojis = generator.generate(5);

        // convert the emojis to a single string
        let result = unique_emojis.join(" ");

        // copy to clipboard
        info!(emojis = %result, "Generated emojis, copying to clipboard");

        match clipboard::copy_to_clipboard(&result) {
            Ok(_) => info!("Emojis copied to clipboard successfully"),
            Err(e) => error!(error = %e, "Failed to copy emojis to clipboard"),
        }
    }
}
