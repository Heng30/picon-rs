use super::{app::App, theme, tr::tr};
use crate::util;
use anyhow::{anyhow, Result};
use egui::{containers::scroll_area::ScrollBarVisibility, FontId, RichText, ScrollArea, Ui};
use egui_extras::{Size, StripBuilder};
use std::{fs, path::Path};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Market {
    pub name: String,
    pub value: f64,
    pub precent: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Stats {
    #[serde(skip)]
    pub errors: Vec<String>,

    pub market: Vec<Market>,
}

pub fn fetch(_api_key: &str, save_path: &Path) -> Result<Stats> {
    let mut stats = Stats::default();

    match fetch_market() {
        Ok(v) => stats.market = v,
        Err(e) => stats.errors.push(format!("fetch market error: {e:?}")),
    }

    _ = save(save_path, &stats);

    Ok(stats)
}

pub fn fetch_market() -> Result<Vec<Market>> {
    const API: &str = "https://heng30.xyz/apisvr/market/latest";

    let client = reqwest::blocking::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();

    Ok(client.get(API).send()?.json::<Vec<Market>>()?)

}

pub fn init(app: &mut App) {
    if let Err(e) = load(app) {
        log::debug!("{e:?}");
    }
}

fn load(app: &mut App) -> Result<()> {
    let path = app.conf.cache_dir.join("stats.json");
    let text = fs::read_to_string(path)?;
    app.stats = serde_json::from_str::<Stats>(&text)?;

    Ok(())
}


fn save(path: &Path, latest: &Stats) -> Result<()> {
    match serde_json::to_string(latest) {
        Ok(text) => Ok(fs::write(path, text)?),
        Err(e) => Err(anyhow!("{e:?}")),
    }
}


pub fn ui(app: &mut App, ui: &mut Ui) {
    let row_height = ui.spacing().interact_size.y * 2.;
    let mut sarea = ScrollArea::vertical()
        .auto_shrink([false, false])
        .scroll_bar_visibility(ScrollBarVisibility::AlwaysVisible);

    if app.is_scroll_to_top_stats {
        sarea = sarea.vertical_scroll_offset(0.0);
        app.is_scroll_to_top_latest = false;
    }

    sarea.show_rows(ui, row_height, 1, |ui, _row_range| {
        market_ui(app, ui);
    });
}

fn market_ui(app: &mut App, ui: &mut Ui) {
    let is_cn = app.conf.ui.is_cn;
    ui.vertical(|ui| {
        ui.vertical_centered(|ui| {
            ui.label(
                RichText::new(tr(is_cn, "经济指数"))
                    .color(egui::Color32::BLACK)
                    .font(FontId::proportional(theme::DEFAULT_FONT_SIZE + 3.)),
            );
        });

        ui.separator();

        for item in app.stats.market.iter() {
            let text_color = if item.precent >= 0. {
                theme::UP_COLOR
            } else {
                theme::DOWN_COLOR
            };

            ui.horizontal(|ui| {
                StripBuilder::new(ui)
                    .size(Size::relative(0.5))
                    .size(Size::remainder())
                    .horizontal(|mut strip| {
                        strip.cell(|ui| {
                            ui.label(
                                RichText::new(&item.name)
                                    .color(text_color)
                                    .font(FontId::proportional(theme::DEFAULT_FONT_SIZE)),
                            );
                        });

                        strip.cell(|ui| {
                            ui.columns(2, |columns| {
                                columns[0].horizontal(|ui| {
                                    ui.label(
                                        RichText::new(&util::pretty_price(item.value))
                                            .color(text_color)
                                            .font(FontId::proportional(theme::DEFAULT_FONT_SIZE)),
                                    );
                                });

                                columns[1].horizontal(|ui| {
                                    ui.label(
                                        RichText::new(&util::pretty_precent(item.precent))
                                            .color(text_color)
                                            .font(FontId::proportional(theme::DEFAULT_FONT_SIZE)),
                                    );
                                });
                            });
                        });
                    });
            });
        }
    });
}
