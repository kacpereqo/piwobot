extern crate dotenv;
mod piwka;
use dotenv::dotenv;
use piwka::LISTA_PIWEK;
use poise::serenity_prelude as serenity;
use poise::Event;
use rand::seq::SliceRandom;
use serenity::model::prelude::*;
use std::collections::HashMap;
use std::env;

async fn event_handler(
    ctx: &serenity::Context,
    event: &Event<'_>,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    _data: &Data,
) -> Result<(), Error> {
    match event {
        Event::Ready { data_about_bot } => {
            println!("Logged in as {}", data_about_bot.user.name);
        }
        Event::InteractionCreate { interaction } => {
            if interaction.kind() == InteractionType::MessageComponent {
                let _interaction = interaction.clone().message_component().unwrap();
                let message_id = _interaction.message.id;
                let choice = _interaction.data.custom_id.clone();
                let mut member = _interaction.clone().member.unwrap();

                if message_id == MessageId(1158079176096628856) {
                    let mut roles = HashMap::new();

                    roles.insert("Grupa 1", RoleId(1157242625770934282));
                    roles.insert("Grupa 2", RoleId(1157272733445529640));
                    roles.insert("Grupa 3", RoleId(1157272763317362749));

                    for role in roles.values() {
                        member.remove_role(&ctx, role).await.unwrap();
                    }

                    let role = roles.get(&choice as &str).unwrap();
                    member.add_role(&ctx, role).await.unwrap();

                    _interaction
                        .create_interaction_response(&ctx, |r| {
                            r.kind(InteractionResponseType::ChannelMessageWithSource)
                                .interaction_response_data(|d| {
                                    d.ephemeral(true).content(format!(
                                        "Grupa dziekańska została ustawiona na **{}**",
                                        choice
                                    ))
                                })
                        })
                        .await
                        .unwrap();
                } else if message_id == MessageId(1158079666087796816) {
                    let mut roles = HashMap::new();

                    roles.insert("Grupa 1", RoleId(1157272795902918657));
                    roles.insert("Grupa 2", RoleId(1157272860293877814));
                    roles.insert("Grupa 3", RoleId(1157272882704035850));
                    roles.insert("Grupa 4", RoleId(1157272923523006494));
                    roles.insert("Grupa 5", RoleId(1157272960810373191));
                    roles.insert("Grupa 6", RoleId(1157272990069817436));

                    for role in roles.values() {
                        member.remove_role(&ctx, role).await.unwrap();
                    }

                    let role = roles.get(&choice as &str).unwrap();
                    member.add_role(&ctx, role).await.unwrap();

                    _interaction
                        .create_interaction_response(&ctx, |r| {
                            r.kind(InteractionResponseType::ChannelMessageWithSource)
                                .interaction_response_data(|d| {
                                    d.ephemeral(true).content(format!(
                                        "Grupa labolatoryjna została ustawiona na **{}**",
                                        choice
                                    ))
                                })
                        })
                        .await
                        .unwrap();
                } else {
                    println!("{:?}", 2);
                }
            }
        }
        _ => {}
    }
    Ok(())
}
/// Daje ci randomowe piwko do wypicia
#[poise::command(slash_command, prefix_command)]
async fn piwko(ctx: Context<'_>) -> Result<(), Error> {
    let randomowe_piwo = LISTA_PIWEK.choose(&mut rand::thread_rng()).unwrap();
    ctx.say(*randomowe_piwo).await?;
    Ok(())
}

struct Data {}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;
#[tokio::main]
async fn main() {
    dotenv().ok();

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_MEMBERS;

    let token = env::var("TOKEN").unwrap();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            event_handler: |_ctx, event, _framework, _data| {
                Box::pin(event_handler(_ctx, event, _framework, _data))
            },
            commands: vec![piwko()],
            ..Default::default()
        })
        .token(token)
        .intents(intents)
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        });

    framework.run().await.unwrap();
}
