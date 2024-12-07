use device_query::Keycode;
use serde::Deserialize;
use std::fmt;
use std::str::FromStr;
use serde::{de, Deserializer, Serialize, Serializer};

#[allow(unused)]
#[derive(Debug, Clone)]
pub struct HotKey {
    pub alt: bool,
    pub ctrl: bool,
    pub shift: bool,
    pub win: bool,
    pub key: Keycode,
}

impl HotKey {
    pub fn new(key: Keycode, ctrl: bool) -> Self {
        Self { alt: false, ctrl: ctrl, shift: false, win: false, key }
    }

    pub fn pressed(self, keys: &Vec<Keycode>) -> bool {
        if self.alt {
            if !keys.contains(&Keycode::LAlt) && !keys.contains(&Keycode::RAlt) {
                return false
            }
        }
        if self.ctrl {
            if !keys.contains(&Keycode::LControl) && !keys.contains(&Keycode::RControl) {
                return false
            }
        }
        if self.shift {
            if !keys.contains(&Keycode::LShift) && !keys.contains(&Keycode::RShift) {
                return false
            }
        }
        if self.win {
            if !keys.contains(&Keycode::LMeta) && !keys.contains(&Keycode::RMeta) {
                return false
            }
        }
        if keys.contains(&self.key) {
            // let mut asdf: Vec<&str> = vec![];
            // if self.alt {
            //     asdf.push("!");
            // }
            // if self.ctrl {
            //     asdf.push("^");
            // }
            // if self.shift {
            //     asdf.push("+");
            // }
            // if self.win {
            //     asdf.push("#");
            // }
            // log::debug!("Pressed {}{:?}", asdf.join(""), &self.key);
            return true
        }
        false
    }
}

impl FromStr for HotKey {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        
        let alt = s.contains('!');
        let ctrl = s.contains('^');
        let shift = s.contains('+');
        let win = s.contains('#');
        let cleaned_str: String = s
            .chars()
            .filter(|&c| !"+#!^".contains(c))  // Keep characters that are not +#!^
            .collect();
        match Keycode::from_str(&cleaned_str) {
            Ok(key) => {
                Ok(HotKey {
                    alt, ctrl, shift, win, key
                })
            },
            Err(_) => Err(String::from("KeyCode invalid!")),
        }
        
    }

    
}

impl fmt::Display for HotKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}{}{}{:?}",
            if self.alt { "!" } else { "" },
            if self.ctrl { "^" } else { "" },
            if self.shift { "+" } else { "" },
            if self.win { "#" } else { "" },
            self.key
        )
    }
}


impl<'de> Deserialize<'de> for HotKey {
    fn deserialize<D>(deserializer: D) -> Result<HotKey, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        HotKey::from_str(&s).map_err(|_| de::Error::custom(format!("{}", "Invalid hotkey")))
    }
}

impl Serialize for HotKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Create a string representation of the HotKey
        let hotkey_str = format!(
            "{}{}{}{}{:?}",
            if self.alt { "!" } else { "" },
            if self.ctrl { "^" } else { "" },
            if self.shift { "+" } else { "" },
            if self.win { "#" } else { "" },
            self.key
        );
        
        // Serialize the string
        serializer.serialize_str(&hotkey_str)
    }
}