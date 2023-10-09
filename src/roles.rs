use poise::serenity_prelude as serenity;
use serenity::model::id::{MessageId, RoleId};

// pub static REACTION_ROLES: phf::Map<MessageId, phf::Map<&'static str, RoleId>> = phf_map! {
//     MessageId(1158079176096628856) => phf_map!{
//         "Grupa 1" => RoleId(1157272795902918657),
//     }
// };

pub struct ReactionRole<'a> {
    pub message_id: MessageId,
    pub interaction_id: &'a str,
    pub role_id: RoleId,
}

pub const REACTION_ROLES: &'static [ReactionRole] = &[
    ReactionRole {
        message_id: MessageId(1158079176096628856),
        interaction_id: "Grupa 1",
        role_id: RoleId(1157242625770934282),
    },
    ReactionRole {
        message_id: MessageId(1158079176096628856),
        interaction_id: "Grupa 2",
        role_id: RoleId(1157272733445529640),
    },
    ReactionRole {
        message_id: MessageId(1158079176096628856),
        interaction_id: "Grupa 3",
        role_id: RoleId(1157272763317362749),
    },
    ReactionRole {
        message_id: MessageId(1160979940280389802),
        interaction_id: "Grupa 1",
        role_id: RoleId(1157272795902918657),
    },
    ReactionRole {
        message_id: MessageId(1160979940280389802),
        interaction_id: "Grupa 2",
        role_id: RoleId(1157272860293877814),
    },
    ReactionRole {
        message_id: MessageId(1160979940280389802),
        interaction_id: "Grupa 3",
        role_id: RoleId(1157272882704035850),
    },
    ReactionRole {
        message_id: MessageId(1160979940280389802),
        interaction_id: "Grupa 4",
        role_id: RoleId(1157272923523006494),
    },
    ReactionRole {
        message_id: MessageId(1160979940280389802),
        interaction_id: "Grupa 5",
        role_id: RoleId(1157272960810373191),
    },
    ReactionRole {
        message_id: MessageId(1160979940280389802),
        interaction_id: "Grupa 6",
        role_id: RoleId(1157272990069817436),
    },
    ReactionRole {
        message_id: MessageId(1160980682433118369),
        interaction_id: "Grupa B2 1",
        role_id: RoleId(1160901910459596901),
    },
    ReactionRole {
        message_id: MessageId(1160980682433118369),
        interaction_id: "Grupa B2 2",
        role_id: RoleId(1160902147190292490),
    },
    ReactionRole {
        message_id: MessageId(1160980682433118369),
        interaction_id: "Grupa C1 1",
        role_id: RoleId(1160902152651296808),
    },
    ReactionRole {
        message_id: MessageId(1160980682433118369),
        interaction_id: "Grupa C1 2",
        role_id: RoleId(1160902159018242059),
    },
    ReactionRole {
        message_id: MessageId(1160980682433118369),
        interaction_id: "Grupa C1 3",
        role_id: RoleId(1160902168468013128),
    },
];
