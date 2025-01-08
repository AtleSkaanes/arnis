use crate::args::Args;
use crate::block_definitions::Block;
use crate::bresenham::bresenham_line;
use crate::element_processing::tree::create_tree;
use crate::floodfill::flood_fill_area;
use crate::osm_parser::ProcessedWay;
use crate::world_editor::WorldEditor;
use rand::Rng;

pub fn generate_leisure(
    editor: &mut WorldEditor,
    element: &ProcessedWay,
    ground_level: i32,
    args: &Args,
) {
    if let Some(leisure_type) = element.tags.get("leisure") {
        let mut previous_node: Option<(i32, i32)> = None;
        let mut corner_addup: (i32, i32, i32) = (0, 0, 0);
        let mut current_leisure: Vec<(i32, i32)> = vec![];

        // Determine block type based on leisure type
        let block_type: Block = match leisure_type.as_str() {
            "park" => {
                if args.winter {
                    Block::SnowBlock
                } else {
                    Block::GrassBlock
                }
            }
            "playground" | "recreation_ground" | "pitch" => {
                if let Some(surface) = element.tags.get("surface") {
                    match surface.as_str() {
                        "clay" => Block::Terracotta,
                        "sand" => Block::Sand,
                        "tartan" => Block::RedTerracotta,
                        _ => Block::GreenStainedHardenedClay,
                    }
                } else {
                    Block::GreenStainedHardenedClay
                }
            }
            "garden" => {
                if args.winter {
                    Block::SnowBlock
                } else {
                    Block::GrassBlock
                }
            }
            "swimming_pool" => Block::Water,
            _ => {
                if args.winter {
                    Block::SnowBlock
                } else {
                    Block::GrassBlock
                }
            }
        };

        // Process leisure area nodes
        for node in &element.nodes {
            if let Some(prev) = previous_node {
                // Draw a line between the current and previous node
                let bresenham_points: Vec<(i32, i32, i32)> =
                    bresenham_line(prev.0, ground_level, prev.1, node.x, ground_level, node.z);
                for (bx, _, bz) in bresenham_points {
                    editor.set_block(
                        block_type,
                        bx,
                        ground_level,
                        bz,
                        Some(&[
                            Block::GrassBlock,
                            Block::StoneBricks,
                            Block::SmoothStone,
                            Block::LightGrayConcrete,
                            Block::Cobblestone,
                            Block::GrayConcrete,
                        ]),
                        None,
                    );
                }

                current_leisure.push((node.x, node.z));
                corner_addup.0 += node.x;
                corner_addup.1 += node.z;
                corner_addup.2 += 1;
            }
            previous_node = Some((node.x, node.z));
        }

        // Flood-fill the interior of the leisure area
        if corner_addup != (0, 0, 0) {
            let polygon_coords: Vec<(i32, i32)> = element
                .nodes
                .iter()
                .map(|n: &crate::osm_parser::ProcessedNode| (n.x, n.z))
                .collect();
            let filled_area: Vec<(i32, i32)> =
                flood_fill_area(&polygon_coords, args.timeout.as_ref());

            for (x, z) in filled_area {
                editor.set_block(
                    block_type,
                    x,
                    ground_level,
                    z,
                    Some(&[Block::GrassBlock]),
                    None,
                );

                // Add decorative elements for parks and gardens
                if matches!(leisure_type.as_str(), "park" | "garden")
                    && editor.check_for_block(x, ground_level, z, Some(&[Block::GrassBlock]), None)
                {
                    let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
                    let random_choice: i32 = rng.gen_range(0..1000);

                    match random_choice {
                        0 => {
                            // Benches
                            editor.set_block(Block::OakLog, x, ground_level + 1, z, None, None);
                            editor.set_block(Block::OakLog, x + 1, ground_level + 1, z, None, None);
                            editor.set_block(Block::OakLog, x - 1, ground_level + 1, z, None, None);
                        }
                        1..=30 => {
                            // Flowers
                            let flower_choice = match rng.gen_range(0..4) {
                                0 => Block::RedFlower,
                                1 => Block::YellowFlower,
                                2 => Block::BlueFlower,
                                _ => Block::WhiteFlower,
                            };
                            editor.set_block(flower_choice, x, ground_level + 1, z, None, None);
                        }
                        31..=70 => {
                            // Grass
                            editor.set_block(Block::Grass, x, ground_level + 1, z, None, None);
                        }
                        71..=80 => {
                            // Tree
                            create_tree(
                                editor,
                                x,
                                ground_level + 1,
                                z,
                                rng.gen_range(1..=3),
                                args.winter,
                            );
                        }
                        _ => {}
                    }
                }

                // Add playground or recreation ground features
                if matches!(leisure_type.as_str(), "playground" | "recreation_ground") {
                    let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
                    let random_choice: i32 = rng.gen_range(0..5000);

                    match random_choice {
                        0..=10 => {
                            // Swing set
                            for y in 1..=4 {
                                editor.set_block(
                                    Block::OakFence,
                                    x - 1,
                                    ground_level + y,
                                    z,
                                    None,
                                    None,
                                );
                                editor.set_block(
                                    Block::OakFence,
                                    x + 1,
                                    ground_level + y,
                                    z,
                                    None,
                                    None,
                                );
                            }
                            editor.set_block(Block::OakFence, x, ground_level + 4, z, None, None);
                            editor.set_block(
                                Block::StoneBlockSlab,
                                x,
                                ground_level + 2,
                                z,
                                None,
                                None,
                            );
                        }
                        11..=20 => {
                            // Slide
                            editor.set_block(Block::OakSlab, x, ground_level + 1, z, None, None);
                            editor.set_block(
                                Block::OakSlab,
                                x + 1,
                                ground_level + 2,
                                z,
                                None,
                                None,
                            );
                            editor.set_block(
                                Block::OakSlab,
                                x + 2,
                                ground_level + 3,
                                z,
                                None,
                                None,
                            );

                            editor.set_block(
                                Block::OakPlanks,
                                x + 2,
                                ground_level + 2,
                                z,
                                None,
                                None,
                            );
                            editor.set_block(
                                Block::OakPlanks,
                                x + 2,
                                ground_level + 1,
                                z,
                                None,
                                None,
                            );

                            editor.set_block(
                                Block::Ladder,
                                x + 2,
                                ground_level + 2,
                                z - 1,
                                None,
                                None,
                            );
                            editor.set_block(
                                Block::Ladder,
                                x + 2,
                                ground_level + 1,
                                z - 1,
                                None,
                                None,
                            );
                        }
                        21..=30 => {
                            // Sandpit
                            editor.fill_blocks(
                                Block::Sand,
                                x - 3,
                                ground_level,
                                z - 3,
                                x + 3,
                                ground_level,
                                z + 3,
                                Some(&[Block::GreenStainedHardenedClay]),
                                None,
                            );
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}
