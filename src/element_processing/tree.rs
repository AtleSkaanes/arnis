use crate::block_definitions::*;
use crate::world_editor::WorldEditor;

/// Helper function to set blocks in a circular pattern around a central point.
fn round1(editor: &mut WorldEditor, material: Block, x: i32, y: i32, z: i32) {
    editor.set_block(material, x - 2, y, z, None, None);
    editor.set_block(material, x + 2, y, z, None, None);
    editor.set_block(material, x, y, z - 2, None, None);
    editor.set_block(material, x, y, z + 2, None, None);
    editor.set_block(material, x - 1, y, z - 1, None, None);
    editor.set_block(material, x + 1, y, z + 1, None, None);
    editor.set_block(material, x + 1, y, z - 1, None, None);
    editor.set_block(material, x - 1, y, z + 1, None, None);
}

/// Helper function to set blocks in a wider circular pattern.
fn round2(editor: &mut WorldEditor, material: Block, x: i32, y: i32, z: i32) {
    editor.set_block(material, x + 3, y, z, None, None);
    editor.set_block(material, x + 2, y, z - 1, None, None);
    editor.set_block(material, x + 2, y, z + 1, None, None);
    editor.set_block(material, x + 1, y, z - 2, None, None);
    editor.set_block(material, x + 1, y, z + 2, None, None);
    editor.set_block(material, x - 3, y, z, None, None);
    editor.set_block(material, x - 2, y, z - 1, None, None);
    editor.set_block(material, x - 2, y, z + 1, None, None);
    editor.set_block(material, x - 1, y, z + 2, None, None);
    editor.set_block(material, x - 1, y, z - 2, None, None);
    editor.set_block(material, x, y, z - 3, None, None);
    editor.set_block(material, x, y, z + 3, None, None);
}

/// Helper function to set blocks in a more scattered circular pattern.
fn round3(editor: &mut WorldEditor, material: Block, x: i32, y: i32, z: i32) {
    editor.set_block(material, x + 3, y, z - 1, None, None);
    editor.set_block(material, x + 3, y, z + 1, None, None);
    editor.set_block(material, x + 2, y, z - 2, None, None);
    editor.set_block(material, x + 2, y, z + 2, None, None);
    editor.set_block(material, x + 1, y, z - 3, None, None);
    editor.set_block(material, x + 1, y, z + 3, None, None);
    editor.set_block(material, x - 3, y, z - 1, None, None);
    editor.set_block(material, x - 3, y, z + 1, None, None);
    editor.set_block(material, x - 2, y, z - 2, None, None);
    editor.set_block(material, x - 2, y, z + 2, None, None);
    editor.set_block(material, x - 1, y, z + 3, None, None);
    editor.set_block(material, x - 1, y, z - 3, None, None);
}

