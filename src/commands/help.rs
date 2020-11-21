use serenity::prelude::*;
use serenity::http::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    Args, CommandResult,
    macros::command,
};

const COMMANDS: [&str; 6] = [
    "commands",
    "rank",
    "last5",
    "clash",
    "masteries",
    "kappa",
];

#[command]
pub async fn help(ctx: &Context, msg: &Message, args: Args) -> CommandResult {

    let reply_message = match args.len() {
        0 => format!("yes this is help, {}. type !help commands list of commands", msg.author),
        1 => args.message().to_string(),
        _ => format!("yes this is help, {}. only one argument at a time plz. type !help commands for list of commands", msg.author),
    };

    msg.channel_id.say(&ctx.http, reply_message).await?;
    Ok(())
}