//! Screen <-> cell mapping, in one place. Every layer and panel uses this.

use egui::{Pos2, Rect, Vec2};

#[derive(Clone, Copy, Debug)]
pub struct Camera {
    /// Map-space point (in cells) at the centre of the view.
    pub center: Vec2,
    /// Screen pixels per cell.
    pub zoom: f32,
}

impl Camera {
    pub fn fit(w: u32, h: u32, view: Rect) -> Self {
        let zoom = (view.width() / w as f32)
            .min(view.height() / h as f32)
            .max(0.5);
        Self {
            center: Vec2::new(w as f32 / 2.0, h as f32 / 2.0),
            zoom,
        }
    }

    pub fn to_screen(&self, view: Rect, map: Vec2) -> Pos2 {
        view.center() + (map - self.center) * self.zoom
    }

    pub fn to_map(&self, view: Rect, screen: Pos2) -> Vec2 {
        (screen - view.center()) / self.zoom + self.center
    }

    /// Screen position of a cell's centre.
    pub fn cell_center(&self, view: Rect, w: u32, cell: usize) -> Pos2 {
        let (x, y) = ((cell as u32 % w) as f32, (cell as u32 / w) as f32);
        self.to_screen(view, Vec2::new(x + 0.5, y + 0.5))
    }

    /// The cell under a screen position, if on the map.
    pub fn pick(&self, view: Rect, w: u32, h: u32, screen: Pos2) -> Option<usize> {
        let m = self.to_map(view, screen);
        if m.x < 0.0 || m.y < 0.0 || m.x >= w as f32 || m.y >= h as f32 {
            return None;
        }
        Some(m.y as usize * w as usize + m.x as usize)
    }

    /// Screen rectangle covered by the whole map.
    pub fn map_rect(&self, view: Rect, w: u32, h: u32) -> Rect {
        Rect::from_min_max(
            self.to_screen(view, Vec2::ZERO),
            self.to_screen(view, Vec2::new(w as f32, h as f32)),
        )
    }

    /// Zoom by `factor` about a screen point, no further in than `max` (points
    /// per cell, from `scale::max_zoom`).
    pub fn zoom_at(&mut self, view: Rect, screen: Pos2, factor: f32, max: f32) {
        let before = self.to_map(view, screen);
        self.zoom = (self.zoom * factor).clamp(0.5, max);
        let after = self.to_map(view, screen);
        self.center += before - after;
    }
}
