use crate::args::Args;
use crate::block_definitions::Block;
use crate::bresenham::bresenham_line;
use crate::floodfill::flood_fill_area;
use crate::osm_parser::{ProcessedElement, ProcessedWay};
use crate::world_editor::WorldEditor;

pub fn generate_highways(
    editor: &mut WorldEditor,
    element: &ProcessedElement,
    ground_level: i32,
    args: &Args,
) {
    if let Some(highway_type) = element.tags().get("highway") {
        if highway_type == "street_lamp" {
            // Handle street lamps
            if let ProcessedElement::Node(first_node) = element {
                let x: i32 = first_node.x;
                let z: i32 = first_node.z;
                for y in 1..=4 {
                    editor.set_block(Block::OakFence, x, ground_level + y, z, None, None);
                }
                editor.set_block(Block::Glowstone, x, ground_level + 5, z, None, None);
            }
        } else if highway_type == "crossing" {
            // Handle traffic signals for crossings
            if let Some(crossing_type) = element.tags().get("crossing") {
                if crossing_type == "traffic_signals" {
                    if let ProcessedElement::Node(node) = element {
                        let x: i32 = node.x;
                        let z: i32 = node.z;
                        for y in 1..=3 {
                            editor.set_block(
                                Block::CobblestoneWall,
                                x,
                                ground_level + y,
                                z,
                                None,
                                None,
                            );
                        }

                        editor.set_block(Block::GreenWool, x, ground_level + 4, z, None, None);
                        editor.set_block(Block::YellowWool, x, ground_level + 5, z, None, None);
                        editor.set_block(Block::RedWool, x, ground_level + 6, z, None, None);

                        if args.winter {
                            editor.set_block(Block::SnowLayer, x, ground_level + 7, z, None, None);
                        }
                    }
                }
            }
        } else if highway_type == "bus_stop" {
            // Handle bus stops
            if let ProcessedElement::Node(node) = element {
                let x: i32 = node.x;
                let z: i32 = node.z;
                for y in 1..=3 {
                    editor.set_block(Block::CobblestoneWall, x, ground_level + y, z, None, None);
                }

                editor.set_block(Block::WhiteWool, x, ground_level + 4, z, None, None);
                editor.set_block(Block::WhiteWool, x + 1, ground_level + 4, z, None, None);
            }
        } else if element
            .tags()
            .get("area")
            .map_or(false, |v: &String| v == "yes")
        {
            let ProcessedElement::Way(way) = element else {
                return;
            };

            // Handle areas like pedestrian plazas
            let mut surface_block: Block = Block::Stone; // Default block

            // Determine the block type based on the 'surface' tag
            if let Some(surface) = element.tags().get("surface") {
                surface_block = match surface.as_str() {
                    "paving_stones" | "sett" => Block::StoneBricks,
                    "bricks" => Block::Brick,
                    "wood" => Block::OakPlanks,
                    "asphalt" => Block::BlackConcrete,
                    "gravel" | "fine_gravel" => Block::Gravel,
                    "grass" => {
                        if args.winter {
                            Block::SnowBlock
                        } else {
                            Block::GrassBlock
                        }
                    }
                    "dirt" => Block::Dirt,
                    "sand" => Block::Sand,
                    "concrete" => Block::LightGrayConcrete,
                    _ => Block::Stone, // Default to stone for unknown surfaces
                };
            }

            // Fill the area using flood fill or by iterating through the nodes
            let polygon_coords: Vec<(i32, i32)> = way
                .nodes
                .iter()
                .map(|n: &crate::osm_parser::ProcessedNode| (n.x, n.z))
                .collect();
            let filled_area: Vec<(i32, i32)> =
                flood_fill_area(&polygon_coords, args.timeout.as_ref());

            for (x, z) in filled_area {
                editor.set_block(surface_block, x, ground_level, z, None, None);
            }
        } else {
            let mut previous_node: Option<(i32, i32)> = None;
            let mut block_type = Block::BlackConcrete;
            let mut block_range: i32 = 2;
            let mut add_stripe = false;

            // Skip if 'layer' or 'level' is negative in the tags
            if let Some(layer) = element.tags().get("layer") {
                if layer.parse::<i32>().unwrap_or(0) < 0 {
                    return;
                }
            }

            if let Some(level) = element.tags().get("level") {
                if level.parse::<i32>().unwrap_or(0) < 0 {
                    return;
                }
            }

            // Determine block type and range based on highway type
            match highway_type.as_str() {
                "footway" | "pedestrian" => {
                    block_type = Block::GrayConcrete;
                    block_range = 1;
                }
                "path" => {
                    block_type = Block::LightGrayConcrete;
                    block_range = 1;
                }
                "motorway" | "primary" => {
                    block_range = 5;
                    add_stripe = true; // Add stripes for motorways and primary roads
                }
                "track" => {
                    block_range = 1;
                }
                "service" => {
                    block_type = Block::GrayConcrete;
                    block_range = 2;
                }
                _ => {
                    if let Some(lanes) = element.tags().get("lanes") {
                        if lanes == "2" {
                            block_range = 3;
                            add_stripe = true;
                        } else if lanes != "1" {
                            block_range = 4;
                            add_stripe = true;
                        }
                    }
                }
            }

            let ProcessedElement::Way(way) = element else {
                return;
            };

            // Iterate over nodes to create the highway
            for node in &way.nodes {
                if let Some(prev) = previous_node {
                    let (x1, z1) = prev;
                    let x2: i32 = node.x;
                    let z2: i32 = node.z;

                    // Generate the line of coordinates between the two nodes
                    let bresenham_points: Vec<(i32, i32, i32)> =
                        bresenham_line(x1, ground_level, z1, x2, ground_level, z2);

                    // Variables to manage dashed line pattern
                    let mut stripe_length: i32 = 0;
                    let dash_length: i32 = 5; // Length of the solid part of the stripe
                    let gap_length: i32 = 5; // Length of the gap part of the stripe

                    for (x, _, z) in bresenham_points {
                        // Draw the road surface for the entire width
                        for dx in -block_range..=block_range {
                            for dz in -block_range..=block_range {
                                let set_x: i32 = x + dx;
                                let set_z: i32 = z + dz;

                                // Zebra crossing logic
                                if highway_type == "footway"
                                    && element.tags().get("footway")
                                        == Some(&"crossing".to_string())
                                {
                                    let is_horizontal: bool = (x2 - x1).abs() >= (z2 - z1).abs();
                                    if is_horizontal {
                                        if set_x % 2 < 1 {
                                            editor.set_block(
                                                Block::WhiteConcrete,
                                                set_x,
                                                ground_level,
                                                set_z,
                                                Some(&[Block::BlackConcrete]),
                                                None,
                                            );
                                        } else {
                                            editor.set_block(
                                                Block::BlackConcrete,
                                                set_x,
                                                ground_level,
                                                set_z,
                                                None,
                                                None,
                                            );
                                        }
                                    } else if set_z % 2 < 1 {
                                        editor.set_block(
                                            Block::WhiteConcrete,
                                            set_x,
                                            ground_level,
                                            set_z,
                                            Some(&[Block::BlackConcrete]),
                                            None,
                                        );
                                    } else {
                                        editor.set_block(
                                            Block::BlackConcrete,
                                            set_x,
                                            ground_level,
                                            set_z,
                                            None,
                                            None,
                                        );
                                    }
                                } else {
                                    editor.set_block(
                                        block_type,
                                        set_x,
                                        ground_level,
                                        set_z,
                                        None,
                                        Some(&[Block::BlackConcrete, Block::WhiteConcrete]),
                                    );
                                }
                            }
                        }

                        // Add a dashed white line in the middle for larger roads
                        if add_stripe {
                            if stripe_length < dash_length {
                                let stripe_x: i32 = x;
                                let stripe_z: i32 = z;
                                editor.set_block(
                                    Block::WhiteConcrete,
                                    stripe_x,
                                    ground_level,
                                    stripe_z,
                                    Some(&[Block::BlackConcrete]),
                                    None,
                                );
                            }

                            // Increment stripe_length and reset after completing a dash and gap
                            stripe_length += 1;
                            if stripe_length >= dash_length + gap_length {
                                stripe_length = 0;
                            }
                        }
                    }
                }
                previous_node = Some((node.x, node.z));
            }
        }
    }
}

