use super::util;
use super::{app::App, theme, tr::tr};
use anyhow::{anyhow, Result};
use egui::{
    containers::scroll_area::ScrollBarVisibility, Button, FontId, ImageButton, RichText,
    ScrollArea, Ui,
};
use egui_extras::{Size, StripBuilder};
use reqwest::header::{HeaderMap, ACCEPT};
use std::{collections::HashSet, fs, path::Path};

const LEFT_HEADER_WIDTH: f32 = 80.;

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
enum SortKey {
    Marker,
    Rank,
    Symbol,
    Price,
    H24,
    D7,
}

impl Default for SortKey {
    fn default() -> Self {
        SortKey::Marker
    }
}

#[derive(Default, Debug, Clone)]
pub struct Setting {
    sort_key: SortKey,
    marker_symbols: HashSet<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Latest {
    pub status: LatestStatus,
    #[serde(default)]
    pub data: Vec<LatestDataItem>,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct LatestStatus {
    pub timestamp: String,
    pub error_message: Option<String>,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct LatestDataItem {
    pub id: u64,
    pub symbol: String,
    pub quote: LatestDataItemQuote,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct LatestDataItemQuote {
    #[serde(rename(deserialize = "USD"), rename(serialize = "USD"))]
    pub usd: LatestDataItemQuoteUSD,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct LatestDataItemQuoteUSD {
    pub price: f64,
    pub percent_change_24h: f64,
    pub percent_change_7d: f64,
}

// curl -H "X-CMC_PRO_API_KEY: $API_KEY" -H "Accept: application/json" -d "start=1&limit=100&convert=USD&aux=cmc_rank" -G https://pro-api.coinmarketcap.com/v1/cryptocurrency/listings/latest
pub fn fetch(api_key: &str, save_path: &Path) -> Result<Latest> {
    const API: &str = "https://pro-api.coinmarketcap.com/v1/cryptocurrency/listings/latest";

    let client = reqwest::blocking::Client::new();
    let resp = client
        .get(API)
        .headers(headers(api_key))
        .query(&[
            ("start", "1"),
            ("limit", "100"),
            ("convert", "USD"),
            ("aux", "cmc_rank"),
        ])
        .send()?
        .json::<Latest>()?;

    if resp.status.error_message.is_none() {
        _ = save(save_path, &resp);
    }

    Ok(resp)
}

pub fn init(app: &mut App) {
    if let Err(e) = load_latest(app) {
        log::debug!("{e:?}");
    }

    if let Err(e) = load_marker_symbols(app) {
        log::debug!("{e:?}");
    }
}

fn load_marker_symbols(app: &mut App) -> Result<()> {
    let path = app.conf.cache_dir.join("marker_symbols.json");
    let text = fs::read_to_string(path)?;
    app.latest_setting.marker_symbols = serde_json::from_str::<Vec<String>>(&text)?
        .into_iter()
        .collect();

    Ok(())
}

fn load_latest(app: &mut App) -> Result<()> {
    let path = app.conf.cache_dir.join("latest.json");
    let text = fs::read_to_string(path)?;
    app.latest = serde_json::from_str::<Latest>(&text)?;

    Ok(())
}

fn save(path: &Path, latest: &Latest) -> Result<()> {
    match serde_json::to_string(latest) {
        Ok(text) => Ok(fs::write(path, text)?),
        Err(e) => Err(anyhow!("{e:?}")),
    }
}

fn headers(api_key: &str) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, "application/json".parse().unwrap());
    headers.insert("X-CMC_PRO_API_KEY", api_key.parse().unwrap());
    headers
}

pub fn ui(app: &mut App, ui: &mut Ui) {
    list_header(app, ui);
    list_body(app, ui);
}

fn list_header(app: &mut App, ui: &mut Ui) {
    let is_cn = app.conf.ui.is_cn;

    ui.horizontal(|ui| {
        StripBuilder::new(ui)
            .size(Size::exact(LEFT_HEADER_WIDTH))
            .size(Size::remainder())
            .horizontal(|mut strip| {
                strip.cell(|ui| {
                    let items = vec![
                        (SortKey::Marker, tr(is_cn, "关注")),
                        (SortKey::Rank, tr(is_cn, "排名")),
                    ];
                    ui.columns(items.len(), |columns| {
                        for (i, v) in items.into_iter().enumerate() {
                            columns[i].horizontal(|ui| {
                                let btn = Button::new(
                                    RichText::new(v.1)
                                        .font(FontId::proportional(theme::DEFAULT_FONT_SIZE + 1.)),
                                )
                                .frame(false);

                                if ui.add(btn).clicked() {
                                    app.latest_setting.sort_key = v.0;
                                }
                            });
                        }
                    });
                });

                strip.cell(|ui| {
                    let items = vec![
                        (SortKey::Symbol, tr(is_cn, "代币")),
                        (SortKey::Price, tr(is_cn, "价格")),
                        (SortKey::H24, tr(is_cn, "24h")),
                        (SortKey::D7, tr(is_cn, "7d")),
                    ];
                    ui.columns(items.len(), |columns| {
                        for (i, v) in items.into_iter().enumerate() {
                            columns[i].horizontal(|ui| {
                                let btn = Button::new(
                                    RichText::new(v.1)
                                        .font(FontId::proportional(theme::DEFAULT_FONT_SIZE + 1.)),
                                )
                                .frame(false);

                                if ui.add(btn).clicked() {
                                    app.latest_setting.sort_key = v.0;
                                }
                            });
                        }
                    });
                });
            });
    });

    ui.add_space(theme::SPACING);
}

fn list_body(app: &mut App, ui: &mut Ui) {
    let num_rows = app.latest.data.len();
    if num_rows == 0 {
        return;
    }

    let row_height = ui.spacing().interact_size.y * 2.;
    let mut sarea = ScrollArea::vertical()
        .auto_shrink([false, false])
        .scroll_bar_visibility(ScrollBarVisibility::AlwaysVisible);

    if app.is_scroll_to_top {
        sarea = sarea.vertical_scroll_offset(0.0);
        app.is_scroll_to_top = false;
    }

    sarea.show_rows(ui, row_height, num_rows, |ui, row_range| {
        for row in row_range {
            list_item(app, ui, row);
        }
    });
}

fn list_item(app: &mut App, ui: &mut Ui, row: usize) {
    let data = app.latest.data[row].clone();

    let text_color = if data.quote.usd.percent_change_24h >= 0. {
        theme::UP_COLOR
    } else {
        theme::DOWN_COLOR
    };

    let marker_icon_id = if app.latest_setting.marker_symbols.contains(&data.symbol) {
        app.circle_red_icon.clone().unwrap().id()
    } else {
        app.circle_gray_icon.clone().unwrap().id()
    };

    ui.add_space(theme::SPACING * 2.);

    ui.horizontal(|ui| {
        StripBuilder::new(ui)
            .size(Size::exact(LEFT_HEADER_WIDTH))
            .size(Size::remainder())
            .horizontal(|mut strip| {
                strip.cell(|ui| {
                    ui.columns(2, |columns| {
                        if columns[0]
                            .add(ImageButton::new(marker_icon_id, theme::ICON_SIZE).frame(false))
                            .clicked()
                        {
                            update_marker_symbols(app, &data.symbol);
                        };

                        columns[1].label(
                            RichText::new(&format!("{}", row + 1))
                                .color(text_color)
                                .font(FontId::proportional(theme::DEFAULT_FONT_SIZE)),
                        );
                    });
                });

                strip.cell(|ui| {
                    let items = vec![
                        if data.symbol.len() > 6 {
                            data.symbol[..6].to_string()
                        } else {
                            data.symbol.clone()
                        },
                        util::pretty_price(data.quote.usd.price),
                        format!("{:.2}%", data.quote.usd.percent_change_24h),
                        format!("{:.2}%", data.quote.usd.percent_change_7d),
                    ];
                    ui.columns(items.len(), |columns| {
                        for (i, v) in items.into_iter().enumerate() {
                            columns[i].horizontal(|ui| {
                                ui.label(
                                    RichText::new(&v)
                                        .color(text_color)
                                        .font(FontId::proportional(theme::DEFAULT_FONT_SIZE)),
                                );
                            });
                        }
                    });
                });
            });
    });

    ui.add_space(theme::SPACING * 2.);
}

fn update_marker_symbols(app: &mut App, symbol: &str) {
    if app.latest_setting.marker_symbols.contains(symbol) {
        app.latest_setting.marker_symbols.remove(symbol);
    } else {
        app.latest_setting.marker_symbols.insert(symbol.to_string());
    }

    match serde_json::to_string(
        &app.latest_setting
            .marker_symbols
            .iter()
            .map(|v| v.clone())
            .collect::<Vec<String>>(),
    ) {
        Ok(text) => {
            let path = app.conf.cache_dir.join("marker_symbols.json");
            _ = fs::write(path, text);
        }
        Err(e) => log::warn!("{e:?}"),
    }
}
