/*
Eyes of Hell: a game where you walk around and look at things
Copyright (C) 2025 UnrelatedString

This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.
*/

mod meat;
use meat::run;
use three_d::WindowSettings;

// what if. Mystic Eyes of Depth Perception

#[cfg(not(target_family = "wasm"))]
#[tokio::main]
async fn main() {
    run(WindowSettings {
        initial_size: Some((1280, 720)),
        ..Default::default()
    }).await.unwrap();
}
