use std::collections::HashMap;

use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct LocalisationRawFileEntry {
    #[serde(default = "a_default")]
    pub id: u32,
    pub Key: String,
    pub enUS: String,
    pub zhTW: String,
    pub deDE: String,
    pub esES: String,
    pub frFR: String,
    pub itIT: String,
    pub koKR: String,
    pub plPL: String,
    #[serde(default = "blank")]
    pub enBG: String,
}

fn a_default() -> u32{
    1 
}

fn blank() -> String {
    String::new()
}

pub fn parse_json_bytes<T: DeserializeOwned>(data: &str) -> Vec<T> {
    serde_json::from_str(data.trim_start_matches("\u{feff}")).expect("Unable to parse JSON")
}

#[derive(Debug, Deserialize, Serialize, Eq, Hash, PartialEq)]
pub enum LocalisationFiles {
    ItemGemsData,
    ItemModifiersData,
    ItemNameAffixesData,
    ItemNamesData,
    ItemRunesData,
    LevelsData,
    MercData,
    MonstersData,
    NPCsData,
    ObjectsData,
    QuestsData,
    ShrinesData,
    SkillsData,
    PrimeMHData,
}


pub fn load_localisation_files() -> HashMap<LocalisationFiles, Vec<LocalisationRawFileEntry>> {
    let item_gems_data: Vec<LocalisationRawFileEntry> = parse_json_bytes(include_str!("./reference/item-gems.json"));
    let item_modifiers_data: Vec<LocalisationRawFileEntry> = parse_json_bytes(include_str!("./reference/item-modifiers.json"));
    let item_nameaffixes_data: Vec<LocalisationRawFileEntry> = parse_json_bytes(include_str!("./reference/item-nameaffixes.json"));
    let item_names_data: Vec<LocalisationRawFileEntry> = parse_json_bytes(include_str!("./reference/item-names.json"));
    let item_runes_data: Vec<LocalisationRawFileEntry> = parse_json_bytes(include_str!("./reference/item-runes.json"));
    let levels_data: Vec<LocalisationRawFileEntry> = parse_json_bytes(include_str!("./reference/levels.json"));
    let mercenaries_data: Vec<LocalisationRawFileEntry> = parse_json_bytes(include_str!("./reference/mercenaries.json"));
    let monsters_data: Vec<LocalisationRawFileEntry> = parse_json_bytes(include_str!("./reference/monsters.json"));
    let npcs_data: Vec<LocalisationRawFileEntry> = parse_json_bytes(include_str!("./reference/npcs.json"));
    let objects_data: Vec<LocalisationRawFileEntry> = parse_json_bytes(include_str!("./reference/objects.json"));
    let quests_data: Vec<LocalisationRawFileEntry> = parse_json_bytes(include_str!("./reference/quests.json"));
    let shrines_data: Vec<LocalisationRawFileEntry> = parse_json_bytes(include_str!("./reference/shrines.json"));
    let skills_data: Vec<LocalisationRawFileEntry> = parse_json_bytes(include_str!("./reference/skills.json"));
    let primemh_data: Vec<LocalisationRawFileEntry> = parse_json_bytes(include_str!("./reference/primemh.json"));
    let mut localisation_source_files: HashMap<LocalisationFiles, Vec<LocalisationRawFileEntry>> = HashMap::new();
    localisation_source_files.insert(LocalisationFiles::ItemGemsData, item_gems_data);
    localisation_source_files.insert(LocalisationFiles::ItemModifiersData, item_modifiers_data);
    localisation_source_files.insert(LocalisationFiles::ItemNameAffixesData, item_nameaffixes_data);
    localisation_source_files.insert(LocalisationFiles::ItemNamesData, item_names_data);
    localisation_source_files.insert(LocalisationFiles::ItemRunesData, item_runes_data);
    localisation_source_files.insert(LocalisationFiles::LevelsData, levels_data);
    localisation_source_files.insert(LocalisationFiles::MercData, mercenaries_data);
    localisation_source_files.insert(LocalisationFiles::MonstersData, monsters_data);
    localisation_source_files.insert(LocalisationFiles::NPCsData, npcs_data);
    localisation_source_files.insert(LocalisationFiles::ObjectsData, objects_data);
    localisation_source_files.insert(LocalisationFiles::QuestsData, quests_data);
    localisation_source_files.insert(LocalisationFiles::ShrinesData, shrines_data);
    localisation_source_files.insert(LocalisationFiles::SkillsData, skills_data);
    localisation_source_files.insert(LocalisationFiles::PrimeMHData, primemh_data);
    return localisation_source_files;
}