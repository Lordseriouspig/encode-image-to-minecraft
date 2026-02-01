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

use clap::Args;

#[derive(Args, Debug)]
pub struct DecodeCmd {
    /// The input mca file
    #[clap(value_parser)]
    pub input: String,
    /// The output image file
    #[clap(value_parser)]
    pub output: String,
}

impl DecodeCmd {
    pub fn execute(&self) {
        // Get world

        // Get image from the world

        // Decode image

        // Save the image to output
    }
}