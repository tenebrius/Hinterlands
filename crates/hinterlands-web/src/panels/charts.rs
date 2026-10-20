//! Charts: rank-size (log-log, with a Zipf reference line) and population over
//! time, drawn directly with the painter.

use super::Panel;
use crate::view::{Msg, ViewState};
use egui::{Color32, Pos2, Rect, Sense, Stroke, Vec2};

pub struct Charts;

/// Draw series of points into a framed box, mapping data bounds to the box.
fn chart(
    ui: &mut egui::Ui,
    height: f32,
    series: &[(&[[f32; 2]], Color32, bool)],
    labels: (&str, &str),
) {
    let (rect, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), height), Sense::hover());
    let painter = ui.painter_at(rect);
    painter.rect_stroke(
        rect,
        2.0,
        Stroke::new(1.0, ui.visuals().weak_text_color()),
        egui::StrokeKind::Inside,
    );
    let all = series.iter().flat_map(|s| s.0.iter());
    let (mut lo, mut hi) = ([f32::MAX; 2], [f32::MIN; 2]);
    for p in all {
        for k in 0..2 {
            lo[k] = lo[k].min(p[k]);
            hi[k] = hi[k].max(p[k]);
        }
    }
    if lo[0] >= hi[0] || lo[1] >= hi[1] {
        return;
    }
    let inner = rect.shrink(6.0);
    let map = |p: [f32; 2]| {
        Pos2::new(
            inner.left() + (p[0] - lo[0]) / (hi[0] - lo[0]) * inner.width(),
            inner.bottom() - (p[1] - lo[1]) / (hi[1] - lo[1]) * inner.height(),
        )
    };
    for (pts, col, line) in series {
        if *line {
            let v: Vec<Pos2> = pts.iter().map(|&p| map(p)).collect();
            painter.add(egui::Shape::line(v, Stroke::new(1.5, *col)));
        } else {
            for &p in pts.iter() {
                painter.circle_filled(map(p), 2.0, *col);
            }
        }
    }
    let small = egui::FontId::proportional(10.0);
    let weak = ui.visuals().weak_text_color();
    painter.text(
        rect.left_top() + Vec2::new(4.0, 2.0),
        egui::Align2::LEFT_TOP,
        labels.1,
        small.clone(),
        weak,
    );
    painter.text(
        rect.right_bottom() - Vec2::new(4.0, 2.0),
        egui::Align2::RIGHT_BOTTOM,
        labels.0,
        small,
        weak,
    );
    let _ = Rect::NOTHING;
}

impl Panel for Charts {
    fn title(&self) -> &'static str {
        "Charts"
    }
    fn ui(
        &mut self,
        ui: &mut egui::Ui,
        v: &ViewState,
        _layers: &[(&'static str, &'static str)],
    ) -> Vec<Msg> {
        let m = &v.metrics;
        ui.label(format!(
            "{} settlements, {:.0} people · >2k: {} · >5k: {} · slope {:.2}",
            m.count, m.total_pop, m.over_2000, m.over_5000, m.rank_slope
        ));
        let mut sizes: Vec<f32> = v
            .snapshot
            .settlements
            .iter()
            .map(|s| s.pop)
            .filter(|&p| p > 1.0)
            .collect();
        sizes.sort_by(|a, b| b.total_cmp(a));
        let pts: Vec<[f32; 2]> = sizes
            .iter()
            .enumerate()
            .map(|(i, &p)| [((i + 1) as f32).log10(), p.log10()])
            .collect();
        let zipf: Vec<[f32; 2]> = match sizes.first() {
            Some(&top) => vec![
                [0.0, top.log10()],
                [
                    (sizes.len() as f32).log10(),
                    (top / sizes.len() as f32).log10(),
                ],
            ],
            None => Vec::new(),
        };
        ui.weak("Rank-size, log-log; grey line is Zipf (slope −1)");
        chart(
            ui,
            140.0,
            &[
                (&zipf, Color32::GRAY, true),
                (&pts, Color32::from_rgb(210, 70, 50), false),
            ],
            ("log rank", "log pop"),
        );
        ui.weak("Population over time: total (blue), largest (red)");
        let total: Vec<[f32; 2]> = v.history.iter().map(|h| [h.year as f32, h.total]).collect();
        let largest: Vec<[f32; 2]> = v
            .history
            .iter()
            .map(|h| [h.year as f32, h.largest])
            .collect();
        chart(
            ui,
            140.0,
            &[
                (&total, Color32::from_rgb(70, 120, 210), true),
                (&largest, Color32::from_rgb(210, 70, 50), true),
            ],
            ("year", "people"),
        );
        Vec::new()
    }
}
