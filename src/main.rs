use std::env;

use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

// Define the event handler
struct Handler;

#[async_trait]
impl EventHandler for Handler {
    // Handle incoming messages
    async fn message(&self, ctx: Context, msg: Message) {
        // Ignore messages from bots
        if msg.author.bot {
            return;
        }

        // Respond to the "!ping" command
        if msg.content.eq_ignore_ascii_case("!ping") {
            println!("Received !ping command from {}", msg.author.name);

            // Send a plain text response
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                eprintln!("Error sending message: {:?}", why);
            }
        }
    }

    // Log when the bot is ready
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected and ready!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    // Retrieve the bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    // Specify the bot's intents
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // Initialize the bot client
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    // Start the bot
    if let Err(why) = client.start().await {
        eprintln!("Client error: {:?}", why);
    }
}
