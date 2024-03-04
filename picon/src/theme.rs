use egui::{
    Color32, ColorImage, Context, FontData, FontDefinitions, FontFamily, Style, Vec2, Visuals,
};

pub const PADDING: f32 = 4.;
pub const SPACING: f32 = 4.;
pub const ICON_SIZE: Vec2 = Vec2::new(24.0, 24.0);
pub const BACK_ICON_SIZE: Vec2 = Vec2::new(16.0, 16.0);

pub const DEFAULT_FONT_SIZE: f32 = 14.0;

pub const TITLE_COLOR: Color32 = Color32::from_rgb(0, 0, 200);
pub const UP_COLOR: Color32 = Color32::DARK_GREEN;
pub const DOWN_COLOR: Color32 = Color32::RED;

pub const BRAND_COLOR: Color32 = Color32::from_rgb(0, 0, 139);
pub const LIGHT_COLOR: Color32 = Color32::GRAY;
pub const INFO_COLOR: Color32 = Color32::from_rgb(144, 144, 153);
pub const SUCCESS_COLOR: Color32 = Color32::from_rgb(102, 194, 58);
pub const WARN_COLOR: Color32 = Color32::from_rgb(230, 162, 60);
pub const DANGER_COLOR: Color32 = Color32::from_rgb(245, 108, 108);

pub const REFRESH_ICON: &[u8] = include_bytes!("./res/image/refresh.png");
pub const LANGUAGE_ICON: &[u8] = include_bytes!("./res/image/lang.png");
pub const BRAND_ICON: &[u8] = include_bytes!("./res/image/brand.png");
pub const BACK_ICON: &[u8] = include_bytes!("./res/image/back-simple.png");
pub const ABOUT_ICON: &[u8] = include_bytes!("./res/image/about.png");
pub const CIRCLE_GRAY_ICON: &[u8] = include_bytes!("./res/image/circle-gray.png");
pub const CIRCLE_RED_ICON: &[u8] = include_bytes!("./res/image/circle-red.png");
pub const LATEST_ICON: &[u8] = include_bytes!("./res/image/latest.png");
pub const TRENDING_ICON: &[u8] = include_bytes!("./res/image/trending.png");
pub const STATS_ICON: &[u8] = include_bytes!("./res/image/stats.png");

pub fn init(ctx: &Context) {
    set_font(ctx);
    set_style(ctx);
}

fn set_style(ctx: &Context) {
    ctx.set_visuals(Visuals::light());

    let mut style: Style = (*ctx.style()).clone();
    style.spacing.scroll_bar_width = 2.0;
    style.visuals.hyperlink_color = LIGHT_COLOR;
    ctx.set_style(style);
}

fn set_font(ctx: &Context) {
    let mut fonts = FontDefinitions::default();

    fonts.font_data.insert(
        "SourceHanSerifCN".to_owned(),
        FontData::from_static(include_bytes!("./res/font/SourceHanSerifCN.ttf")),
    );

    fonts
        .families
        .get_mut(&FontFamily::Proportional)
        .unwrap()
        .insert(0, "SourceHanSerifCN".to_owned());

    ctx.set_fonts(fonts);
}

pub fn load_image_from_memory(image_data: &[u8]) -> ColorImage {
    let image = image::load_from_memory(image_data).unwrap();
    let size = [image.width() as _, image.height() as _];
    let image_buffer = image.to_rgba8();
    let pixels = image_buffer.as_flat_samples();
    ColorImage::from_rgba_unmultiplied(size, pixels.as_slice())
}
