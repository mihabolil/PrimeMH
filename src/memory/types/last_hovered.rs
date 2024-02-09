use crate::memory::process::D2RInstance;
use derivative::Derivative;

#[allow(unused)]
#[repr(u32)]
#[derive(Debug, Clone, Default, PartialEq)]
pub enum UnitType {
    #[default]
    Player = 0,
    Monster,
    Object,
    Missile,
    Item,
    Tile,
    ServerPlayer,
    ServerMonster,
    ServerObject,
    ServerMissile,
    ServerItem,
    ServerTile,
}


#[repr(C)]
#[derive(Derivative, Debug, Clone)]
#[derivative(Default)]
pub struct LastHovered {
    pub is_hovered: bool,
    pub is_tooltip: bool,
    pub unit_type: UnitType,
    pub unit_id: u32,
}

impl LastHovered {
    pub fn get_hovered(d2rprocess: &D2RInstance) -> LastHovered {
        d2rprocess.read_mem_offset::<LastHovered>(d2rprocess.offsets.hover)
    }
}



