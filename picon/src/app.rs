use super::{
    about, apikey,
    config::Config,
    latest::{self, Latest},
    stats::{self, Stats},
    theme,
    tr::tr,
    trending, util,
};
use egui::{
    containers::Frame, Align, Button, Color32, Context, ImageButton, Layout, Pos2, RichText,
    Stroke, TextureHandle, Ui, Window,
};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::mpsc::{self, Receiver, SyncSender};
use std::sync::Arc;

#[allow(unused)]
#[derive(Clone, Debug)]
enum MsgType {
    Info,
    Warn,
    Success,
    Danger,
}

impl Default for MsgType {
    fn default() -> Self {
        Self::Info
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum CurrentPanel {
    Latest,
    Trending,
    Stats,
    About,
}

impl Default for CurrentPanel {
    fn default() -> Self {
        Self::Latest
    }
}

#[derive(Clone, Debug, Default)]
struct MsgSpec {
    msg: String,
    msg_type: MsgType,
    timestamp: i64,
}

#[derive(Clone, Debug)]
enum ChannelInnerItem {
    Latest(Latest),
    Trending(()),
    Stats(Stats),
}

#[derive(Clone, Debug)]
enum ChannelItem {
    ErrMsg((CurrentPanel, String)),
    Item(ChannelInnerItem),
}

#[derive(Clone, Default)]
pub struct App {
    pub is_scroll_to_top_latest: bool,
    pub is_scroll_to_top_trending: bool,
    pub is_scroll_to_top_stats: bool,

    pub is_fetching_latest: bool,
    pub is_fetching_trending: bool,
    pub is_fetching_stats: bool,

    pub latest: Latest,
    pub trending: (),
    pub stats: Stats,

    pub current_panel: CurrentPanel,
    pub prev_panel: CurrentPanel,

    pub conf: Config,

    pub about_setting: about::Setting,
    pub latest_setting: latest::Setting,
    msg_spec: MsgSpec,

    tx: Option<Arc<SyncSender<ChannelItem>>>,
    rx: Option<Rc<RefCell<Receiver<ChannelItem>>>>,

    pub cmc_pro_api_key: String,

    brand_icon: Option<TextureHandle>,
    refresh_icon: Option<TextureHandle>,
    language_icon: Option<TextureHandle>,
    about_icon: Option<TextureHandle>,
    pub back_icon: Option<TextureHandle>,
    pub circle_gray_icon: Option<TextureHandle>,
    pub circle_red_icon: Option<TextureHandle>,
    pub latest_icon: Option<TextureHandle>,
    pub trending_icon: Option<TextureHandle>,
    pub stats_icon: Option<TextureHandle>,
}

impl App {
    pub fn new() -> Self {
        let mut app = App::default();
        let (tx, rx) = mpsc::sync_channel(10);
        (app.tx, app.rx) = (Some(Arc::new(tx)), Some(Rc::new(RefCell::new(rx))));
        app.cmc_pro_api_key = apikey::CMC_PRO_API_KEY.to_string();

        app
    }

    pub fn init(&mut self, ctx: &Context) {
        if let Err(e) = self.conf.init() {
            log::warn!("{e:?}");
        }

        self.brand_icon = Some(ctx.load_texture(
            "brand-icon",
            theme::load_image_from_memory(theme::BRAND_ICON),
            Default::default(),
        ));

        self.refresh_icon = Some(ctx.load_texture(
            "refresh-icon",
            theme::load_image_from_memory(theme::REFRESH_ICON),
            Default::default(),
        ));

        self.language_icon = Some(ctx.load_texture(
            "language-icon",
            theme::load_image_from_memory(theme::LANGUAGE_ICON),
            Default::default(),
        ));

        self.about_icon = Some(ctx.load_texture(
            "about-icon",
            theme::load_image_from_memory(theme::ABOUT_ICON),
            Default::default(),
        ));

        self.back_icon = Some(ctx.load_texture(
            "back-icon",
            theme::load_image_from_memory(theme::BACK_ICON),
            Default::default(),
        ));

        self.circle_gray_icon = Some(ctx.load_texture(
            "circle_gray-icon",
            theme::load_image_from_memory(theme::CIRCLE_GRAY_ICON),
            Default::default(),
        ));

        self.circle_red_icon = Some(ctx.load_texture(
            "circle_red-icon",
            theme::load_image_from_memory(theme::CIRCLE_RED_ICON),
            Default::default(),
        ));

        self.latest_icon = Some(ctx.load_texture(
            "latest-icon",
            theme::load_image_from_memory(theme::LATEST_ICON),
            Default::default(),
        ));

        self.trending_icon = Some(ctx.load_texture(
            "trending-icon",
            theme::load_image_from_memory(theme::TRENDING_ICON),
            Default::default(),
        ));

        self.stats_icon = Some(ctx.load_texture(
            "stats-icon",
            theme::load_image_from_memory(theme::STATS_ICON),
            Default::default(),
        ));

        latest::init(self);
        stats::init(self);

        self.fetch_latest();
        // self.fetch_stats();
    }

    // only call this function when switching to secondary layer, such as `about` panel
    pub fn switch_panel(&mut self, panel: CurrentPanel) {
        self.prev_panel = self.current_panel;
        self.current_panel = panel;
    }

    pub fn ui(&mut self, ctx: &Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if self.current_panel != CurrentPanel::About {
                self.header(ui);
            }

            match self.current_panel {
                CurrentPanel::Latest => latest::ui(self, ui),
                CurrentPanel::Trending => trending::ui(self, ui),
                CurrentPanel::Stats => stats::ui(self, ui),
                CurrentPanel::About => about::ui(self, ui),
            }

            self.update_data();
        });

        self.popup_message(ctx);
    }

