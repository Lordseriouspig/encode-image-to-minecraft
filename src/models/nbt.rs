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

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct McaChunk {
    pub x: u8,
    pub z: u8,
    pub nbt: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PaletteEntry {
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BlockStates {
    pub palette: Vec<PaletteEntry>,
    pub data: Option<Vec<i64>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Section {
    #[serde(rename = "Y")]
    pub y: i8,
    pub block_states: BlockStates,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChunkNBT {
    #[serde(rename = "DataVersion")]
    pub data_version: i32,

    #[serde(rename = "xPos")]
    pub x_pos: i32,

    #[serde(rename = "zPos")]
    pub z_pos: i32,

    #[serde(rename = "yPos")]
    pub y_pos: i32,

    #[serde(rename = "Status")]
    pub status: String,

    pub sections: Vec<Section>,
}