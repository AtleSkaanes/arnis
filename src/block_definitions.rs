#![allow(unused)]

use fastnbt::Value;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::colors::RGBTuple;

#[allow(clippy::enum_variant_names)]
#[derive(Copy, Clone, PartialEq, Eq, Ord, PartialOrd, Hash, Debug)]
pub enum Block {
    // Lazy static blocks
    AcaciaPlanks,
    Air,
    Andesite,
    BirchLeaves,
    BirchLog,
    BlackConcrete,
    Blackstone,
    BlueFlower,
    BlueTerracotta,
    Brick,
    Cauldron,
    ChiseledStoneBricks,
    CobblestoneWall,
    Cobblestone,
    CrackedPolishedBlackstoneBricks,
    CrackedStoneBricks,
    CrimsonPlanks,
    CutSandstone,
    CyanConcrete,
    DarkOakPlanks,
    DeepslateBricks,
    Diorite,
    Dirt,
    EndStoneBricks,
    Farmland,
    Glass,
    Glowstone,
    Granite,
    GrassBlock,
    Grass,
    Gravel,
    GrayConcrete,
    GrayTerracotta,
    GreenStainedHardenedClay,
    GreenWool,
    HayBale,
    IronBars,
    IronBlock,
    JunglePlanks,
    Ladder,
    LightBlueConcrete,
    LightBlueTerracotta,
    LightGrayConcrete,
    MossBlock,
    MossyCobblestone,
    MudBricks,
    NetherBricks,
    OakFence,
    OakLeaves,
    OakLog,
    OakPlanks,
    OakSlab,
    OrangeTerracotta,
    Podzol,
    PolishedAndesite,
    PolishedBasalt,
    PolishedBlackstoneBricks,
    PolishedBlackstone,
    PolishedDeepslate,
    PolishedDiorite,
    PolishedGranite,
    Prismarine,
    PurpurBlock,
    PurpurPillar,
    QuartzBricks,
    Rail,
    RedFlower,
    RedNetherBricks,
    RedTerracotta,
    RedWool,
    Sand,
    Sandstone,
    Scaffolding,
    SmoothQuartz,
    SmoothRedSandstone,
    SmoothSandstone,
    SmoothStone,
    Sponge,
    SpruceLog,
    SprucePlanks,
    StoneBlockSlab,
    StoneBrickSlab,
    StoneBricks,
    Stone,
    Terracotta,
    WarpedPlanks,
    Water,
    WhiteConcrete,
    WhiteFlower,
    WhiteStainedGlass,
    WhiteTerracotta,
    WhiteWool,
    YellowConcrete,
    YellowFlower,
    YellowWool,
    LimeConcrete,
    CyanWool,
    BlueConcrete,
    PurpleConcrete,
    RedConcrete,
    MagentaConcrete,
    BrownWool,
    OxidizedCopper,
    YellowTerracotta,
    SnowBlock,
    SnowLayer,
    Sign,

    Carrots,
    DarkOakDoorLower,
    DarkOakDoorUpper,
    Potatoes,
    Wheat,

    Bedrock,
}

impl Block {
    pub fn namespace(&self) -> &str {
        "mincraft"
    }

