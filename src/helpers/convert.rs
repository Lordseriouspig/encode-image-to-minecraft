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
use std::collections::HashMap;

pub fn to_block(value: u8) -> Result<&'static str> {
    let block_map: HashMap<u8, &'static str> = crate::models::blocks::build_block_map();

    let block = block_map
        .get(&value)
        .context(format!("Byte not found in block map: {}", value))?;

    Ok(*block)
}

pub fn from_block(block: &str) -> Result<u8> {
    let block_map: HashMap<u8, &'static str> = crate::models::blocks::build_block_map();

    let id = block_map
        .iter()
        .find_map(|(key, &val)| if val == block { Some(*key) } else { None })
        .context(format!("Block not found in block map: {}", block))?;
    Ok(id)
}
