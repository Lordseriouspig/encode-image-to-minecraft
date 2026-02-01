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

pub fn index_to_xy(n: usize) -> (usize, usize) {
    // quick helper to determine chunk position
    let mut count = 0;

    for s in 0.. {
        let diagonal_len = s + 1;

        if count + diagonal_len > n {
            let offset = n - count;

            if s % 2 == 0 {
                let x = s - offset;
                let y = offset;
                return (x, y);
            } else {
                let x = offset;
                let y = s - offset;
                return (x, y);
            }
        }

        count += diagonal_len;
    }

    unreachable!()
}