    pub fn name(&self) -> &str {
        match self {
            Self::AcaciaPlanks => "acacia_planks",
            Self::Air => "air",
            Self::Andesite => "andesite",
            Self::BirchLeaves => "birch_leaves",
            Self::BirchLog => "birch_log",
            Self::BlackConcrete => "black_concrete",
            Self::Blackstone => "blackstone",
            Self::BlueFlower => "blue_orchid",
            Self::BlueTerracotta => "blue_terracotta",
            Self::Brick => "bricks",
            Self::Cauldron => "cauldron",
            Self::ChiseledStoneBricks => "chiseled_stone_bricks",
            Self::CobblestoneWall => "cobblestone_wall",
            Self::Cobblestone => "cobblestone",
            Self::CrackedPolishedBlackstoneBricks => "cracked_polished_blackstone_bricks",
            Self::CrackedStoneBricks => "cracked_stone_bricks",
            Self::CrimsonPlanks => "crimson_planks",
            Self::CutSandstone => "cut_sandstone",
            Self::CyanConcrete => "cyan_concrete",
            Self::DarkOakPlanks => "dark_oak_planks",
            Self::DeepslateBricks => "deepslate_bricks",
            Self::Diorite => "diorite",
            Self::Dirt => "dirt",
            Self::EndStoneBricks => "end_stone_bricks",
            Self::Farmland => "farmland",
            Self::Glass => "glass_pane",
            Self::Glowstone => "glowstone",
            Self::Granite => "granite",
            Self::GrassBlock => "grass_block",
            Self::Grass => "tall_grass",
            Self::Gravel => "gravel",
            Self::GrayConcrete => "gray_concrete",
            Self::GrayTerracotta => "gray_terracotta",
            Self::GreenStainedHardenedClay => "green_terracotta",
            Self::GreenWool => "green_wool",
            Self::HayBale => "hay_block",
            Self::IronBars => "iron_bars",
            Self::IronBlock => "iron_block",
            Self::JunglePlanks => "jungle_planks",
            Self::Ladder => "ladder",
            Self::LightBlueConcrete => "light_blue_concrete",
            Self::LightBlueTerracotta => "light_blue_terracotta",
            Self::LightGrayConcrete => "light_gray_concrete",
            Self::MossBlock => "moss_block",
            Self::MossyCobblestone => "mossy_cobblestone",
            Self::MudBricks => "mud_bricks",
            Self::NetherBricks => "nether_bricks",
            Self::OakFence => "oak_fence",
            Self::OakLeaves => "oak_leaves",
            Self::OakLog => "oak_log",
            Self::OakPlanks => "oak_planks",
            Self::OakSlab => "oak_slab",
            Self::OrangeTerracotta => "orange_terracotta",
            Self::Podzol => "podzol",
            Self::PolishedAndesite => "polished_andesite",
            Self::PolishedBasalt => "polished_basalt",
            Self::PolishedBlackstoneBricks => "polished_blackstone_bricks",
            Self::PolishedBlackstone => "polished_blackstone",
            Self::PolishedDeepslate => "polished_deepslate",
            Self::PolishedDiorite => "polished_diorite",
            Self::PolishedGranite => "polished_granite",
            Self::Prismarine => "prismarine",
            Self::PurpurBlock => "purpur_block",
            Self::PurpurPillar => "purpur_pillar",
            Self::QuartzBricks => "quartz_bricks",
            Self::Rail => "rail",
            Self::RedFlower => "poppy",
            Self::RedNetherBricks => "red_nether_bricks",
            Self::RedTerracotta => "red_terracotta",
            Self::RedWool => "red_wool",
            Self::Sand => "sand",
            Self::Sandstone => "sandstone",
            Self::Scaffolding => "scaffolding",
            Self::SmoothQuartz => "smooth_quartz",
            Self::SmoothRedSandstone => "smooth_red_sandstone",
            Self::SmoothSandstone => "smooth_sandstone",
            Self::SmoothStone => "smooth_stone",
            Self::Sponge => "sponge",
            Self::SpruceLog => "spruce_log",
            Self::SprucePlanks => "spruce_planks",
            Self::StoneBlockSlab => "stone_slab",
            Self::StoneBrickSlab => "stone_brick_slab",
            Self::StoneBricks => "stone_bricks",
            Self::Stone => "stone",
            Self::Terracotta => "terracotta",
            Self::WarpedPlanks => "warped_planks",
            Self::Water => "water",
            Self::WhiteConcrete => "white_concrete",
            Self::WhiteFlower => "azure_bluet",
            Self::WhiteStainedGlass => "white_stained_glass",
            Self::WhiteTerracotta => "white_terracotta",
            Self::WhiteWool => "white_wool",
            Self::YellowConcrete => "yellow_concrete",
            Self::YellowFlower => "dandelion",
            Self::YellowWool => "yellow_wool",
            Self::LimeConcrete => "lime_concrete",
            Self::CyanWool => "cyan_wool",
            Self::BlueConcrete => "blue_concrete",
            Self::PurpleConcrete => "purple_concrete",
            Self::RedConcrete => "red_concrete",
            Self::MagentaConcrete => "magenta_concrete",
            Self::BrownWool => "brown_wool",
            Self::OxidizedCopper => "oxidized_copper",
            Self::YellowTerracotta => "yellow_terracotta",
            Self::Carrots => "carrots",
            Self::DarkOakDoorLower => "dark_oak_door",
            Self::DarkOakDoorUpper => "dark_oak_door",
            Self::Potatoes => "potatoes",
            Self::Wheat => "wheat",
            Self::Bedrock => "bedrock",
            Self::SnowBlock => "snow_block",
            Self::SnowLayer => "snow",
            Self::Sign => "oak_sign",
        }
    }

