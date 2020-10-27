use std::{env};

use rand::Rng;
use rant::{Rant, RantOptions};
use regex::Regex;
use serenity::model::prelude::*;
use serenity::{Client, async_trait, client::EventHandler, model::{channel::Message, prelude::Ready}, prelude::*};

const ENV_DISCORD_TOKEN: &str = "RANTBOT_TOKEN";

const EMOJI_SUCCESS: char = '✅';
const EMOJI_COMPILE_ERROR: char = '❓';
const EMOJI_RUNTIME_ERROR: char = '❌';

const MAX_OUTPUT_SIZE: usize = 1900;

fn run_rant(src: &str) -> Result<String, (char, String)> {
    let mut rant = Rant::with_options(RantOptions {
        enable_require: false,
        debug_mode: true,
        seed: rand::thread_rng().gen(),
        .. Default::default()
    });

    let mut errors = vec![];
    match rant.compile(src, &mut errors) {
        Ok(pgm) => {
            match rant.run_into_string(&pgm) {
                Ok(mut output) => {
                    while output.len() > MAX_OUTPUT_SIZE {
                        output.pop();
                    }
                    Ok(output)
                },
                Err(error) => {
                    let errmsg = format!("Crashed!\n\n```[{}] {}\n\nstack trace:\n{}```", 
                        error.error_type, 
                        error.description, 
                        error.stack_trace.unwrap_or("(none)".to_owned())
                    );
                    Err((EMOJI_RUNTIME_ERROR, errmsg))
                }
            }
        },
        Err(_) => {
            let mut err_list = String::new();
            for (errno, error) in errors.iter().enumerate() {
                err_list.push_str(
                    &format!("{}. ({}) {}: {}\n", 
                        errno + 1,
                        error.pos().map_or("0,0".to_owned(), |pos| format!("{},{}", pos.line(), pos.col())),
                        error.code(), 
                        error.message()
                    )
                );
            }
            let errmsg = format!("Build failed!\n\n```\n{}\n```", &err_list);
            Err((EMOJI_COMPILE_ERROR, errmsg))
        }
    }
}

struct Handler {
    trigger_regex: Regex
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot {
            return
        }
        if let Some(caps) = self.trigger_regex.captures(&msg.content) {
            if let Some(pgm_src) = caps.get(1) {
                match run_rant(pgm_src.as_str()) {
                    Ok(output) => {
                        msg.react(&ctx.http, EMOJI_SUCCESS).await;
                        msg.reply(&ctx.http, format!("```\n{}\n```", output)).await;
                    },
                    Err((err_react, err_msg)) => {
                        msg.react(&ctx.http, err_react).await;
                        msg.reply(&ctx.http, err_msg).await;
                    }
                }
            }
        }
    }

    async fn ready(&self, ctx: Context, _: Ready) {
        let status = format!("Rant {} ({})", rant::RANT_VERSION, rant::BUILD_VERSION);
        ctx.set_presence(Some(Activity::playing(&status)), prelude::OnlineStatus::Online).await;
    }
}


#[tokio::main]
async fn main() {
    let token = env::var(ENV_DISCORD_TOKEN).expect("no client token found");
    let mut client = Client::builder(token.trim())
            .event_handler(Handler {
                trigger_regex: Regex::new(r#"(?s)```rantbot\s+(.*)\s*```"#).unwrap()
            })
            .await
            .expect("failed to init discord client");

    if let Err(err) = client.start().await {
        eprintln!("client error: {:?}", err);
    }
}
