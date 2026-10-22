//! The scale framework of the real-scale view (R9): how far the map zooms, how
//! many metres a pixel covers, and when map symbols give way to the landscape
//! drawn at true size. Every size is in metres and converted through `cell_km`,
//! so nothing depends on the cell size.

use egui::{Align2, Color32, Painter, Pos2, Rect, Stroke};

/// The deepest zoom: metres per screen point (about 0.5 m per physical pixel
/// on a high-density screen).
pub const MIN_M_PER_PX: f32 = 1.0;
/// The deepest zoom before R9, in points per cell. Symbols (icons, smoke,
/// rings, route lines) stop growing here so they stay symbols.
pub const SYMBOL_ZOOM: f32 = 64.0;
/// Zoom (points per cell) by which map symbols of roads and rivers have faded
/// out in favour of the true-width landscape, unless kept on.
pub const SYMBOL_GONE: f32 = 256.0;

/// The deepest zoom in points per cell for a map of this cell size.
pub fn max_zoom(cell_km: f32) -> f32 {
    (cell_km * 1000.0 / MIN_M_PER_PX).max(SYMBOL_ZOOM)
}

/// Metres covered by one screen point at this zoom.
pub fn m_per_px(zoom: f32, cell_km: f32) -> f32 {
    cell_km * 1000.0 / zoom
}

/// Zoom used to size symbols: the real zoom, capped at `SYMBOL_ZOOM`.
pub fn symbol_zoom(zoom: f32) -> f32 {
    zoom.min(SYMBOL_ZOOM)
}

/// Opacity of road and river symbols: full out to the old deepest zoom, then
/// fading (in log zoom) until the true-width landscape has taken over; always
/// full when the "map symbols at full zoom" layer is on.
pub fn symbol_alpha(zoom: f32, keep: bool) -> f32 {
    if keep || zoom <= SYMBOL_ZOOM {
        return 1.0;
    }
    let t = (zoom / SYMBOL_ZOOM).ln() / (SYMBOL_GONE / SYMBOL_ZOOM).ln();
    1.0 - t.clamp(0.0, 1.0)
}

/// A scale bar in the map's lower-left corner: a round length of about 120
/// points, in metres or kilometres.
pub fn draw_bar(painter: &Painter, rect: Rect, zoom: f32, cell_km: f32) {
    let mpp = m_per_px(zoom, cell_km);
    let target = 120.0 * mpp;
    let pow = 10f32.powf(target.log10().floor());
    let len_m = [1.0, 2.0, 5.0, 10.0]
        .iter()
        .map(|k| k * pow)
        .filter(|&l| l <= target)
        .fold(pow, f32::max);
    let len_px = len_m / mpp;
    let base = Pos2::new(rect.left() + 16.0, rect.bottom() - 18.0);
    let ink = crate::style::INK;
    let halo = Color32::from_rgba_unmultiplied(250, 247, 240, 210);
    let bar = Rect::from_min_max(base - egui::vec2(0.0, 5.0), base + egui::vec2(len_px, 0.0));
    painter.rect_filled(bar.expand(2.0), 2.0, halo);
    // Alternating halves, the old way of drawing a scale.
    let half = Rect::from_min_max(bar.min, Pos2::new(bar.center().x, bar.max.y));
    painter.rect_filled(half, 0.0, ink);
    painter.rect_stroke(bar, 0.0, Stroke::new(1.0, ink), egui::StrokeKind::Inside);
    let text = if len_m >= 1000.0 {
        format!("{} km", len_m / 1000.0)
    } else {
        format!("{len_m} m")
    };
    let label = format!("{text}   ({mpp:.1} m per point)");
    let at = Pos2::new(base.x, bar.top() - 4.0);
    let font = crate::style::serif(13.0);
    for (dx, dy) in [(-1.0, 0.0), (1.0, 0.0), (0.0, -1.0), (0.0, 1.0)] {
        painter.text(
            at + egui::vec2(dx, dy),
            Align2::LEFT_BOTTOM,
            &label,
            font.clone(),
            halo,
        );
    }
    painter.text(at, Align2::LEFT_BOTTOM, label, font, ink);
}
