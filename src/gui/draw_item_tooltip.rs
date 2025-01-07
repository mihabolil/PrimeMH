use notan::draw::*;
use notan::math::Rect;
use notan::prelude::*;

use crate::memory::gamedata::GameData;
use crate::settings::Settings;
use crate::types::last_hovered::UnitType;
use crate::types::stats::{format_stat_list, StatEnum, format_affixes};

pub fn draw_item_tooltip(draw: &mut Draw, game_data: &GameData, settings: &Settings, font: &Font, scale: &f32, mouse_pos: (i32, i32)) {
    if !settings.item_hover.enabled {
        return;
    }
    if game_data.last_hovered.is_hovered && game_data.last_hovered.is_tooltip {
        if game_data.last_hovered.unit_type == UnitType::Item {
            match game_data.items.iter().find(|item| item.unit_id == game_data.last_hovered.unit_id) {
                Some(item) => { 
                    
                    let font_size = settings.item_hover.text_size * scale;
                    let player_level = game_data.player.stats.iter().find(|s| s.stat == StatEnum::Level).unwrap().value;
                    let item_stat_str = format_stat_list(&item.item_stats, player_level).join("\n");
                    let item_stat_str = format_affixes(item_stat_str.clone(), item.is_ethereal(), item.num_sockets);

                    // log::info!("item {:?}  {:?} {:?}", item.txt_file_no, item_stat_str, mouse_pos);
                    draw.text(font, &item_stat_str)
                        .position(-500.0 as f32, -500.0 as f32)
                        .size(font_size)
                        .color(Color::from_hex(0xFFFFFF00))
                        .h_align_center()
                        .v_align_bottom();

                    let bounds: Rect = draw.last_text_bounds();
                    let pad = font_size / 10.0;

                    draw.rect((mouse_pos.0 as f32 - pad - bounds.width, mouse_pos.1 as f32 - bounds.height - pad), (bounds.width + pad + pad, bounds.height + pad + pad))
                        .color(Color::from_hex(0x000000AA));
                    
                    draw.text(font, &item_stat_str)
                        .position(mouse_pos.0 as f32 - (bounds.width / 2.0) + 1.5, mouse_pos.1 as f32 - bounds.height as f32 + 1.5)
                        .size(font_size)
                        .color(Color::from_hex(0x000000FF))
                        .h_align_center()
                        .v_align_top();

                    draw.text(font, &item_stat_str)
                        .position(mouse_pos.0 as f32 - (bounds.width / 2.0), mouse_pos.1 as f32 - bounds.height as f32)
                        .size(font_size)
                        .color(Color::from_hex(0x6D6DFFFF))
                        .h_align_center()
                        .v_align_top();
                },
                None => (),
            }
        }
    }
}