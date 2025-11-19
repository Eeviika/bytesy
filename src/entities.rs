#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TileType {
    Wall,
    Character,
    Interactable
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TileData {
    bits: [[bool; 8]; 8],
    tile_type: TileType,
}

pub struct Tile {
    pub data: TileData,
    pub x: u8,
    pub y: u8,
}

pub struct Entity {
    pub data: TileData,
    pub x: u8,
    pub y: u8,
}

pub struct Avatar {
    pub entity: Entity,
}

pub struct NPC {
    pub entity: Entity,
}

pub struct Item {
    pub npc: NPC,
}