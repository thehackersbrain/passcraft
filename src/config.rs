use std::collections::HashMap;
use lazy_static::lazy_static;

pub const CONFIG_NUMFROM: u32 = 0;
pub const CONFIG_NUMTO: u32 = 100;

lazy_static! {
    pub static ref CHAR_MAP: HashMap<char, char> = {
        let mut map = HashMap::new();
        map.insert('e', '3');
        map.insert('a', '4');
        map.insert('o', '0');
        map.insert('i', '!');
        map.insert('i', '1');
        map.insert('l', '1');
        map.insert('a', '@');
        map.insert('s', '$');

        map
    };
}
