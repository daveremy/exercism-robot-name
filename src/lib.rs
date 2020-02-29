use lazy_static::lazy_static;
use rand::Rng;
use std::collections::HashSet;
use std::sync::Mutex;

lazy_static! {
    static ref ROBOT_NAMES: Mutex<HashSet<String>> = Mutex::new(HashSet::new());
}
pub struct Robot {
    name: String,
}

impl Robot {
    pub fn new() -> Self {
        Robot {
            name: Self::unique_name(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn reset_name(&mut self) {
        self.name = Self::unique_name();
        let mut names = ROBOT_NAMES.lock().unwrap();
        names.remove(&self.name);
    }

    fn unique_name() -> String {
        let mut names = ROBOT_NAMES.lock().unwrap();
        loop {
            let name = Self::gen_name();
            if !names.contains(&name) {
                names.insert(name.to_string());
                break name;
            }
        }
    }

    fn gen_name() -> String {
        let mut rng = rand::thread_rng();
        let letter1: char = rng.gen_range(b'A', b'Z') as char;
        let letter2: char = rng.gen_range(b'A', b'Z') as char;
        let number: u32 = rng.gen_range(0, 1000);
        format!("{}{}{:03}", letter1, letter2, number)
    }
}
