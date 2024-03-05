use super::{app::App, theme, tr::tr};
use crate::util;
use anyhow::{anyhow, Result};
use egui::{
    containers::scroll_area::ScrollBarVisibility, Color32, FontId, RichText, ScrollArea, Ui,
};
use egui_extras::{Size, StripBuilder};
use std::{fs, path::Path};

type UiItems = Vec<UiItem>;

struct UiItem {
    name: String,
    value: String,
    color: Color32,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Stats {
    #[serde(skip)]
    pub errors: Vec<String>,

    pub market: Vec<Market>,

    pub crypto: Crypto,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Market {
    pub name: String,
    pub value: f64,
    pub precent: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Crypto {
    pub greed_fear: GreedFear,
    pub global: Global,
    pub gas_fee: GasFee,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct GreedFear {
    pub data: Vec<GreedFearData>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct GreedFearData {
    pub value: String,
    pub timestamp: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Global {
    pub total_market_cap_usd: u64,
    pub total_24h_volume_usd: u64,
    pub bitcoin_percentage_of_market_cap: f64,
    pub last_updated: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct GasFee {
    pub bitcoin: (u64, u64, u64),
    pub ethereum: u64,
}

impl Crypto {
    fn into_items(&self, is_cn: bool) -> UiItems {
        let mut items = vec![];

        if self.greed_fear.data.len() == 2 {
            items.push(UiItem {
                name: tr(is_cn, "贪婪恐慌(今天/昨天)").to_string(),
                value: format!(
                    "{}/{}",
                    self.greed_fear.data[0].value, self.greed_fear.data[1].value
                ),
                color: if self.greed_fear.data[0]
                    .value
                    .parse::<u32>()
                    .unwrap_or_default()
                    >= 50
                {
                    theme::UP_COLOR
                } else {
                    theme::DOWN_COLOR
                },
            });
        }

        items.push(UiItem {
            name: tr(is_cn, "加密总市值(USD)").to_string(),
            value: util::format_number_with_commas(&format!(
                "{}",
                self.global.total_market_cap_usd
            )),
            color: theme::UP_COLOR,
        });

        items.push(UiItem {
            name: tr(is_cn, "24h交易量(USD)").to_string(),
            value: util::format_number_with_commas(&format!(
                "{}",
                self.global.total_24h_volume_usd
            )),
            color: theme::UP_COLOR,
        });

        items.push(UiItem {
            name: tr(is_cn, "BTC市值占比").to_string(),
            value: util::pretty_precent(self.global.bitcoin_percentage_of_market_cap),
            color: if self.global.bitcoin_percentage_of_market_cap >= 50. {
                theme::UP_COLOR
            } else {
                theme::DOWN_COLOR
            },
        });

        items.push(UiItem {
            name: tr(is_cn, "BTC油费(慢/正常/快)").to_string(),
            value: format!(
                "{}/{}/{} vSat",
                self.gas_fee.bitcoin.0, self.gas_fee.bitcoin.1, self.gas_fee.bitcoin.2
            ),
            color: theme::UP_COLOR,
        });

        items.push(UiItem {
            name: tr(is_cn, "ETH油费").to_string(),
            value: format!("{:.0} GWei", self.gas_fee.ethereum as f64 / 1e9),
            color: theme::UP_COLOR,
        });

        items
    }
}

pub fn fetch(_api_key: &str, save_path: &Path) -> Result<Stats> {
    let mut stats = Stats::default();

    match fetch_market() {
        Ok(v) => stats.market = v,
        Err(e) => stats.errors.push(format!("fetch market error: {e:?}")),
    }

    match fetch_crypto() {
        Ok(v) => stats.crypto = v,
        Err(e) => stats
            .errors
            .push(format!("fetch crypto stats error: {e:?}")),
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

pub fn fetch_crypto() -> Result<Crypto> {
    const API: &str = "https://heng30.xyz/apisvr/cryptocurrency/stats";

    let client = reqwest::blocking::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();

    Ok(client.get(API).send()?.json::<Crypto>()?)
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
        ui.add_space(theme::SPACING * 2.);
        crypto_ui(app, ui);
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
                                RichText::new(tr(is_cn, &item.name))
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

fn crypto_ui(app: &mut App, ui: &mut Ui) {
    let is_cn = app.conf.ui.is_cn;
    ui.vertical(|ui| {
        ui.vertical_centered(|ui| {
            ui.label(
                RichText::new(tr(is_cn, "加密指数"))
                    .color(Color32::BLACK)
                    .font(FontId::proportional(theme::DEFAULT_FONT_SIZE + 3.)),
            );
        });

        ui.separator();

        for item in app.stats.crypto.into_items(is_cn).into_iter() {
            ui.horizontal(|ui| {
                StripBuilder::new(ui)
                    .size(Size::relative(0.5))
                    .size(Size::remainder())
                    .horizontal(|mut strip| {
                        strip.cell(|ui| {
                            ui.label(
                                RichText::new(&item.name)
                                    .color(item.color)
                                    .font(FontId::proportional(theme::DEFAULT_FONT_SIZE)),
                            );
                        });

                        strip.cell(|ui| {
                            ui.label(
                                RichText::new(&item.value)
                                    .color(item.color)
                                    .font(FontId::proportional(theme::DEFAULT_FONT_SIZE)),
                            );
                        });
                    });
            });
        }
    });
}
