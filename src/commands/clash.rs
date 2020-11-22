use serenity::prelude::*;
use serenity::http::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    Args, CommandResult, HelpOptions, help_commands, CommandGroup,
    macros::{group, command, help},
};
use riven::RiotApi;
use riven::consts::Region;

#[command]
pub async fn clash(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    // let mut rt = tokio::runtime::Runtime::new().unwrap();
    // rt.block_on(async {
        
    // });
    dotenv::dotenv().expect("Failed to load .env file");
    let riot_key = env::var("RIOT_API")
        .expect("Expected a token in the environment");
    
}