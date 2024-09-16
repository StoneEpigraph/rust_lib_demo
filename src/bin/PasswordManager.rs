use std::collections::HashMap;
use std::marker::PhantomData;

struct Locked;
struct Unlocked;


struct PasswordManager<State = Locked> {
    master_password: String,
    passwords: HashMap<String, String>,
    version: u32,
    state: PhantomData<State>,
}

impl PasswordManager {
    pub fn new(master_password: String) -> Self {
        PasswordManager {
            master_password,
            passwords: Default::default(),
            version: 0,
            state: PhantomData::<Locked>,
        }
    }
}

impl<State> PasswordManager<State> {
    pub fn version(&self) -> &u32 {
        &self.version
    }
}

impl PasswordManager<Locked> {
    pub fn unlock(self, master_password: String) -> PasswordManager<Unlocked> {
        PasswordManager {
            master_password: self.master_password,
            passwords: self.passwords,
            version: self.version + 1,
            state: PhantomData::<Unlocked>,
        }
    }
}

impl PasswordManager<Unlocked> {
    pub fn lock(self, master_password: String) -> PasswordManager<Locked> {
        PasswordManager {
            master_password: self.master_password,
            passwords: self.passwords,
            version: self.version,
            state: PhantomData::<Locked>,
        }
    }

    pub fn list_password(&self) -> &HashMap<String, String> {
        &self.passwords
    }

    pub fn add_password(&mut self, username: String, new_password: String) {
        self.passwords.insert(username, new_password);
    }
}

fn main() {
    let mut manager = PasswordManager::new("123".to_string());
    println!("version: {:?}", manager.version());
    let mut manager = manager.unlock("123".to_string());
    manager.add_password("stone".to_string(), "qepflangy".to_string());
    manager.add_password("stoneepigraph".to_string(), "qepflangy".to_string());
    println!("version: {:?}", manager.version());
    let manager = manager.lock("123".to_string());
    println!("version: {:?}", manager.version());
    let manager = manager.unlock(String::from("123"));
    println!("password list: {:?}", manager.list_password());
    let manager = manager.lock(String::from("123"));
    println!("version: {:?}", manager.version());
}