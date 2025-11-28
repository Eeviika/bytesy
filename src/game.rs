/* #region Usages */
use macroquad::color::Color;
use slotmap::SlotMap;

use crate::{
    dialog::Dialog,
    entities::{Avatar, Item, Npc, Tile},
    rooms::{Ending, Exit, Room},
};
/* #endregion */

/* #region Keys */
slotmap::new_key_type! { pub struct TileId; }
slotmap::new_key_type! { pub struct RoomId; }
slotmap::new_key_type! { pub struct NpcId; }
slotmap::new_key_type! { pub struct ItemId; }
slotmap::new_key_type! { pub struct ExitId; }
slotmap::new_key_type! { pub struct EndingId; }
slotmap::new_key_type! { pub struct DialogId; }
slotmap::new_key_type! { pub struct PaletteId; }
/* #endregion */

/* #region Structs */
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorType {
    Background,
    Tile,
    Sprite,
    Avatar,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ColorPalette {
    name: String,
    transparency: Color,
    background: Color,
    tile: Color,
    sprite: Color,
    avatar: Color,
}

#[derive(Clone, Debug)]
pub struct World {
    tiles: SlotMap<TileId, Tile>,
    rooms: SlotMap<RoomId, Room>,
    npcs: SlotMap<NpcId, Npc>,
    items: SlotMap<ItemId, Item>,
    exits: SlotMap<ExitId, Exit>,
    endings: SlotMap<EndingId, Ending>,
    dialogs: SlotMap<DialogId, Dialog>,
    palettes: SlotMap<PaletteId, ColorPalette>,
    avatar: Avatar,
}
/* #endregion */

/* #region Implementations */
impl ColorPalette {
    pub fn type_to_color(&self, color_type: ColorType) -> Color {
        match color_type {
            ColorType::Background => self.background,
            ColorType::Avatar => self.avatar,
            ColorType::Sprite => self.sprite,
            ColorType::Tile => self.tile,
        }
    }

    pub fn transparency(&self) -> Color {
        self.transparency
    }
    pub fn background(&self) -> Color {
        self.background
    }
    pub fn tile(&self) -> Color {
        self.tile
    }
    pub fn sprite(&self) -> Color {
        self.sprite
    }
    pub fn avatar(&self) -> Color {
        self.avatar
    }
}

impl Default for ColorPalette {
    fn default() -> Self {
        ColorPalette {
            name: "Terminal".into(),
            transparency: Color { r: 0.0, g: 0.0, b: 0.0, a: 0.0 },
            background: Color { r: 0.0, g: 0.0, b: 0.0, a: 1.0 },
            tile: Color { r: 0.5, g: 0.5, b: 0.5, a: 1.0 },
            sprite: Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 },
            avatar: Color { r: 0.8, g: 1.0, b: 0.8, a: 1.0 }
        }
    }
}
/* #endregion */
