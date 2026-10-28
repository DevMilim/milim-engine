use std::{collections::BTreeMap, sync::Arc};

use engine_core::{
    core::Handler,
    math::{Anchor, Color, Rect, Vector2},
};

use crate::{FontAsset, ImageAsset};

#[derive(Debug, Clone)]
pub enum DrawCommand {
    Sprite {
        position: Vector2,
        image: Handler<ImageAsset>,
        anchor: Anchor,
        source: Option<Rect>,
        flip_v: bool,
        flip_h: bool,
        rotation: f32,
    },
    Rect {
        color: Color,
        rect: Rect,
        anchor: Anchor,
    },
    Line {
        start: Vector2,
        end: Vector2,
        color: Color,
        thickness: f32,
    },
    Text {
        text: Arc<str>,
        font: Handler<FontAsset>,
        position: Vector2,
        color: Color,
        size_px: u32,
        anchor: Anchor,
    },
}

pub struct RenderQueue {
    pub queue: BTreeMap<u32, Vec<DrawCommand>>,
    pub camera: Vector2,
}

impl Default for RenderQueue {
    fn default() -> Self {
        Self {
            queue: BTreeMap::default(),
            camera: Vector2::ZERO,
        }
    }
}

impl RenderQueue {
    pub fn new(camera: Vector2) -> Self {
        Self {
            queue: BTreeMap::new(),
            camera,
        }
    }

    pub fn push(&mut self, layer: u32, cmd: DrawCommand) {
        self.queue.entry(layer).or_default().push(cmd);
    }

    pub fn command_count(&self) -> usize {
        self.queue.values().map(Vec::len).sum()
    }

    pub fn sort(&mut self) {
        for layer in self.queue.values_mut() {
            layer.sort_unstable_by_key(|cmd| match cmd {
                DrawCommand::Sprite { image, .. } => (1, image.id),
                _ => (0, 0),
            });
        }
    }

    pub fn clear(&mut self) {
        for layer in self.queue.values_mut() {
            layer.clear();
        }
    }
    pub fn draw(&mut self, z_index: u32, command: DrawCommand) {
        self.push(z_index, command);
    }

    pub fn draw_rect(&mut self, rect: Rect, color: Color, anchor: Anchor, z_index: u32) {
        self.push(
            z_index,
            DrawCommand::Rect {
                color,
                rect,
                anchor,
            },
        );
    }

    pub fn draw_sprite(
        &mut self,
        position: Vector2,
        texture: Handler<ImageAsset>,
        anchor: Anchor,
        source: Option<Rect>,
        flip_v: bool,
        flip_h: bool,
        z_index: u32,
        rotation: f32,
    ) {
        self.push(
            z_index,
            DrawCommand::Sprite {
                position,
                image: texture,
                anchor,
                source,
                flip_v,
                flip_h,
                rotation,
            },
        )
    }
    pub fn draw_line(
        &mut self,
        start: Vector2,
        end: Vector2,
        color: Color,
        thickness: f32,
        z_index: u32,
    ) {
        self.push(
            z_index,
            DrawCommand::Line {
                start,
                end,
                color,
                thickness,
            },
        );
    }
    pub fn camera_mut(&mut self) -> &mut Vector2 {
        &mut self.camera
    }
    pub fn draw_text(
        &mut self,
        text: Arc<str>,
        font: Handler<FontAsset>,
        position: Vector2,
        color: Color,
        anchor: Anchor,
        size_px: u32,
        z_index: u32,
    ) {
        self.push(
            z_index,
            DrawCommand::Text {
                text,
                font,
                position,
                color,
                anchor,
                size_px,
            },
        );
    }
}
