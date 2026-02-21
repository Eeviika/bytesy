use crate::game::{ColorType, DialogId};

/// Data for an 8x8 texture.
///
/// `TileData` represents a small, 8x8, boolean pixel texture that can be
/// used by tiles, NPCs, and other engine elements. Each pixel is stored as a
/// `bool`, allowing what is as close to one bit per pixel as can be in standard Rust.
/// The data can contain up to **two frames** of animation. Most things use this struct.
///
/// ## Fields
///
/// - `frames` - A `[[[bool; 8]; 8]; 2]` array that is indexed by frame;row;col.
/// - `color_type` - A `ColorType` enum that determines how the renderer will color this texture.
///
/// ## Example
///
/// ```ignore
/// use crate::entities::TileData;
/// use crate::game::ColorType;
///
/// let default_data = TileData::default();
///
/// let custom_data = TileData {
///     frames: [[[false; 8]; 8]; 2],
///     color_type: ColorType::Tile,
/// };
/// ```
///
/// ## Notes
///
/// `TileData` is inexpensive to create, but writing out full frame data in code can be tedious and unnecessary.
/// Alternative means of creation are encouraged, such as using the in-engine editor.
/// (Or just don't...)
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
    pub interact_sound_id: Option<u32>, // todo: make a SoundId type
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
            color_type: ColorType::Tile,
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
            name: "Tile".into(),
        }
    }
}
