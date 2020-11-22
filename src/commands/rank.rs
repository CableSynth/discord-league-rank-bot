use std::env;
use serenity::prelude::*;
use serenity::http::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    Args, CommandResult, HelpOptions, help_commands, CommandGroup,
    macros::{group, command, help},
};
use serenity::utils::MessageBuilder;
use riven::RiotApi;
use riven::consts::{
    Region, QueueType
};

#[command]
pub async fn rank(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    // let mut rt = tokio::runtime::Runtime::new().unwrap();
    // rt.block_on(async {
        
    // });
    let summoner_name = match args.single::<String>(){
        Ok(summoner_name) => summoner_name,
        Err(_sommoner_name) => {
            msg.channel_id.say(&ctx.http, format!("No name passed to rank command")).await;
            return Ok(())
        },
    };
    dotenv::dotenv().expect("Failed to load .env file");
    let riot_key = env::var("RIOT_API")
        .expect("Expected a token in the environment");
    let riot_api = RiotApi::with_key(riot_key);
    // Get summoner data
    let summoner = riot_api.summoner_v4()
        .get_by_summoner_name(Region::NA, &summoner_name).await
        .expect("Get summoner Failed")
        .expect("There is no summoner with that name.");

    let summoner_league_info = riot_api.league_v4()
        .get_league_entries_for_summoner(Region::NA, &summoner.id).await
        .expect("Get rank failed");
    
    // make this better
    let mut soloq = &summoner_league_info[0];
    for (_i, queue) in summoner_league_info.iter().enumerate() {
        if queue.queue_type == QueueType::RANKED_SOLO_5x5 {
            soloq = queue;
            break;
        } else {
            soloq = queue
        }
    }
    
    let response = MessageBuilder::new()
        .push("The rank of, ")
        .push_bold(&summoner_name)
        .push("is")
        .push_italic(soloq.tier)
        .push_italic(soloq.rank)
        .build();

    if let Err(why) = msg.channel_id.say(&ctx.http, &response).await {
        println!("Error sending message: {:?}", why);
    }
    Ok(())
}