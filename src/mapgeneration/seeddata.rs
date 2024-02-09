use std::path::PathBuf;

use crate::{settings::Settings, types::seedvalues::Difficulty};

use num_traits::FromPrimitive;
use rayon::prelude::{IntoParallelRefMutIterator, ParallelIterator};

use super::{
    super::types::seedvalues::SeedValues,
    blacha,
    jsondata::{LevelName, SeedData},
    mapgrid, pois,
    pois::POI,
    walkableexits,
};

pub struct SeedRequest {
    pub map_seed: u32,
    pub difficulty: u32,
    pub d2lodpath: PathBuf,
    pub blacha_exe: PathBuf,
}

pub fn generate_seed_data(seeddata: &SeedValues, settings: &Settings) -> SeedData {
    let difficulty = match seeddata.difficulty {
        Difficulty::Normal => 0,
        Difficulty::Nightmare => 1,
        Difficulty::Hell => 2,
        Difficulty::Invalid => 0,
    };
    let seed_request = SeedRequest {
        map_seed: seeddata.map_seed,
        difficulty,
        d2lodpath: settings.general.d2lodpath.clone(),
        blacha_exe: settings.general.blacha_exe.clone(),
    };
    let mut seed_data_json: SeedData = blacha::get_seed_data(seed_request);
    walkableexits::get_walkable_exits(&mut seed_data_json);
    seed_data_json.update_names();
    seed_data_json.process_map_data();
    seed_data_json
}

impl SeedData {
    // this should probably be it's own serializer instead but meh
    pub fn update_names(&mut self) {
        self.levels.par_iter_mut().for_each(|level_data| {
            level_data.name = LevelName::from_u32(level_data.id).unwrap_or_default();
        });
    }
    pub fn process_map_data(&mut self) {
        self.levels.par_iter_mut().for_each(|level_data| {
            if level_data.level_image.map_grid.is_none() {
                let map_grid = mapgrid::level_data_to_walkable(level_data);
                let edge_grid = mapgrid::level_data_to_edges(&map_grid); // if you find this slow, do a release build
                let pois: Vec<POI> = pois::get_preset_pois(level_data);
                level_data.level_image.map_grid = Some(map_grid);
                level_data.level_image.map_edges = Some(edge_grid);
                level_data.level_image.pois = pois;
                level_data.level_image.map_image = None;
            }
        });
    }
}
