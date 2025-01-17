use std::time::{Duration, Instant};

use crate::memory::gamedata::GameData;

use super::{missile::Missile, skills::{PlayerSkill, Skill}};

#[derive(Debug)]
pub struct BuffTimers {
    pub battle_orders: BuffTimer,
    pub battle_command: BuffTimer,
}

impl Default for BuffTimers {
    fn default() -> Self {
        Self {
            battle_orders: BuffTimer::default(),
            battle_command: BuffTimer::default(),
        }
    }
}

#[allow(unused)]
#[derive(Debug)]
pub struct BuffTimer {
    pub timer: Instant,
    pub expiration: Instant,
    pub level: u8,
    pub buff_type: Missile
}

impl BuffTimer {
    pub fn new(level: u8, buff_type: Missile, synergy1: Option<&PlayerSkill>, synergy2: Option<&PlayerSkill>) -> Self {
        Self {
            timer: Instant::now(),
            expiration: calculate_expiration(&buff_type, level, synergy1, synergy2),
            level,
            buff_type
        }
    }
}

impl Default for BuffTimer {
    fn default() -> Self {
        Self {
            timer: Instant::now(),
            expiration: Instant::now(),
            level: 0,
            buff_type: Missile::Unknown
        }
    }
}


pub fn check_buff_timers(game_data: &GameData, buff_timers: &mut BuffTimers) {
    //has missile collided
    for missile in game_data.missiles.iter() {
        if missile.collided && missile.missile_data.skill_level > 0 {
            // I hate this convention
            if missile.txt_file_no == Missile::Battleorders {
                
                let shout = game_data.player.skills.iter().find(|skill| skill.skill == Skill::Shout);
                let battle_command = game_data.player.skills.iter().find(|skill| skill.skill == Skill::BattleCommand);
                buff_timers.battle_orders = BuffTimer::new(missile.missile_data.skill_level, Missile::Battleorders, shout, battle_command);
                // log::info!("Battle Orders missile collided {:?}", buff_timers.battle_orders);
                // break;
            }
            if missile.txt_file_no == Missile::Battlecommand {
                let shout = game_data.player.skills.iter().find(|skill| skill.skill == Skill::Shout);
                let battle_orders = game_data.player.skills.iter().find(|skill| skill.skill == Skill::BattleOrders);
                buff_timers.battle_command = BuffTimer::new(missile.missile_data.skill_level, Missile::Battlecommand, shout, battle_orders);
                // log::info!("Battle command missile collided {:?}", buff_timers.battle_command);
                // break;
            }
        }
    };
}


fn calculate_expiration(buff_type: &Missile, level: u8, synergy1: Option<&PlayerSkill>, synergy2: Option<&PlayerSkill>) -> Instant {
    match buff_type {
        Missile::Battleorders => {
            let mut duration = match level {
                0 => 0,
                1 => 30,
                2 => 40,
                3 => 50,
                4 => 60,
                5 => 70,
                6 => 80,
                7 => 90,
                8 => 100,
                9 => 110,
                10 => 120,
                11 => 130,
                12 => 140,
                13 => 150,
                14 => 160,
                15 => 170,
                16 => 180,
                17 => 190,
                18 => 200,
                19 => 210,
                20 => 220,
                21 => 230,
                22 => 240,
                23 => 250,
                24 => 260,
                25 => 270,
                26 => 280,
                27 => 290,
                28 => 300,
                29 => 310,
                30 => 320,
                31..=35 => 350,
                36 => 380,
                37 => 390,
                38 => 400,
                39 => 410,
                40 => 420,
                41 => 430,
                42 => 440,
                43 => 450,
                44 => 460,
                45..=u8::MAX => 470
            };
            if synergy1.is_some() {
                let shout: &PlayerSkill = synergy1.unwrap();
                if shout.hard_points > 0 {
                    duration = duration + (shout.hard_points as u64 * 5);
                }
            }
            if synergy2.is_some() {
                let battle_command: &PlayerSkill = synergy2.unwrap();
                if battle_command.hard_points > 0 {
                    duration = duration + (battle_command.hard_points as u64 * 5);
                }
            }
            
            return Instant::now() + Duration::from_secs(duration)
        },
        Missile::Battlecommand => {
            let mut duration = match level {
                0 => 0,
                1 => 30,
                2 => 40,
                3 => 50,
                4 => 60,
                5 => 70,
                6 => 80,
                7 => 90,
                8 => 100,
                9 => 110,
                10 => 120,
                11 => 130,
                12 => 140,
                13 => 150,
                14 => 160,
                15 => 170,
                16 => 180,
                17 => 190,
                18 => 200,
                19 => 210,
                20 => 220,
                20..=24 => 220,
                25..=u8::MAX => 270
            };
            if synergy1.is_some() {
                let shout: &PlayerSkill = synergy1.unwrap();
                if shout.hard_points > 0 {
                    duration = duration + (shout.hard_points as u64 * 5);
                }
            }
            if synergy2.is_some() {
                let battle_orders: &PlayerSkill = synergy2.unwrap();
                if battle_orders.hard_points > 0 {
                    duration = duration + (battle_orders.hard_points as u64 * 5);
                }
            }
            return Instant::now() + Duration::from_secs(duration)
        },
        _ => {
            return Instant::now() + Duration::from_secs(0)
        }
    }
}