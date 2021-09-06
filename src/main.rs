use dotenv::dotenv;
use lazy_static::lazy_static;
use regex::Regex;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use std::env;

struct Handler;
// const REE: Regex = Regex::new("(パソコン|PC)(の電源(を)*)*(((点|つ)け)|((入|い)れ))て").unwrap();

struct Commands {
    pc_on: Regex,
    light_on: Regex,
}

lazy_static! {
    static ref COMMANDS: Commands = {
        let pc_on = Regex::new(r"(パソコン|PC)(の電源)?を?(((点|つ)け)|((入|い)れ))て").unwrap();
        let light_on =
            Regex::new(r"(照明|ライト|明かり|燈火|灯|あかり)を(((点|つ)け)|((灯|とも)し))て")
                .unwrap();
        Commands { pc_on, light_on }
    };
}

async fn turn_on_pc(ctx: &Context, msg: &Message) {
    if let Err(why) = msg
        .channel_id
        .say(
            &ctx.http,
            "任せて……ください プロデューサーさん……！
    ",
        )
        .await
    {
        println!("Error sending message: {:?}", why);
    }
}

async fn turn_on_light(ctx: &Context, msg: &Message) {
    if let Err(why) = msg
        .channel_id
        .say(
            &ctx.http,
            "わ、わたしでよければ……！
    ",
        )
        .await
    {
        println!("Error sending message: {:?}", why);
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if COMMANDS.pc_on.is_match(&msg.content) {
            turn_on_pc(&ctx, &msg).await;
        } else if COMMANDS.light_on.is_match(&msg.content) {
            turn_on_light(&ctx, &msg).await;
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
