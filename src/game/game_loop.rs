// Anima Engine. The quirky game engine
// Copyright (C) 2016  Dragoș Tiselice
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use super::game::Game;

use time;

/// A `struct` that helps you create a very simple game loop.
///
/// # Examples
///
/// ```no-run
/// pub struct MyGame;
///
/// impl Game for MyGame {
///     fn update(&self, dt: Duration) {
///         // Update game state.
///         // Return `false` when game needs to stop.
///     }
/// }
///
/// GameLoop::new(MyGame).run();
/// ```
pub struct GameLoop<T: Game> {
    pub game: T
}

impl<T: Game> GameLoop<T> {
    /// Creates a `GameLoop`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use anima_engine::time::Duration;
    /// # use anima_engine::game::Game;
    /// # use anima_engine::game::GameLoop;
    /// pub struct MyGame;
    ///
    /// impl Game for MyGame {
    ///     fn update(&self, dt: Duration) {
    ///         // Update game state.
    ///         // Return `false` when game needs to stop.
    ///     }
    /// }
    ///
    /// GameLoop::new(MyGame);
    /// ```
    pub fn new(game: T) -> GameLoop<T> {
        GameLoop { game: game }
    }

    /// Runs `GameLoop`'s `Game` in a loop while feeding the time between frames to the `Game`'s
    /// `update`.
    ///
    /// # Examples
    ///
    /// ```no-run
    /// pub struct MyGame;
    ///
    /// impl Game for MyGame {
    ///     fn update(&self, dt: Duration) {
    ///         // Update game state.
    ///         // Return `false` when game needs to stop.
    ///     }
    /// }
    ///
    /// GameLoop::new(MyGame).run();
    /// ```
    pub fn run(&self) {
        let mut last = time::get_time();

        loop {
            let now = time::get_time();

            if !self.game.update(now - last) {
                break;
            }

            last = now;
        }
    }
}
