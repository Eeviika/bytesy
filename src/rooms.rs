use macroquad::math::IVec2;

use crate::game::{DialogId, EndingId, ExitId, ItemId, NpcId, TileId};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TransitionEffect {
    FadeToWhite,
    FadeToBlack,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Exit {
    name: String,
    one_way: bool,
    enabled: bool,
    entrance_pos: IVec2,
    exit_pos: IVec2,
    dialog_id: Option<DialogId>,
    effect: Option<TransitionEffect>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Ending {
    name: String,
    position: IVec2,
    enabled: bool,
    dialog_id: DialogId,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Room {
    name: String,
    tiles: [[Option<TileId>; 16]; 16],
    npcs: [[Option<NpcId>; 16]; 16],
    items: [[Option<ItemId>; 16]; 16],
    exits: [[Option<ExitId>; 16]; 16],
    endings: [[Option<EndingId>; 16]; 16],
    avatar: Option<(u16, u16)>,
}
