use crate::args::Args;
use crate::block_definitions::Block;
use crate::bresenham::bresenham_line;
use crate::element_processing::tree::create_tree;
use crate::floodfill::flood_fill_area;
use crate::osm_parser::ProcessedWay;
use crate::world_editor::WorldEditor;
use rand::Rng;

pub fn generate_landuse(
    editor: &mut WorldEditor,
    element: &ProcessedWay,
    ground_level: i32,
    args: &Args,
) {
    let mut previous_node: Option<(i32, i32)> = None;
    let mut corner_addup: (i32, i32, i32) = (0, 0, 0);
    let mut current_landuse: Vec<(i32, i32)> = vec![];

    // Determine block type based on landuse tag
    let binding: String = "".to_string();
    let landuse_tag: &String = element.tags.get("landuse").unwrap_or(&binding);

    let block_type = match landuse_tag.as_str() {
        "greenfield" | "meadow" | "grass" => {
            if args.winter {
                Block::SnowBlock
            } else {
                Block::GrassBlock
            }
        }
        "farmland" => Block::Farmland,
        "forest" => {
            if args.winter {
                Block::SnowBlock
            } else {
                Block::GrassBlock
            }
        }
        "cemetery" => Block::Podzol,
        "beach" => Block::Sand,
        "construction" => Block::Dirt,
        "traffic_island" => Block::StoneBlockSlab,
        "residential" => Block::StoneBricks,
        "commercial" => Block::SmoothStone,
        "education" => Block::LightGrayConcrete,
        "industrial" => Block::Cobblestone,
        "military" => Block::GrayConcrete,
        "railway" => Block::Gravel,
        _ => {
            if args.winter {
                Block::SnowBlock
            } else {
                Block::GrassBlock
            }
        }
    };

    let bresenham_block: Block = if args.winter {
        Block::SnowBlock
    } else {
        Block::GrassBlock
    };

    // Process landuse nodes to fill the area
    for node in &element.nodes {
        let x: i32 = node.x;
        let z: i32 = node.z;

        if let Some(prev) = previous_node {
            // Generate the line of coordinates between the two nodes
            let bresenham_points: Vec<(i32, i32, i32)> =
                bresenham_line(prev.0, ground_level, prev.1, x, ground_level, z);
            for (bx, _, bz) in bresenham_points {
                editor.set_block(bresenham_block, bx, ground_level, bz, None, None);
            }

            current_landuse.push((x, z));
            corner_addup = (corner_addup.0 + x, corner_addup.1 + z, corner_addup.2 + 1);
        }

        previous_node = Some((x, z));
    }

    // If there are landuse nodes, flood-fill the area
    if !current_landuse.is_empty() {
        let polygon_coords: Vec<(i32, i32)> = element.nodes.iter().map(|n| (n.x, n.z)).collect();
        let floor_area: Vec<(i32, i32)> = flood_fill_area(&polygon_coords, args.timeout.as_ref());

        let mut rng: rand::prelude::ThreadRng = rand::thread_rng();

        for (x, z) in floor_area {
            if landuse_tag == "traffic_island" {
                editor.set_block(block_type, x, ground_level + 1, z, None, None);
            } else if landuse_tag == "construction" || landuse_tag == "railway" {
                editor.set_block(block_type, x, ground_level, z, None, Some(&[Block::Sponge]));
            } else {
                editor.set_block(block_type, x, ground_level, z, None, None);
            }

            // Add specific features for different landuse types
            match landuse_tag.as_str() {
                "cemetery" => {
                    if (x % 3 == 0) && (z % 3 == 0) {
                        let random_choice: i32 = rng.gen_range(0..100);
                        if random_choice < 15 {
                            // Place graves
                            if editor.check_for_block(
                                x,
                                ground_level,
                                z,
                                Some(&[Block::Podzol]),
                                None,
                            ) {
                                if rng.gen_bool(0.5) {
                                    editor.set_block(
                                        Block::Cobblestone,
                                        x - 1,
                                        ground_level + 1,
                                        z,
                                        None,
                                        None,
                                    );
                                    editor.set_block(
                                        Block::StoneBrickSlab,
                                        x - 1,
                                        ground_level + 2,
                                        z,
                                        None,
                                        None,
                                    );
                                    editor.set_block(
                                        Block::StoneBrickSlab,
                                        x,
                                        ground_level + 1,
                                        z,
                                        None,
                                        None,
                                    );
                                    editor.set_block(
                                        Block::StoneBrickSlab,
                                        x + 1,
                                        ground_level + 1,
                                        z,
                                        None,
                                        None,
                                    );
                                } else {
                                    editor.set_block(
                                        Block::Cobblestone,
                                        x,
                                        ground_level + 1,
                                        z - 1,
                                        None,
                                        None,
                                    );
                                    editor.set_block(
                                        Block::StoneBrickSlab,
                                        x,
                                        ground_level + 2,
                                        z - 1,
                                        None,
                                        None,
                                    );
                                    editor.set_block(
                                        Block::StoneBrickSlab,
                                        x,
                                        ground_level + 1,
                                        z,
                                        None,
                                        None,
                                    );
                                    editor.set_block(
                                        Block::StoneBrickSlab,
                                        x,
                                        ground_level + 1,
                                        z + 1,
                                        None,
                                        None,
                                    );
                                }
                            }
                        } else if random_choice < 30 {
                            if editor.check_for_block(
                                x,
                                ground_level,
                                z,
                                Some(&[Block::Podzol]),
                                None,
                            ) {
                                editor.set_block(
                                    Block::RedFlower,
                                    x,
                                    ground_level + 1,
                                    z,
                                    None,
                                    None,
                                );
                            }
                        } else if random_choice < 33 {
                            create_tree(
                                editor,
                                x,
                                ground_level + 1,
                                z,
                                rng.gen_range(1..=3),
                                args.winter,
                            );
                        }
                    }
                }
                "forest" => {
                    if !editor.check_for_block(x, ground_level, z, None, Some(&[Block::Water])) {
                        let random_choice: i32 = rng.gen_range(0..21);
                        if random_choice == 20 {
                            create_tree(
                                editor,
                                x,
                                ground_level + 1,
                                z,
                                rng.gen_range(1..=3),
                                args.winter,
                            );
                        } else if random_choice == 2 {
                            let flower_block: Block = match rng.gen_range(1..=4) {
                                1 => Block::RedFlower,
                                2 => Block::BlueFlower,
                                3 => Block::YellowFlower,
                                _ => Block::WhiteFlower,
                            };
                            editor.set_block(flower_block, x, ground_level + 1, z, None, None);
                        } else if random_choice <= 1 {
                            editor.set_block(Block::Grass, x, ground_level + 1, z, None, None);
                        }
                    }
                }
                "farmland" => {
                    // Check if the current block is not water or another undesired block
                    if !editor.check_for_block(x, ground_level, z, None, Some(&[Block::Water])) {
                        if x % 15 == 0 || z % 15 == 0 {
                            // Place water on the edges
                            editor.set_block(
                                Block::Water,
                                x,
                                ground_level,
                                z,
                                Some(&[Block::Farmland]),
                                None,
                            );
                            editor.set_block(
                                Block::Air,
                                x,
                                ground_level + 1,
                                z,
                                Some(&[
                                    Block::Grass,
                                    Block::Wheat,
                                    Block::Carrots,
                                    Block::Potatoes,
                                ]),
                                None,
                            );
                        } else {
                            // Set the block below as farmland
                            editor.set_block(Block::Farmland, x, ground_level, z, None, None);

                            // If a random condition is met, place a special object
                            if rng.gen_range(0..76) == 0 {
                                let special_choice: i32 = rng.gen_range(1..=10);
                                if special_choice <= 2 {
                                    create_tree(
                                        editor,
                                        x,
                                        ground_level + 1,
                                        z,
                                        rng.gen_range(1..=3),
                                        args.winter,
                                    );
                                } else if special_choice <= 6 {
                                    editor.set_block(
                                        Block::HayBale,
                                        x,
                                        ground_level + 1,
                                        z,
                                        None,
                                        None,
                                    );
                                } else {
                                    editor.set_block(
                                        Block::OakLeaves,
                                        x,
                                        ground_level + 1,
                                        z,
                                        None,
                                        None,
                                    );
                                }
                            } else {
                                // Set crops only if the block below is farmland
                                if editor.check_for_block(
                                    x,
                                    ground_level,
                                    z,
                                    Some(&[Block::Farmland]),
                                    None,
                                ) {
                                    let crop_choice =
                                        [Block::Wheat, Block::Carrots, Block::Potatoes]
                                            [rng.gen_range(0..3)];
                                    editor.set_block(
                                        crop_choice,
                                        x,
                                        ground_level + 1,
                                        z,
                                        None,
                                        None,
                                    );
                                }
                            }
                        }
                    }
                }
                "construction" => {
                    let random_choice: i32 = rng.gen_range(0..1501);
                    if random_choice < 6 {
                        editor.set_block(Block::Scaffolding, x, ground_level + 1, z, None, None);
                        if random_choice < 2 {
                            editor.set_block(
                                Block::Scaffolding,
                                x,
                                ground_level + 2,
                                z,
                                None,
                                None,
                            );
                            editor.set_block(
                                Block::Scaffolding,
                                x,
                                ground_level + 3,
                                z,
                                None,
                                None,
                            );
                        } else if random_choice < 4 {
                            editor.set_block(
                                Block::Scaffolding,
                                x,
                                ground_level + 2,
                                z,
                                None,
                                None,
                            );
                            editor.set_block(
                                Block::Scaffolding,
                                x,
                                ground_level + 3,
                                z,
                                None,
                                None,
                            );
                            editor.set_block(
                                Block::Scaffolding,
                                x,
                                ground_level + 4,
                                z,
                                None,
                                None,
                            );
                            editor.set_block(
                                Block::Scaffolding,
                                x,
                                ground_level + 1,
                                z + 1,
                                None,
                                None,
                            );
                        } else {
                            editor.set_block(
                                Block::Scaffolding,
                                x,
                                ground_level + 2,
                                z,
                                None,
                                None,
                            );
                            editor.set_block(
                                Block::Scaffolding,
                                x,
                                ground_level + 3,
                                z,
                                None,
                                None,
                            );
                            editor.set_block(
                                Block::Scaffolding,
                                x,
                                ground_level + 4,
                                z,
                                None,
                                None,
                            );
                            editor.set_block(
                                Block::Scaffolding,
                                x,
                                ground_level + 5,
                                z,
                                None,
                                None,
                            );
                            editor.set_block(
                                Block::Scaffolding,
                                x - 1,
                                ground_level + 1,
                                z,
                                None,
                                None,
                            );
                            editor.set_block(
                                Block::Scaffolding,
                                x + 1,
                                ground_level + 1,
                                z - 1,
                                None,
                                None,
                            );
                        }
                    } else if random_choice < 30 {
                        let construction_items: [Block; 11] = [
                            Block::OakLog,
                            Block::Cobblestone,
                            Block::Gravel,
                            Block::Glowstone,
                            Block::Stone,
                            Block::CobblestoneWall,
                            Block::BlackConcrete,
                            Block::Sand,
                            Block::OakPlanks,
                            Block::Dirt,
                            Block::Brick,
                        ];
                        editor.set_block(
                            construction_items[rng.gen_range(0..construction_items.len())],
                            x,
                            ground_level + 1,
                            z,
                            None,
                            None,
                        );
                    } else if random_choice < 35 {
                        if random_choice < 30 {
                            editor.set_block(Block::Dirt, x, ground_level + 1, z, None, None);
                            editor.set_block(Block::Dirt, x, ground_level + 2, z, None, None);
                            editor.set_block(Block::Dirt, x + 1, ground_level + 1, z, None, None);
                            editor.set_block(Block::Dirt, x, ground_level + 1, z + 1, None, None);
                        } else {
                            editor.set_block(Block::Dirt, x, ground_level + 1, z, None, None);
                            editor.set_block(Block::Dirt, x, ground_level + 2, z, None, None);
                            editor.set_block(Block::Dirt, x - 1, ground_level + 1, z, None, None);
                            editor.set_block(Block::Dirt, x, ground_level + 1, z - 1, None, None);
                        }
                    } else if random_choice < 150 {
                        editor.set_block(
                            Block::Air,
                            x,
                            ground_level,
                            z,
                            None,
                            Some(&[Block::Sponge]),
                        );
                    }
                }
                "grass" => {
                    if rng.gen_range(1..=7) != 1
                        && !editor.check_for_block(x, ground_level, z, None, Some(&[Block::Water]))
                    {
                        editor.set_block(Block::Grass, x, ground_level + 1, z, None, None);
                    }
                }
                "meadow" => {
                    if !editor.check_for_block(x, ground_level, z, None, Some(&[Block::Water])) {
                        let random_choice: i32 = rng.gen_range(0..1001);
                        if random_choice < 5 {
                            create_tree(
                                editor,
                                x,
                                ground_level + 1,
                                z,
                                rng.gen_range(1..=3),
                                args.winter,
                            );
                        } else if random_choice < 800 {
                            editor.set_block(Block::Grass, x, ground_level + 1, z, None, None);
                        }
                    }
                }
                _ => {}
            }
        }
    }
}
