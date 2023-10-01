const LISTA_PIWEK: &'static [&str] = &[
    "https://www.kp.pl/img/butelki/tyskie.png?v2",
    "https://www.kp.pl/img/butelki/lech.png?v2",
    "https://www.kp.pl/img/butelki/zubr.png?v2",
    "https://www.kp.pl/img/butelki/debowe.png?v2",
    "https://www.kp.pl/img/butelki/ksiazece.png?v2",
    "https://www.kp.pl/img/butelki/redds.png?v2",
    "https://www.kp.pl/img/butelki/hardmade.png?v2",
    "https://www.kp.pl/img/butelki/peroni.png?v2",
    "https://www.kp.pl/img/butelki/wojak.png?v2",
    "https://www.kp.pl/img/butelki/kozel.png?v2",
    "https://www.kp.pl/uploads/kp/beers2/Puszka_pilzner.png",
    "https://www.kp.pl/uploads/kp/beers2/Puszka_pilzner.png",
    "https://www.kp.pl/uploads/kp/beers2/Tyskie_klasyczne.png",
    "https://www.kp.pl/uploads/kp/beers2/TYS_ZERO_BT500_16_02_23.png",
    "https://www.kp.pl/uploads/kp/beers2/TYS_GRONIE_BT500_16_02_23.png",
    "https://www.kp.pl/uploads/kp/beers2/LECH_STRAWBERRY.png",
    "https://www.kp.pl/uploads/kp/beers2/Lech_icemojito.png",
    "https://www.kp.pl/uploads/kp/beers2/Lech_iceshandy.png",
    "https://www.kp.pl/uploads/kp/beers2/LECH_FREE_SMAKOWKI_RB500_packshot_DFS.png",
    "https://www.kp.pl/uploads/kp/beers2/LECH_FREE_SMAKOWKI_RB500_packshot_marakuja.png",
    "https://www.kp.pl/uploads/kp/beers2/LECH_FREE_SMAKOWKI_RB500_packshot_granat.png",
    "https://www.kp.pl/uploads/kp/beers2/LECH_FREE_SMAKOWKI_RB500_packshot_POMELO.png",
    "https://www.kp.pl/uploads/kp/beers2/LECH_FREE_SMAKOWKI_RB500_packshot_arbuz.png",
    "https://www.kp.pl/uploads/kp/beers2/LECH_FREE_SMAKOWKI_RB500_packshot_limonka.png",
    "https://www.kp.pl/uploads/kp/beers2/LECH_ACTIVE_RB500_MANGO_CYTRYNA_szron_PNG_prev.png",
    "https://www.kp.pl/uploads/kp/beers2/LECH_ACTIVE_RB500_LICZI_CYTRYNA_szron_PNG_prev.png",
    "https://www.kp.pl/uploads/kp/beers2/Lech_FREE_LAGER_RB500.png",
    "https://www.kp.pl/uploads/kp/beers2/Lech_pils.png",
    "https://www.kp.pl/uploads/kp/beers2/lech-easy.png",
    "https://www.kp.pl/uploads/kp/beers2/Lech_premium.png",
    "https://a.assecobs.com/_img/amigohurt/832552d1-7e7d-4cd0-92ea-75520c756777/piwo-but-zw-harnas-0-5l-20-.jpg",
    "https://res.cloudinary.com/dj484tw6k/image/upload/v1678146885/3362.png",
    "https://www.kp.pl/uploads/kp/beers2/Zubr.png",
    "https://www.kp.pl/uploads/kp/beers2/Ksiazece_lager.jpg",
    "https://www.kp.pl/uploads/kp/beers2/Ksiazece_ciemnelagodne.png",
    "https://www.kp.pl/uploads/kp/beers2/Ksiazece_czerwonylager.png",
    "https://www.kp.pl/uploads/kp/beers2/KSI_Pszenica_NAB_1.png ",
    "https://www.kp.pl/uploads/kp/beers2/Ksiazece_zlotepszeniczne.png",
    "https://www.kp.pl/uploads/kp/beers2/Ksiazece_porter.png",
    "https://www.kp.pl/uploads/kp/beers2/Ksiazece_irish_ale.png",
    "https://www.kp.pl/uploads/kp/beers2/KSI_IPA_NAB_00_1.png",
    "https://www.kp.pl/uploads/kp/beers2/Ksiazece_ipa.png",
    "https://www.kp.pl/uploads/kp/beers2/Debowe.png",
    "https://www.kp.pl/uploads/kp/beers2/Redds_mango.png",
    "https://www.kp.pl/uploads/kp/beers2/Redds_zurawina.png",
    "https://www.kp.pl/uploads/kp/beers2/Redds_jablko.png",
    "https://www.kp.pl/uploads/kp/beers2/redds_marakuja.png",
    "https://www.kp.pl/uploads/kp/beers2/HARDMADE_Peach_V6.png",
    "https://www.kp.pl/uploads/kp/beers2/HARDMADE_Raspberry_V7.png",
    "https://www.kp.pl/uploads/kp/beers2/HARDMADE_Pear_V6.png",
    "https://www.kp.pl/uploads/kp/beers2/HARDMADE_Rhubarb_V10.png",
    "https://www.kp.pl/uploads/kp/beers2/HARDMADE_Strawberry_V4.png",
    "https://www.kp.pl/uploads/kp/beers2/CJ_ORGINAL_2022.png",
    "https://www.kp.pl/uploads/kp/beers2/CJ_PIRATE_ORANGE_2022.png",
    "https://www.kp.pl/uploads/kp/beers2/CJ_CUBA.png",
    "https://www.kp.pl/uploads/kp/beers2/CJ_DAIQUIRI_BEZ.png",
    "https://www.kp.pl/uploads/kp/beers2/CJ_SOUR_3.png",
    "https://www.kp.pl/uploads/kp/beers2/CJ_LIME.png",
    "https://www.kp.pl/uploads/kp/beers2/CJ_MOJITO.png",
    "https://www.kp.pl/uploads/kp/beers2/peroni.jpg",
    "https://www.kp.pl/uploads/kp/beers2/PNA_0_0_330ML_BOTTLE_UNSPRITZ_CAPON_HR.png",
    "https://www.kp.pl/uploads/kp/beers2/Wojak.png",
    "https://www.kp.pl/uploads/kp/beers2/Grolsch.png",
    "https://www.kp.pl/uploads/kp/beers2/Pilsner_urquell.png",
    "https://www.kp.pl/uploads/kp/beers2/pilsner_kufel.jpg",
    "https://www.kp.pl/uploads/kp/beers2/Kozel_lezak.png",
    "https://www.kp.pl/uploads/kp/beers2/Kozel_cerny.png",
    "https://www.kp.pl/uploads/kp/beers2/KOZEL_00_PACKSHOT.png",
];

use poise::serenity_prelude as serenity;
use poise::Event;
use rand::seq::SliceRandom;
use serenity::model::prelude::*;
use std::collections::HashMap;

async fn event_handler(
    ctx: &serenity::Context,
    event: &Event<'_>,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
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
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_MEMBERS;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            event_handler: |_ctx, event, _framework, _data| {
                Box::pin(event_handler(_ctx, event, _framework, _data))
            },
            commands: vec![piwko()],
            ..Default::default()
        })
        .token("MTE1NzQxOTcyMzEzMTIxMTg5Ng.GU2Afi.UfX_vP1D66vIKXzLvDDhB8lFJwToT1DQTrOc-k")
        .intents(intents)
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        });

    framework.run().await.unwrap();
}