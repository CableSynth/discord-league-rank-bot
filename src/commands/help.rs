use std::collections::HashSet;
use serenity::prelude::*;
use serenity::http::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    Args, CommandResult, HelpOptions, help_commands, CommandGroup,
    macros::{group, command, help},
};

const COMMANDS: [&str; 6] = [
    "commands",
    "rank",
    "last5",
    "clash",
    "masteries",
    "kappa",
];

// #[command]
// pub async fn help(ctx: &Context, msg: &Message, args: Args) -> CommandResult {

//     let reply_message = match args.len() {
//         0 => format!("yes this is help, {}. type !help commands for list of commands", msg.author),
//         1 => args.message().to_string(),
//         _ => format!("yes this is help, {}. only one argument at a time plz. type !help commands for list of commands", msg.author),
//     };

//     msg.channel_id.say(&ctx.http, reply_message).await?;
//     Ok(())
// }

// The framework provides two built-in help commands for you to use.
// But you can also make your own customized help command that forwards
// to the behaviour of either of them.
#[help]
// This replaces the information that a user can pass
// a command-name as argument to gain specific information about it.
#[individual_command_tip =
"Hello! こんにちは！Hola! Bonjour! 您好!\n\
If you want more information about a specific command, just pass the command as argument."]
// Some arguments require a `{}` in order to replace it with contextual information.
// In this case our `{}` refers to a command's name.
#[command_not_found_text = "Could not find: `{}`."]
// Define the maximum Levenshtein-distance between a searched command-name
// and commands. If the distance is lower than or equal the set distance,
// it will be displayed as a suggestion.
// Setting the distance to 0 will disable suggestions.
#[max_levenshtein_distance(3)]

async fn my_help(
    context: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>
) -> CommandResult {
    let _ = help_commands::with_embeds(context, msg, args, help_options, groups, owners).await;
    Ok(())
}