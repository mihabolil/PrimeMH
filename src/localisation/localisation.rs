use std::collections::HashMap;

use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::settings::Locales;

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct LocalisationEntry {
    id: u32,
    Key: String,
    enUS: String,
    zhTW: String,
    deDE: String,
    esES: String,
    frFR: String,
    itIT: String,
    koKR: String,
    plPL: String,
}

pub struct Localisation {
    pub item_gems: HashMap<String, LocalisationEntry>,
    pub item_modifiers: HashMap<String, LocalisationEntry>,
    pub item_nameaffixes: HashMap<String, LocalisationEntry>,
    pub item_names: HashMap<String, LocalisationEntry>,
    pub item_runes: HashMap<String, LocalisationEntry>,
    pub levels: HashMap<String, LocalisationEntry>,
    pub mercenaries: HashMap<String, LocalisationEntry>,
    pub monsters: HashMap<String, LocalisationEntry>,
    pub npcs: HashMap<String, LocalisationEntry>,
    pub objects: HashMap<String, LocalisationEntry>,
    pub quests: HashMap<String, LocalisationEntry>,
    pub shrines: HashMap<String, LocalisationEntry>,
    pub skills: HashMap<String, LocalisationEntry>,
}

impl Localisation {
    pub fn get_npc_name(&self, key_name: String, locale: &Locales) -> Option<&String> {
        

        let new_key_name: String = key_name.chars()
            .filter(|&c| !c.is_digit(10) && !c.is_whitespace())
            .collect();
        
        match self.npcs.get(&new_key_name.to_lowercase()) {
            Some(npc) => {
                match locale {
                    Locales::enUS => return Some(&npc.enUS),
                    Locales::zhTW => return Some(&npc.zhTW),
                    Locales::deDE => return Some(&npc.deDE),
                    Locales::esES => return Some(&npc.esES),
                    Locales::frFR => return Some(&npc.frFR),
                    Locales::itIT => return Some(&npc.itIT),
                    Locales::koKR => return Some(&npc.koKR),
                    Locales::plPL => return Some(&npc.plPL),
                    Locales::enBG => return Some(&npc.enUS),
                    Locales::Unknown => return Some(&npc.enUS),
                }
            }
            None => {
                log::error!("key_name {:?}", key_name);
                return None;
            }
        };
        
    }
}


fn parse_json_bytes<T: DeserializeOwned>(data: &str) -> Vec<T> {
    serde_json::from_str(data.trim_start_matches("\u{feff}")).expect("Unable to parse JSON")
}

pub fn load_localisation_data() -> Localisation {
    let item_gems_data: Vec<LocalisationEntry> = parse_json_bytes(include_str!("./reference/item-gems.json"));
    let item_modifiers_data: Vec<LocalisationEntry> = parse_json_bytes(include_str!("./reference/item-modifiers.json"));
    let item_nameaffixes_data: Vec<LocalisationEntry> = parse_json_bytes(include_str!("./reference/item-nameaffixes.json"));
    let item_names_data: Vec<LocalisationEntry> = parse_json_bytes(include_str!("./reference/item-names.json"));
    let item_runes_data: Vec<LocalisationEntry> = parse_json_bytes(include_str!("./reference/item-runes.json"));
    let levels_data: Vec<LocalisationEntry> = parse_json_bytes(include_str!("./reference/levels.json"));
    let mercenaries_data: Vec<LocalisationEntry> = parse_json_bytes(include_str!("./reference/mercenaries.json"));
    let monsters_data: Vec<LocalisationEntry> = parse_json_bytes(include_str!("./reference/monsters.json"));
    let npcs_data: Vec<LocalisationEntry> = parse_json_bytes(include_str!("./reference/npcs.json"));
    let objects_data: Vec<LocalisationEntry> = parse_json_bytes(include_str!("./reference/objects.json"));
    let quests_data: Vec<LocalisationEntry> = parse_json_bytes(include_str!("./reference/quests.json"));
    let shrines_data: Vec<LocalisationEntry> = parse_json_bytes(include_str!("./reference/shrines.json"));
    let skills_data: Vec<LocalisationEntry> = parse_json_bytes(include_str!("./reference/skills.json"));

    let item_gems: HashMap<String, LocalisationEntry> = vec_to_hashmap(item_gems_data);
    let item_modifiers: HashMap<String, LocalisationEntry> = vec_to_hashmap(item_modifiers_data);
    let item_nameaffixes: HashMap<String, LocalisationEntry> = vec_to_hashmap(item_nameaffixes_data);
    let item_names: HashMap<String, LocalisationEntry> = vec_to_hashmap(item_names_data);
    let item_runes: HashMap<String, LocalisationEntry> = vec_to_hashmap(item_runes_data);
    let levels: HashMap<String, LocalisationEntry> = vec_to_hashmap(levels_data);
    let mercenaries: HashMap<String, LocalisationEntry> = vec_to_hashmap(mercenaries_data);
    let monsters: HashMap<String, LocalisationEntry> = vec_to_hashmap(monsters_data);
    let npcs: HashMap<String, LocalisationEntry> = vec_to_hashmap(npcs_data);
    let objects: HashMap<String, LocalisationEntry> = vec_to_hashmap(objects_data);
    let quests: HashMap<String, LocalisationEntry> = vec_to_hashmap(quests_data);
    let shrines: HashMap<String, LocalisationEntry> = vec_to_hashmap(shrines_data);
    let skills: HashMap<String, LocalisationEntry> = vec_to_hashmap(skills_data);

    Localisation { item_gems, item_modifiers, item_nameaffixes, item_names, item_runes, levels, mercenaries, monsters, npcs, objects, quests, shrines, skills }
}

fn vec_to_hashmap(file_data: Vec<LocalisationEntry>) -> HashMap<String, LocalisationEntry> {
    let mut hashmap: HashMap<String, LocalisationEntry> = HashMap::new();
    for entry in file_data {
        hashmap.insert(entry.Key.clone().to_lowercase().replace("-",""), entry);
    }
    hashmap
}