/// Generates a siding using stone brick slabs
pub fn generate_siding(editor: &mut WorldEditor, element: &ProcessedWay, ground_level: i32) {
    let mut previous_node: Option<(i32, i32)> = None;
    let siding_block: Block = Block::StoneBrickSlab;

    for node in &element.nodes {
        let x: i32 = node.x;
        let z: i32 = node.z;

        // Draw the siding using Bresenham's line algorithm between nodes
        if let Some(prev) = previous_node {
            let bresenham_points: Vec<(i32, i32, i32)> =
                bresenham_line(prev.0, ground_level + 1, prev.1, x, ground_level + 1, z);
            for (bx, by, bz) in bresenham_points {
                if !editor.check_for_block(
                    bx,
                    by - 1,
                    bz,
                    None,
                    Some(&[Block::BlackConcrete, Block::WhiteConcrete]),
                ) {
                    editor.set_block(siding_block, bx, by, bz, None, None);
                }
            }
        }

        previous_node = Some((x, z));
    }
}

/// Generates an aeroway
pub fn generate_aeroway(editor: &mut WorldEditor, way: &ProcessedWay, ground_level: i32) {
    let mut previous_node: Option<(i32, i32)> = None;
    let surface_block = Block::LightGrayConcrete;

    for node in &way.nodes {
        if let Some(prev) = previous_node {
            let points = bresenham_line(prev.0, ground_level, prev.1, node.x, ground_level, node.z);

            for (x, y, z) in points {
                for dx in -12..=1 {
                    for dz in -12..=1 {
                        editor.set_block(surface_block, x + dx, y, z + dz, None, None);
                    }
                }
            }
        }
        previous_node = Some((node.x, node.z));
    }
}
