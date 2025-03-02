use crate::memory::process::D2RInstance;
use num_derive::FromPrimitive;
use derivative::Derivative;

impl KeyBindings {
    pub fn get_keybindings(d2rprocess: &D2RInstance) -> Self {
        d2rprocess.read_mem_offset::<KeyBindings>(d2rprocess.offsets.keybindings)
    }
}


#[repr(C)]
#[derive(Derivative, Debug, Clone)]
#[derivative(Default)]
pub struct KeyBindings {
    #[derivative(Default(value = "[KeyBinding::default(); 128]"))]
    pub bindings: [KeyBinding; 128]
}


#[derive(Debug, Clone, Copy)]
pub struct KeyBinding {
    pub command: KeyBind,
    _dummy: u16,
    pub key: u16,
    pub is_primary: u16,
    _dummy2: u16,
}

impl Default for KeyBinding {
    fn default() -> KeyBinding {
        KeyBinding {
            command: KeyBind::None,
            _dummy: 0,
            key: 0,
            is_primary: 0,
            _dummy2: 0,
        }
    }
}


#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive)]
pub enum KeyBind {
    CharacterScreen,
    Inventory,
    PartyScreen,
    MessageLog,
    QuestLog,
    Chat,
    HelpScreen,
    Automap,
    CenterAutomap,
    FadeAutomap,
    PartyOnAutomap,
    NamesOnAutomap,
    SkillTree,
    SkillSpeedBar,
    Skill1,
    Skill2,
    Skill3,
    Skill4,
    Skill5,
    Skill6,
    Skill7,
    Skill8,
    ShowBelt,
    UsePotion1,
    UsePotion2,
    UsePotion3,
    UsePotion4,
    SayHelp,
    SayFollowMe,
    SayThisIsForYou,
    SayThanks,
    SaySorry,
    SayBye,
    SayNowYouDie,
    HoldRun,
    ToggleRunWalk,
    StandStill,
    ShowItems,
    ClearScreen,
    SelectPreviousSkill,
    SelectNextSkill,
    ClearMessages,
    ScreenShot,
    ShowPortraits,
    SwapWeapons,
    ToggleMiniMap,
    Skill9,
    Skill10,
    Skill11,
    Skill12,
    Skill13,
    Skill14,
    Skill15,
    Skill16,
    MercenaryScreen,
    SayRetreat,
    OpenMenu,
    Zoom,
    LegacyToggle,
    ForceMove,
    HoradricCube,
    Unknown1,
    Unknown2,
    Unknown3,
    None,
}