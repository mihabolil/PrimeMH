use notan::text::Font;

mod draw_item_log;
mod draw_lines;
mod draw_map;
mod draw_objects;
mod draw_presets;
mod draw_units;
mod images;
pub mod ui;
mod util;

pub struct Fonts {
    pub exocet_font: Font,
    pub formal_font: Font,
    pub korean_font: Font,
    pub taiwan_font: Font,
    pub blizzard_font: Font,
}