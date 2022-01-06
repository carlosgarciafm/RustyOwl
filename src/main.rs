use std::env;

use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::model::channel::Message;
use serenity::framework::standard::{
    StandardFramework,
    CommandResult,
    macros::{
        command,
        group
    }
};

#[group]
#[commands(hello)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!"))
        .group(&GENERAL_GROUP);

    // Login using token.
    let token = env::var("RUSTYOWL_TOKEN").expect("token");
    let mut client = Client::builder(token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // Start listening for events.
    if let Err(why) = client.start().await {
        println!("Error running client: {:?}", why);
    }
}

#[command]
async fn hello(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Hello World!").await?;

    Ok(())
}
