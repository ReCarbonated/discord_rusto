pub mod emojis;
pub mod generic;
pub mod listeners;
pub mod math;
pub mod stickers;
pub mod timer;


pub use emojis::*;
pub use generic::*;
pub use listeners::*;
pub use math::*;
pub use stickers::*;
pub use timer::*;

// #[help]
// #[individual_command_tip = "Pass a specific command to get details"]
// #[indention_prefix = "+"]
// #[max_levenshtein_distance(3)]
// #[command_not_found_text = "Could not find: `{}`."]
// async fn my_help(
//     context: &Context,
//     msg: &Message,
//     args: Args,
//     help_options: &'static HelpOptions,
//     groups: &[&'static CommandGroup],
//     owners: HashSet<UserId>,
// ) -> CommandResult {
//     let _ = help_commands::with_embeds(context, msg, args, help_options, groups, owners).await;
//     Ok(())
// }