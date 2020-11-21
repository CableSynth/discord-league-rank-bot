use serenity::prelude::*;
use serenity::http::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    Args, CommandResult,
    macros::command,
};

#[command]
pub async fn help(ctx: &Context, msg: &Message, args: Args) -> CommandResult {

    
    if args.is_empty() {
        
        msg.channel_id.say(&ctx.http, format!("yes this is help, {}. type !help commands", msg.author)).await?;
    }
    // match args {
    // };
    msg.channel_id.say(&ctx.http, "yes this is help").await?;

    Ok(())
}