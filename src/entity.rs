
use bevy::prelude::Bundle;
use bevy_ascii_terminal::BLACK;
use bevy_ascii_terminal::TileColor;

use crate::render::*;
use crate::movement::*;

#[derive(Debug, Bundle)]
pub struct MovingEntityBundle {
    pub renderable: Renderable,
    pub position: Position,
    pub movement: Movement,
}

impl MovingEntityBundle {
    pub fn new(fg_color: TileColor, glyph: char) -> Self {
        Self {
            renderable: Renderable {
                fg_color,
                bg_color: BLACK,
                glyph,
            },
            position: Position::default(),
            movement: Movement::default(),
        }
    }
}
