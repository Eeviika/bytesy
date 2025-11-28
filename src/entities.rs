use crate::game::{ColorType, DialogId};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TileData {
    frames: [[[bool; 8]; 8]; 2],
    color_type: ColorType,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Tile {
    pub data: TileData,
    pub is_wall: bool,
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Entity {
    pub data: TileData,
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Avatar {
    pub entity: Entity,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Npc {
    pub entity: Entity,
    pub interact_sound_id: Option<u32>,
    pub dialog_id: Option<DialogId>,
    pub enabled: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Item {
    pub npc: Npc,
}

impl Default for TileData {
    fn default() -> Self {
        TileData {
            frames: [[[false; 8]; 8]; 2],
            color_type: ColorType::Tile
        }
    }
}

impl Default for Avatar {
    fn default() -> Self {
        let data = TileData {
            frames: TileData::default().frames,
            color_type: ColorType::Avatar,
        };
        let entity = Entity {
            data,
            name: "Player".into(),
        };
        Avatar { entity }
    }
}

impl Default for Tile {
    fn default() -> Self {
            let data = TileData {
            frames: TileData::default().frames,
            color_type: ColorType::Tile,
        };
        Tile {
            data,
            is_wall: false,
            name: "Tile".into()
        }
    }
}
