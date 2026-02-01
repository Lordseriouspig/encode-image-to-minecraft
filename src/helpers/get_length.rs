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

use anyhow::{Context, Result};
use mca::RegionReader;
use fastnbt::from_bytes;
use crate::helpers::convert::from_block;
use crate::models::nbt::ChunkNBT;

pub fn get_length(region: RegionReader) -> Result<usize> {
    let chunk = region
        .get_chunk(0, 0)?
        .context("Chunk not found")?;
    let decompressed = chunk.decompress()?;
    let chunk_nbt: ChunkNBT = from_bytes(&decompressed)?;
    let section = &chunk_nbt.sections[0];
    let palette = &section.block_states.palette;
    let data = section.block_states.data.as_ref().context("No data found")?;
    let mut blocks: Vec<String> = Vec::with_capacity(4);

    // shouldnt happen but just in case
    if palette.len() == 1 {
        let name = palette[0].name.clone();
        blocks = vec![name; 4];
    } else {
        let palette_len = palette.len();
        let bits_per_block = (palette_len as f64).log2().ceil().max(4.0) as usize;
        let values_per_long = 64 / bits_per_block;
        let mask = (1u64 << bits_per_block) - 1;

        for i in 0..4 {
            let data_index = i / values_per_long;
            let bit_offset = (i % values_per_long) * bits_per_block;

            let val = ((data[data_index] as u64) >> bit_offset) & mask;
            let palette_index = val as usize;

            let block_name = palette[palette_index].name.clone();
            blocks.push(block_name)
        }
    }

    let mut block_data: Vec<u8> = Vec::new();
    for block in blocks {
        block_data.push(from_block(&block)?);
    }
    let length = u32::from_be_bytes(
        block_data
            .get(0..4)
            .context("Not enough block data for length")?
            .try_into()
            .context("Invalid length bytes")?,
    ) as usize;
    Ok(length)
}