/// Function to create different types of trees.
pub fn create_tree(editor: &mut WorldEditor, x: i32, y: i32, z: i32, typetree: u8, snow: bool) {
    let mut blacklist: Vec<Block> = Vec::new();
    blacklist.extend(building_corner_variations());
    blacklist.extend(building_wall_variations());
    blacklist.extend(building_floor_variations());
    blacklist.push(Block::Water);

    if editor.check_for_block(x, y - 1, z, None, Some(&blacklist)) {
        return;
    }

    match typetree {
        1 => {
            // Oak tree
            editor.fill_blocks(Block::OakLog, x, y, z, x, y + 8, z, None, None);
            editor.fill_blocks(
                Block::OakLeaves,
                x - 1,
                y + 3,
                z,
                x - 1,
                y + 9,
                z,
                None,
                None,
            );
            editor.fill_blocks(
                Block::OakLeaves,
                x + 1,
                y + 3,
                z,
                x + 1,
                y + 9,
                z,
                None,
                None,
            );
            editor.fill_blocks(
                Block::OakLeaves,
                x,
                y + 3,
                z - 1,
                x,
                y + 9,
                z - 1,
                None,
                None,
            );
            editor.fill_blocks(
                Block::OakLeaves,
                x,
                y + 3,
                z + 1,
                x,
                y + 9,
                z + 1,
                None,
                None,
            );
            editor.fill_blocks(Block::OakLeaves, x, y + 9, z, x, y + 10, z, None, None);
            round1(editor, Block::OakLeaves, x, y + 8, z);
            round1(editor, Block::OakLeaves, x, y + 7, z);
            round1(editor, Block::OakLeaves, x, y + 6, z);
            round1(editor, Block::OakLeaves, x, y + 5, z);
            round1(editor, Block::OakLeaves, x, y + 4, z);
            round1(editor, Block::OakLeaves, x, y + 3, z);
            round2(editor, Block::OakLeaves, x, y + 7, z);
            round2(editor, Block::OakLeaves, x, y + 6, z);
            round2(editor, Block::OakLeaves, x, y + 5, z);
            round2(editor, Block::OakLeaves, x, y + 4, z);
            round3(editor, Block::OakLeaves, x, y + 6, z);
            round3(editor, Block::OakLeaves, x, y + 5, z);

            if snow {
                editor.set_block(Block::SnowLayer, x, y + 11, z, None, None);
                editor.set_block(Block::SnowLayer, x + 1, y + 10, z, None, None);
                editor.set_block(Block::SnowLayer, x - 1, y + 10, z, None, None);
                editor.set_block(Block::SnowLayer, x, y + 10, z - 1, None, None);
                editor.set_block(Block::SnowLayer, x, y + 10, z + 1, None, None);
                round1(editor, Block::SnowLayer, x, y + 9, z);
                round1(editor, Block::SnowLayer, x, y + 8, z);
                round1(editor, Block::SnowLayer, x, y + 7, z);
                round1(editor, Block::SnowLayer, x, y + 6, z);
                round2(editor, Block::SnowLayer, x, y + 8, z);
                round2(editor, Block::SnowLayer, x, y + 7, z);
                round2(editor, Block::SnowLayer, x, y + 6, z);
                round2(editor, Block::SnowLayer, x, y + 5, z);
                round3(editor, Block::SnowLayer, x, y + 7, z);
                round3(editor, Block::SnowLayer, x, y + 6, z);
            }
        }
        2 => {
            // Spruce tree
            editor.fill_blocks(Block::SpruceLog, x, y, z, x, y + 9, z, None, None);
            editor.fill_blocks(
                Block::BirchLeaves,
                x - 1,
                y + 3,
                z,
                x - 1,
                y + 10,
                z,
                None,
                None,
            );
            editor.fill_blocks(
                Block::BirchLeaves,
                x + 1,
                y + 3,
                z,
                x + 1,
                y + 10,
                z,
                None,
                None,
            );
            editor.fill_blocks(
                Block::BirchLeaves,
                x,
                y + 3,
                z - 1,
                x,
                y + 10,
                z - 1,
                None,
                None,
            );
            editor.fill_blocks(
                Block::BirchLeaves,
                x,
                y + 3,
                z + 1,
                x,
                y + 10,
                z + 1,
                None,
                None,
            );
            editor.set_block(Block::BirchLeaves, x, y + 10, z, None, None);
            round1(editor, Block::BirchLeaves, x, y + 9, z);
            round1(editor, Block::BirchLeaves, x, y + 7, z);
            round1(editor, Block::BirchLeaves, x, y + 6, z);
            round1(editor, Block::BirchLeaves, x, y + 4, z);
            round1(editor, Block::BirchLeaves, x, y + 3, z);
            round2(editor, Block::BirchLeaves, x, y + 6, z);
            round2(editor, Block::BirchLeaves, x, y + 3, z);

            if snow {
                editor.set_block(Block::SnowLayer, x, y + 11, z, None, None);
                editor.set_block(Block::SnowLayer, x + 1, y + 11, z, None, None);
                editor.set_block(Block::SnowLayer, x - 1, y + 11, z, None, None);
                editor.set_block(Block::SnowLayer, x, y + 11, z - 1, None, None);
                editor.set_block(Block::SnowLayer, x, y + 11, z + 1, None, None);
                round1(editor, Block::SnowLayer, x, y + 10, z);
                round1(editor, Block::SnowLayer, x, y + 8, z);
                round1(editor, Block::SnowLayer, x, y + 7, z);
                round1(editor, Block::SnowLayer, x, y + 5, z);
                round1(editor, Block::SnowLayer, x, y + 4, z);
                round2(editor, Block::SnowLayer, x, y + 7, z);
                round2(editor, Block::SnowLayer, x, y + 4, z);
            }
        }
        3 => {
            // Birch tree
            editor.fill_blocks(Block::BirchLog, x, y, z, x, y + 6, z, None, None);
            editor.fill_blocks(
                Block::BirchLeaves,
                x - 1,
                y + 2,
                z,
                x - 1,
                y + 7,
                z,
                None,
                None,
            );
            editor.fill_blocks(
                Block::BirchLeaves,
                x + 1,
                y + 2,
                z,
                x + 1,
                y + 7,
                z,
                None,
                None,
            );
            editor.fill_blocks(
                Block::BirchLeaves,
                x,
                y + 2,
                z - 1,
                x,
                y + 7,
                z - 1,
                None,
                None,
            );
            editor.fill_blocks(
                Block::BirchLeaves,
                x,
                y + 2,
                z + 1,
                x,
                y + 7,
                z + 1,
                None,
                None,
            );
            editor.fill_blocks(Block::BirchLeaves, x, y + 7, z, x, y + 8, z, None, None);
            round1(editor, Block::BirchLeaves, x, y + 6, z);
            round1(editor, Block::BirchLeaves, x, y + 5, z);
            round1(editor, Block::BirchLeaves, x, y + 4, z);
            round1(editor, Block::BirchLeaves, x, y + 3, z);
            round1(editor, Block::BirchLeaves, x, y + 2, z);
            round2(editor, Block::BirchLeaves, x, y + 2, z);
            round2(editor, Block::BirchLeaves, x, y + 3, z);
            round2(editor, Block::BirchLeaves, x, y + 4, z);

            if snow {
                editor.set_block(Block::SnowLayer, x, y + 9, z, None, None);
                editor.set_block(Block::SnowLayer, x + 1, y + 8, z, None, None);
                editor.set_block(Block::SnowLayer, x - 1, y + 8, z, None, None);
                editor.set_block(Block::SnowLayer, x, y + 8, z - 1, None, None);
                editor.set_block(Block::SnowLayer, x, y + 8, z + 1, None, None);
                round1(editor, Block::SnowLayer, x, y + 7, z);
                round1(editor, Block::SnowLayer, x, y + 6, z);
                round1(editor, Block::SnowLayer, x, y + 5, z);
                round1(editor, Block::SnowLayer, x, y + 4, z);
                round1(editor, Block::SnowLayer, x, y + 3, z);
                round2(editor, Block::SnowLayer, x, y + 3, z);
                round2(editor, Block::SnowLayer, x, y + 4, z);
                round2(editor, Block::SnowLayer, x, y + 5, z);
            }
        }
        _ => {} // Do nothing if typetree is not recognized
    }
}
