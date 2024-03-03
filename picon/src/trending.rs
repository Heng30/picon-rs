use super::{app::App, theme, tr::tr};
use egui::{RichText, Ui};

pub fn ui(app: &mut App, ui: &mut Ui) {
    let is_cn = app.conf.ui.is_cn;
    ui.vertical_centered(|ui| {
        ui.label(RichText::new(tr(is_cn, "没有实现...")).color(theme::TITLE_COLOR));
    });
}
