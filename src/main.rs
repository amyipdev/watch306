use std::{
    env,
    sync::atomic::{AtomicBool, Ordering},
    thread
};

use serenity::{
    all::{GatewayIntents, Channel, Ready},
    async_trait,
    client::{Context, EventHandler},
    model::channel::Message,
    Client,
};

struct Handler;

static BOUND: AtomicBool = AtomicBool::new(false);

// Process: upon receiving a message, check if bound. If so, drop
// Otherwise, bind to that channel - make the thread that checks once
// per minute if 306 is up. Only send messages on state change. Pass
// the message/context info into the thread so we don't have to deal with
// weird Discord BS
#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if BOUND.load(Ordering::Relaxed) {
            return;
        }
        if msg.content.starts_with("306!bind ") {
            //if let Err(w) = msg.channel_id.say(&ctx.http, "Pong!").await {
            //    println!("Error sending message: {w:?}");
            //}
            if let Ok(c) = msg.channel_id.to_channel(&ctx.http).await {
                if let Channel::Guild(d) = c {
                    if d.name == "acm-server" {
                        if let Some(z) = msg.guild_id {
                            if z.get()
                                == u64::from_str_radix(&env::var("WATCH306_GUILD_ID").unwrap(), 10).unwrap()
                            {
                                BOUND.store(true, Ordering::Relaxed);
                                thread::spawn(|| futures::executor::block_on(thread_handle(ctx, msg)));
                            }
                        }
                    }
                }
            }
        }
    }
    async fn ready(&self, _: Context, ready: Ready) {
        println!("Connected as {}", ready.user.name);
    }
}

static IS306UP: AtomicBool = AtomicBool::new(true);

async fn thread_handle(ctx: Context, msg: Message) -> Result<(), serenity::prelude::SerenityError> {
    println!("Bound!");
    let remainder = msg.content.splitn(2, "306!bind ").nth(1).unwrap();
    let oks = format!("CS306 is back up, {}", remainder);
    let downed = format!("CS306 has gone down, {}", remainder);
    loop {
        // this isn't called in a tokio context, so we have to block
        let resp = reqwest::blocking::get("https://cs306.acmcsuf.com").expect("some error").status().is_success();
        if resp != IS306UP.load(Ordering::Relaxed) {
            IS306UP.store(resp, Ordering::Relaxed);
            if resp {
                msg.channel_id.say(&ctx.http, &oks).await?;
            } else {
                msg.channel_id.say(&ctx.http, &downed).await?;
            }
        }
        std::thread::sleep(std::time::Duration::from_millis(10000));
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = env::var("WATCH306_TOKEN")?;
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await?;
    if let Err(w) = client.start().await {
        eprintln!("Client error: {w:?}");
    }
    Ok(())
}
