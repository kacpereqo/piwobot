extern crate dotenv;
mod piwka;
mod roles;
use dotenv::dotenv;
use piwka::LISTA_PIWEK;
use poise::serenity_prelude as serenity;
use poise::Event;
use rand::seq::SliceRandom;
use roles::REACTION_ROLES;
use serenity::model::prelude::*;
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
                // find all roles with message_id
                let roles = REACTION_ROLES
                    .iter()
                    .filter(|role| role.message_id == message_id)
                    .collect::<Vec<_>>();

                for role in roles.iter() {
                    member.remove_role(&ctx, role.role_id).await.unwrap();
                }

                let choiced_role = roles
                    .iter()
                    .find(|role| role.interaction_id == choice)
                    .unwrap();

                member.add_role(&ctx, choiced_role.role_id).await.unwrap();

                _interaction
                    .create_interaction_response(&ctx, |r| {
                        r.kind(InteractionResponseType::ChannelMessageWithSource)
                            .interaction_response_data(|d| {
                                d.ephemeral(true)
                                    .content(format!("Przydzielono ci **{}**", choice))
                            })
                    })
                    .await
                    .unwrap();
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
                ctx.set_presence(Some(Activity::playing("jebać PE")), OnlineStatus::Online)
                    .await;

                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        });

    framework.run().await.unwrap();
}