    pub fn properties(&self) -> Option<Value> {
        match self {
            Self::Carrots => Some(Value::Compound({
                let mut map: HashMap<String, Value> = HashMap::new();
                map.insert("age".to_string(), Value::String("7".to_string()));
                map
            })),

            Self::DarkOakDoorLower => Some(Value::Compound({
                let mut map: HashMap<String, Value> = HashMap::new();
                map.insert("half".to_string(), Value::String("lower".to_string()));
                map
            })),

            Self::DarkOakDoorUpper => Some(Value::Compound({
                let mut map: HashMap<String, Value> = HashMap::new();
                map.insert("half".to_string(), Value::String("upper".to_string()));
                map
            })),

            Self::Potatoes => Some(Value::Compound({
                let mut map: HashMap<String, Value> = HashMap::new();
                map.insert("age".to_string(), Value::String("7".to_string()));
                map
            })),

            Self::Potatoes => Some(Value::Compound({
                let mut map: HashMap<String, Value> = HashMap::new();
                map.insert("age".to_string(), Value::String("7".to_string()));
                map
            })),

            Self::Sign => Some(Value::Compound({
                let mut map: HashMap<String, Value> = HashMap::new();
                map.insert("rotation".to_string(), Value::String("6".to_string()));
                map.insert(
                    "waterlogged".to_string(),
                    Value::String("false".to_string()),
                );
                map
            })),

            _ => None,
        }
    }
}

// Variations for building corners
pub fn building_corner_variations() -> Vec<Block> {
    vec![
        Block::StoneBricks,
        Block::Cobblestone,
        Block::Brick,
        Block::MossyCobblestone,
        Block::Sandstone,
        Block::RedNetherBricks,
        Block::Blackstone,
        Block::SmoothQuartz,
        Block::ChiseledStoneBricks,
        Block::PolishedBasalt,
        Block::CutSandstone,
        Block::PolishedBlackstoneBricks,
        Block::Andesite,
        Block::Granite,
        Block::Diorite,
        Block::CrackedStoneBricks,
        Block::Prismarine,
        Block::BlueTerracotta,
        Block::QuartzBricks,
    ]
}

// Variations for building walls
pub fn building_wall_variations() -> Vec<Block> {
    building_wall_color_map()
        .into_iter()
        .map(|(_, block)| block)
        .collect()
}

// https://wiki.openstreetmap.org/wiki/Key:building:colour
pub fn building_wall_color_map() -> Vec<(RGBTuple, Block)> {
    vec![
        ((233, 107, 57), Block::Brick),
        ((18, 12, 13), Block::CrackedPolishedBlackstoneBricks),
        ((76, 127, 153), Block::CyanConcrete),
        ((0, 0, 0), Block::DeepslateBricks),
        ((186, 195, 142), Block::EndStoneBricks),
        ((57, 41, 35), Block::GrayTerracotta),
        ((112, 108, 138), Block::LightBlueTerracotta),
        ((122, 92, 66), Block::MudBricks),
        ((24, 13, 14), Block::NetherBricks),
        ((159, 82, 36), Block::OrangeTerracotta),
        ((128, 128, 128), Block::PolishedAndesite),
        ((174, 173, 174), Block::PolishedDiorite),
        ((141, 101, 142), Block::PurpurPillar),
        ((142, 60, 46), Block::RedTerracotta),
        ((153, 83, 28), Block::SmoothRedSandstone),
        ((224, 216, 175), Block::SmoothSandstone),
        ((188, 182, 179), Block::SmoothStone),
        ((35, 86, 85), Block::WarpedPlanks),
        ((255, 255, 255), Block::WhiteConcrete),
        ((209, 177, 161), Block::WhiteTerracotta),
        ((191, 147, 42), Block::YellowTerracotta),
    ]
}

// Variations for building floors
pub fn building_floor_variations() -> Vec<Block> {
    building_wall_color_map()
        .into_iter()
        .map(|(_, block)| block)
        .collect()
}

pub fn building_floor_color_map() -> Vec<(RGBTuple, Block)> {
    vec![
        ((181, 101, 59), Block::AcaciaPlanks),
        ((22, 15, 16), Block::Blackstone),
        ((104, 51, 74), Block::CrimsonPlanks),
        ((82, 55, 26), Block::DarkOakPlanks),
        ((182, 133, 99), Block::JunglePlanks),
        ((33, 128, 185), Block::LightBlueConcrete),
        ((78, 103, 43), Block::MossBlock),
        ((171, 138, 88), Block::OakPlanks),
        ((0, 128, 0), Block::OxidizedCopper),
        ((18, 12, 13), Block::PolishedBlackstone),
        ((64, 64, 64), Block::PolishedDeepslate),
        ((255, 255, 255), Block::PolishedDiorite),
        ((143, 96, 79), Block::PolishedGranite),
        ((141, 101, 142), Block::PurpurBlock),
        ((128, 0, 0), Block::RedNetherBricks),
        ((153, 83, 28), Block::SmoothRedSandstone),
        ((128, 96, 57), Block::SprucePlanks),
        ((128, 128, 128), Block::StoneBricks),
        ((150, 93, 68), Block::Terracotta),
        ((35, 86, 85), Block::WarpedPlanks),
    ]
}
