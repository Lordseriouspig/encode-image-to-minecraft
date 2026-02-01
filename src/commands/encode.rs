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

use clap::Parser;
use std::{fs::File, io::{Read, Write}};
use crate::helpers::write::write_region_buf;

#[derive(Parser, Debug)]
pub struct EncodeCmd {
    /// The input image file
    #[clap(value_parser)]
    pub input: String,
    /// The output mca file
    #[clap(value_parser)]
    pub output: String,
}
impl EncodeCmd {
    pub fn execute(&self) {
        // Get Image
        let mut file = File::open(&self.input).expect("Failed to open input image file");
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).expect("Failed to read input image file");

        // Prepend length info
        let length = buffer.len() as u32;
        let length_bytes: [u8; 4] = length.to_be_bytes();
        let mut buffer_with_length = Vec::with_capacity(4 + buffer.len());
        buffer_with_length.extend_from_slice(&length_bytes);
        buffer_with_length.extend_from_slice(&buffer);
        
        // Do the stuff
        let data = write_region_buf(buffer_with_length).expect("Failed to write region buffer");

        // Write to the file
        File::create(&self.output)
            .expect("Failed to create output mca file")
            .write_all(&data)
            .expect("Failed to write to output mca file");
    }
}