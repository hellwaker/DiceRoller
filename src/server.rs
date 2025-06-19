use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

use crate::roller;

pub struct Handler;

#[async_trait]
impl serenity::prelude::EventHandler for Handler {
    //Handles getting a message
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println! ("Error sending message: {why:?}");
            }
        }
        if msg.content == "!roll" {
            let result = roller::roll_dice(5,20);

            if let Err(why) = msg.channel_id.say(&ctx.http, result[0].to_string()).await {
                println! ("Error sending message: {why:?}");
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected", ready.user.name);
    }
}