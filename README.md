<!--
 Copyright (C) 2026 Lordseriouspig
 
 This file is part of encode-image-to-minecraft.
 
 encode-image-to-minecraft is free software: you can redistribute it and/or modify
 it under the terms of the GNU General Public License as published by
 the Free Software Foundation, either version 3 of the License, or
 (at your option) any later version.
 
 encode-image-to-minecraft is distributed in the hope that it will be useful,
 but WITHOUT ANY WARRANTY; without even the implied warranty of
 MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 GNU General Public License for more details.
 
 You should have received a copy of the GNU General Public License
 along with encode-image-to-minecraft.  If not, see <https://www.gnu.org/licenses/>.
-->

# Encode Image to Minecraft
A small little project to encode and decode an image (or any kind of file) in to a minecraft region (.mca) file! It works by converting the file to bytes and assigning each byte a minecraft block, and then creating a region file using those blocks.

## Usage
You can choose to install the binary from the GitHub releases, or you can choose to build it yourself from source. You'll need to have rust installed if you want to build it yourself.
Once you've got the binary, you can run it from the command line. The program comes with two commands.

Encode will encode any input file into a minecraft region file. You can then drag that into ``%appdata%\.minecraft\saves\<your minecraft world>\region``
```
encode-image-to-minecraft encode <input_file>.whatever <output_file>.mca
```

Decode will decode a minecraft region file back to the original file. Note that if you have placed the region file into the minecraft world, minecraft will do whatever it does to it and you may not be able to decode the file. I have tried and failed to fix this lmao. To prevent this, keep a copy of the file before running minecraft.
```
encode-image-to-minecraft decode <input_file>.mca <output_file>.whatever
```