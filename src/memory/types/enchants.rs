use crate::memory::structs::MonsterData;

#[repr(u8)]
#[allow(unused)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MonsterEnchants {
    Unknown = 0,
    ExtraStrong = 5,
    ExtraFast = 6,
    Cursed = 7,
    MagicResistant = 8,
    FireEnchanted = 9,
    LightningEnchanted = 17,
    ColdEnchanted = 18,
    ManaBurn = 25,
    Teleportation = 26,
    SpectralHit = 27,
    StoneSkin = 28,
    MultipleShots = 29,
}

impl TryFrom<u8> for MonsterEnchants {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            5 => Ok(MonsterEnchants::ExtraStrong),
            6 => Ok(MonsterEnchants::ExtraFast),
            7 => Ok(MonsterEnchants::Cursed),
            8 => Ok(MonsterEnchants::MagicResistant),
            9 => Ok(MonsterEnchants::FireEnchanted),
            17 => Ok(MonsterEnchants::LightningEnchanted),
            18 => Ok(MonsterEnchants::ColdEnchanted),
            25 => Ok(MonsterEnchants::ManaBurn),
            26 => Ok(MonsterEnchants::Teleportation),
            27 => Ok(MonsterEnchants::SpectralHit),
            28 => Ok(MonsterEnchants::StoneSkin),
            29 => Ok(MonsterEnchants::MultipleShots),
            _ => Err(""),
        }
    }
}

pub fn get_monster_enchants(monster_data: &MonsterData) -> Vec<MonsterEnchants> {
    let enchants: Vec<MonsterEnchants> = monster_data
        .mon_u_mod
        .iter()
        .filter_map(|&mod_id| MonsterEnchants::try_from(mod_id).ok())
        .collect();


    enchants
}