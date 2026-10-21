//! The look of the app around the map: a bundled serif for map lettering and
//! headings (EB Garamond, SIL Open Font License, Latin subset in `assets/fonts`),
//! and a parchment panel theme in the map's colours.

use egui::{Color32, FontData, FontDefinitions, FontFamily, FontId, Stroke, TextStyle};
use std::sync::Arc;

const SERIF: &str = "serif";
const SERIF_ITALIC: &str = "serif-italic";

/// Map lettering and headings.
pub fn serif(size: f32) -> FontId {
    FontId::new(size, FontFamily::Name(SERIF.into()))
}

/// Water and woodland names.
pub fn serif_italic(size: f32) -> FontId {
    FontId::new(size, FontFamily::Name(SERIF_ITALIC.into()))
}

/// Dark brown ink for text on parchment.
pub const INK: Color32 = Color32::from_rgb(52, 38, 26);
/// The ground under and around the map.
pub const MAP_SURROUND: Color32 = Color32::from_rgb(58, 54, 48);

pub fn install(ctx: &egui::Context) {
    let mut fonts = FontDefinitions::default();
    for (name, bytes) in [
        (
            SERIF,
            &include_bytes!("../assets/fonts/EBGaramond-Medium.ttf")[..],
        ),
        (
            SERIF_ITALIC,
            &include_bytes!("../assets/fonts/EBGaramond-MediumItalic.ttf")[..],
        ),
    ] {
        fonts
            .font_data
            .insert(name.into(), Arc::new(FontData::from_static(bytes)));
        // Anything outside the subset falls back to the default fonts.
        let mut chain = vec![name.to_string()];
        chain.extend(fonts.families[&FontFamily::Proportional].iter().cloned());
        fonts.families.insert(FontFamily::Name(name.into()), chain);
    }
    ctx.set_fonts(fonts);

    let parchment = Color32::from_rgb(241, 234, 218);
    let mut v = egui::Visuals::light();
    v.panel_fill = parchment;
    v.window_fill = parchment;
    v.faint_bg_color = Color32::from_rgb(233, 224, 204);
    v.extreme_bg_color = Color32::from_rgb(250, 246, 236);
    v.code_bg_color = Color32::from_rgb(230, 220, 198);
    v.window_stroke = Stroke::new(1.0, Color32::from_rgb(168, 148, 118));
    v.hyperlink_color = Color32::from_rgb(140, 70, 40);
    v.selection.bg_fill = Color32::from_rgb(196, 150, 96);
    v.selection.stroke = Stroke::new(1.0, INK);
    let w = &mut v.widgets;
    w.noninteractive.bg_fill = parchment;
    w.noninteractive.bg_stroke = Stroke::new(1.0, Color32::from_rgb(196, 180, 150));
    w.noninteractive.fg_stroke = Stroke::new(1.0, INK);
    for (wv, bg, edge) in [
        (&mut w.inactive, (228, 216, 192), (178, 158, 126)),
        (&mut w.hovered, (218, 200, 166), (140, 104, 64)),
        (&mut w.active, (204, 180, 140), (110, 76, 40)),
        (&mut w.open, (222, 208, 180), (150, 120, 84)),
    ] {
        wv.bg_fill = Color32::from_rgb(bg.0, bg.1, bg.2);
        wv.weak_bg_fill = wv.bg_fill;
        wv.bg_stroke = Stroke::new(1.0, Color32::from_rgb(edge.0, edge.1, edge.2));
        wv.fg_stroke = Stroke::new(1.5, INK);
    }
    // The map is drawn in one palette, so the panels keep it in dark mode too.
    ctx.all_styles_mut(|s| {
        s.visuals = v.clone();
        s.text_styles.insert(TextStyle::Heading, serif(22.0));
    });
}
