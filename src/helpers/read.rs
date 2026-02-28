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
use crate::helpers::{chunk_pos::index_to_xy, convert::from_block, get_length::get_length};
use crate::models::nbt::*;
use anyhow::{ensure, Context, Result};
use fastnbt::from_bytes;
use mca::RegionReader;

pub fn read_region_buf(data: &[u8]) -> Result<Vec<u8>> {
    let region = RegionReader::new(&data)?;
    let size = get_length(region.clone())?;
    let mut block_data: Vec<u8> = Vec::new();

    // determine number of chunks
    let chunk_num = (size + CH_BLOCKS - 1) / CH_BLOCKS;
    // no clue how this could happen but idk
    ensure!(
        chunk_num <= REGION_CH,
        "Your file is too big! Got {}B, max 100,663,296B (around 100MB)",
        size
    );
    for ch in 0..chunk_num {
        let (chunk_x, chunk_z) = index_to_xy(ch);

        let chunk = region
            .get_chunk(chunk_x, chunk_z)?
            .context("Chunk not found")?;
        let decompressed = chunk.decompress()?;
        let chunk_nbt: ChunkNBT = from_bytes(&decompressed)?;

        let section_num = chunk_nbt.sections.len();
        ensure!(
            section_num <= 24,
            "Too many sections in chunk {}. Got {}",
            ch,
            section_num
        );

        for sec in 0..section_num {
            let section = &chunk_nbt.sections[sec];
            let palette = &section.block_states.palette;
            let data = section
                .block_states
                .data
                .as_ref()
                .context("No data found")?;
            let mut blocks: Vec<String> = Vec::with_capacity(SC_BLOCKS);
            // shouldnt happen but just in case
            if palette.len() == 1 {
                let name = palette[0].name.clone();
                blocks = vec![name; SC_BLOCKS];
            } else {
                let palette_len = palette.len();
                let bits_per_block = (palette_len as f64).log2().ceil().max(4.0) as usize;
                let values_per_long = 64 / bits_per_block;
                let mask = (1u64 << bits_per_block) - 1;

                for blk in 0..SC_BLOCKS {
                    let data_index = blk / values_per_long;
                    let bit_offset = (blk % values_per_long) * bits_per_block;

                    let val = ((data[data_index] as u64) >> bit_offset) & mask;
                    let palette_index = val as usize;

                    let block_name = palette[palette_index].name.clone();
                    blocks.push(block_name);
                }
            }

            for (i, block) in blocks.iter().enumerate() {
                if ch == 0 && sec == 0 && i < 4 {
                    continue;
                }
                if block_data.len() >= size {
                    break;
                }
                ensure!(
                    block != "minecraft:air",
                    "Reading past data bounds at address {}",
                    block_data.len()
                );
                block_data.push(from_block(&block)?);
            }
        }
    }

    Ok(block_data)
}