    fn header(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                ui.heading(
                    RichText::new(tr(
                        self.conf.ui.is_cn,
                        match self.current_panel {
                            CurrentPanel::Latest => "行情",
                            CurrentPanel::Trending => "热门",
                            CurrentPanel::Stats => "指数",
                            _ => "",
                        },
                    ))
                    .color(theme::BRAND_COLOR),
                );
            });

            // double-clicked-area to scroll to top
            ui.with_layout(
                Layout::centered_and_justified(egui::Direction::LeftToRight),
                |ui| {
                    Frame::none().show(ui, |ui| {
                        let btn = Button::new("").frame(false);
                        if ui.add(btn).double_clicked() {
                            match self.current_panel {
                                CurrentPanel::Latest => self.is_scroll_to_top_latest = true,
                                CurrentPanel::Trending => self.is_scroll_to_top_trending = true,
                                CurrentPanel::Stats => self.is_scroll_to_top_stats = true,
                                _ => (),
                            }
                        }
                    });
                },
            );

            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                ui.add_space(theme::PADDING * 2.);

                if ui
                    .add(
                        ImageButton::new(
                            self.about_icon.clone().unwrap().id(),
                            theme::ICON_SIZE * 0.9,
                        )
                        .frame(false),
                    )
                    .clicked()
                {
                    self.switch_panel(CurrentPanel::About);
                }

                if ui
                    .add(
                        ImageButton::new(
                            self.stats_icon.clone().unwrap().id(),
                            theme::ICON_SIZE * 0.9,
                        )
                        .frame(false),
                    )
                    .clicked()
                {
                    self.current_panel = CurrentPanel::Stats;
                }

                if ui
                    .add(
                        ImageButton::new(self.latest_icon.clone().unwrap().id(), theme::ICON_SIZE)
                            .frame(false),
                    )
                    .clicked()
                {
                    self.current_panel = CurrentPanel::Latest;
                }

                if ui
                    .add(
                        ImageButton::new(
                            self.language_icon.clone().unwrap().id(),
                            theme::ICON_SIZE,
                        )
                        .frame(false),
                    )
                    .clicked()
                {
                    self.conf.ui.is_cn = !self.conf.ui.is_cn;
                    if let Err(e) = self.conf.save() {
                        log::warn!("{e:?}");
                    }
                }

                if ui
                    .add(
                        ImageButton::new(self.refresh_icon.clone().unwrap().id(), theme::ICON_SIZE)
                            .frame(false),
                    )
                    .clicked()
                {
                    match self.current_panel {
                        CurrentPanel::Latest => self.fetch_latest(),
                        CurrentPanel::Stats => self.fetch_stats(),
                        CurrentPanel::Trending => todo!(),
                        _ => (),
                    }
                }

                let is_show_refresh_text = match self.current_panel {
                    CurrentPanel::Latest => self.is_fetching_latest,
                    CurrentPanel::Stats => self.is_fetching_stats,
                    _ => false,
                };

                if is_show_refresh_text {
                    ui.label(
                        RichText::new(tr(self.conf.ui.is_cn, "正在刷新")).color(theme::TITLE_COLOR),
                    );
                }
            });
        });

        ui.add_space(theme::SPACING);
    }

    fn update_data(&mut self) {
        let rx = self.rx.clone();

        if let Ok(item) = rx.unwrap().borrow_mut().try_recv() {
            match item {
                ChannelItem::ErrMsg((panel, msg)) => {
                    match panel {
                        CurrentPanel::Latest => self.is_fetching_latest = false,
                        CurrentPanel::Trending => self.is_fetching_trending = false,
                        CurrentPanel::Stats => self.is_fetching_stats = false,
                        _ => (),
                    }
                    self.show_message(msg, MsgType::Warn);
                }
                ChannelItem::Item(item) => match item {
                    ChannelInnerItem::Latest(item) => {
                        if let Some(e) = item.status.error_message {
                            self.show_message(e, MsgType::Warn);
                        } else {
                            if !item.data.is_empty() {
                                self.latest = item;
                                latest::update_addition_info(self);
                                latest::sort_by_key(self, self.latest_setting.sort_key, false);
                            }
                        }
                        self.is_fetching_latest = false;
                    }
                    ChannelInnerItem::Trending(_) => {
                        self.is_fetching_trending = false;
                        unimplemented!();
                    }
                    ChannelInnerItem::Stats(item) => {
                        if !item.errors.is_empty() {
                            self.show_message(item.errors.join("\n\n"), MsgType::Warn);
                        } else {
                            self.stats = item;
                        }
                        self.is_fetching_stats = false;
                    }
                },
            }
        };
    }

    fn fetch_latest(&mut self) {
        if self.is_fetching_latest {
            return;
        }
        self.is_fetching_latest = true;

        let tx = self.tx.clone();
        let cache_dir = self.conf.cache_dir.clone();
        let api_key = self.cmc_pro_api_key.clone();

        std::thread::spawn(move || {
            match latest::fetch(&api_key, cache_dir.join("latest.json").as_path()) {
                Err(e) => {
                    _ = tx
                        .unwrap()
                        .try_send(ChannelItem::ErrMsg((CurrentPanel::Latest, e.to_string())));
                }
                Ok(v) => {
                    _ = tx
                        .unwrap()
                        .try_send(ChannelItem::Item(ChannelInnerItem::Latest(v)));
                }
            }
        });
    }

    fn fetch_stats(&mut self) {
        if self.is_fetching_stats {
            return;
        }
        self.is_fetching_stats = true;

        let tx = self.tx.clone();
        let cache_dir = self.conf.cache_dir.clone();
        let api_key = String::default();

        std::thread::spawn(move || {
            match stats::fetch(&api_key, cache_dir.join("stats.json").as_path()) {
                Err(e) => {
                    _ = tx
                        .unwrap()
                        .try_send(ChannelItem::ErrMsg((CurrentPanel::Stats, e.to_string())));
                }
                Ok(v) => {
                    _ = tx
                        .unwrap()
                        .try_send(ChannelItem::Item(ChannelInnerItem::Stats(v)));
                }
            }
        });
    }

    fn popup_message(&mut self, ctx: &Context) {
        let mut is_show = util::timestamp() - self.msg_spec.timestamp < 5_i64;

        let frame = Frame::none()
            .fill(match self.msg_spec.msg_type {
                MsgType::Success => theme::SUCCESS_COLOR,
                MsgType::Warn => theme::WARN_COLOR,
                MsgType::Danger => theme::DANGER_COLOR,
                _ => theme::INFO_COLOR,
            })
            .rounding(0.0)
            .inner_margin(theme::PADDING)
            .stroke(Stroke {
                width: 1.0,
                color: Color32::BLACK,
            });

        Window::new("popup-message")
            .title_bar(false)
            .open(&mut is_show)
            .collapsible(false)
            .auto_sized()
            .constrain(true)
            .interactable(false)
            .fixed_pos(Pos2::new(theme::PADDING, theme::PADDING))
            .frame(frame)
            .show(ctx, |ui| {
                ui.label(&self.msg_spec.msg);
            });
    }

    fn show_message(&mut self, msg: String, msg_type: MsgType) {
        self.msg_spec.msg = msg;
        self.msg_spec.msg_type = msg_type;
        self.msg_spec.timestamp = util::timestamp();
    }
}

#[allow(unused)]
pub fn is_mobile(ctx: &egui::Context) -> bool {
    let screen_size = ctx.screen_rect().size();
    screen_size.x < 550.0
}
