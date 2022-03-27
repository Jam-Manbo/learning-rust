use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content.to_lowercase() == "!ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error while sending message: {:?}", why);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("Logged in with {}!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let token = String::from("my discord bot token is private!!");
    // this is for environment var
    // let token = std::env::var("DISCORD_TOKEN").expect("Expect a discord bot token to be here");
    let mut client = Client::builder(&token).event_handler(Handler).await.expect("Err while creating a client");

    if let Err(why) = client.start().await {
        println!("Client Error: {:?}", why);
    }
}