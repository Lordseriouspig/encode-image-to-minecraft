// Copyright (C) 2026 Lordseriouspig
//
// This file is part of encode-image-to-minecraft.
//
// encode-image-to-minecraft is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// encode-image-to-minecraft is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with encode-image-to-minecraft.  If not, see <https://www.gnu.org/licenses/>.

use crate::constants::region::*;
use crate::helpers::{chunk_pos::index_to_xy, convert::to_block};
use crate::models::nbt::*;
use anyhow::{ensure, Context, Result};
use fastnbt::to_bytes;
use mca::RegionWriter;
use std::collections::HashMap;

pub fn write_region_buf(buffer: Vec<u8>) -> Result<Vec<u8>> {
    let mut data: Vec<McaChunk> = Vec::new(); // array of chunk data
    let mut blocks: Vec<&'static str> = Vec::with_capacity(buffer.len()); // array of block names
    let mut writer = RegionWriter::new(); // mca region writer
    let mut buf: Vec<u8> = Vec::new(); // final region buffer

    // find the blocks
    for byte in buffer {
        let block = to_block(byte)?;
        blocks.push(block);
    }

    // determine number of chunks
    let chunk_num = (blocks.len() + CH_BLOCKS - 1) / CH_BLOCKS;
    ensure!(
        chunk_num <= REGION_CH,
        "Your file is too big! Got {}B, max 100,663,296B (around 100MB)",
        blocks.len()
    );
    let mut chunks: Vec<ChunkNBT> = Vec::new(); // array of chunks

    // build all the chunks
    for ch in 0..chunk_num {
        let ch_start = ch * CH_BLOCKS;
        let ch_end = ((ch + 1) * CH_BLOCKS).min(blocks.len());
        let ch_blocks = blocks[ch_start..ch_end].to_vec();

        let mut sections: Vec<Section> = Vec::new();
        let section_num = (ch_blocks.len() + SC_BLOCKS - 1) / SC_BLOCKS;

        ensure!(
            section_num <= 24,
            "Too many sections in chunk {}. Got {}",
            ch,
            section_num
        );

        for sec in 0..section_num {
            let sc_start = sec * SC_BLOCKS;
            let sc_end = ((sec + 1) * SC_BLOCKS).min(ch_blocks.len());
            let mut sc_blocks = ch_blocks[sc_start..sc_end].to_vec();
            sc_blocks.resize(SC_BLOCKS, "minecraft:air");
            ensure!(
                sc_blocks.len() == SC_BLOCKS,
                "Section blocks length is not {} for some reason",
                SC_BLOCKS
            );

            // get the palette
            let mut palette: Vec<PaletteEntry> = Vec::new();
            let mut palette_map: HashMap<String, usize> = HashMap::new();

            for &block in &sc_blocks {
                if !palette_map.contains_key(block) {
                    palette_map.insert(block.to_string(), palette.len());
                    palette.push(PaletteEntry {
                        name: block.to_string(),
                    });
                }
            }

            // packs the data with some wizardry
            let palette_len = palette.len();
            let bits_per_block = if palette_len <= 1 {
                0
            } else {
                (palette_len as f64).log2().ceil().max(4.0) as usize
            };

            let data_i64: Option<Vec<i64>> = if palette_len <= 1 {
                None
            } else {
                let values_per_long = 64 / bits_per_block;
                let storage_len = (SC_BLOCKS + values_per_long - 1) / values_per_long;
                let mut block_data: Vec<u64> = vec![0; storage_len];

                for i in 0..sc_blocks.len() {
                    let block = sc_blocks[i];
                    let index = *palette_map
                        .get(block)
                        .context("Palette index missing for block")?
                        as u64;

                    let data_index = i / values_per_long;
                    let bit_offset = (i % values_per_long) * bits_per_block;
                    block_data[data_index] |= index << bit_offset;
                }

                Some(block_data.into_iter().map(|x| x as i64).collect())
            };

            // build a section
            let section = Section {
                y: sec as i8 - 4,
                block_states: BlockStates {
                    palette,
                    data: data_i64,
                },
            };

            sections.push(section)
        }
        // determine x and y pos of the chunk
        let (x, y) = index_to_xy(ch);

        let chunk = ChunkNBT {
            data_version: DATA_VERSION,
            x_pos: x as i32,
            z_pos: y as i32,
            y_pos: 0,
            status: "full".to_string(),
            sections,
        };
        chunks.push(chunk);
    }

    for chunk in chunks {
        let nbt_bytes = to_bytes(&chunk)?;
        data.push(McaChunk {
            x: chunk.x_pos as u8,
            z: chunk.z_pos as u8,
            nbt: nbt_bytes,
        })
    }

    // aaaand that should work, in theory (A GAME THEORY!!!!!!)
    // get it, because minecraft
    // no one is going to read this, what am i saying
    // please send help

    for chunk in data {
        writer.push_chunk(&chunk.nbt, (chunk.x, chunk.z))?;
    }
    writer.write(&mut buf)?;

    Ok(buf)
}
