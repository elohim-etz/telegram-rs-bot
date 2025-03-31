use std::collections::HashMap;
use std::sync::Mutex;
use chrono::{Utc, Duration};
use lazy_static::lazy_static;
use teloxide::types::User;

lazy_static! {
    static ref PERCENT_CACHE: Mutex<HashMap<(i64, String), (u8, chrono::DateTime<Utc>)>> = 
        Mutex::new(HashMap::new());
}

pub fn get_daily_percentage(user: &User, command: &str) -> u8 {
    let key = (user.id.0 as i64, command.to_string()); 
    let now = Utc::now();
    
    let mut cache = PERCENT_CACHE.lock().unwrap();
    
    if let Some((percent, timestamp)) = cache.get(&key) {
        if now.signed_duration_since(*timestamp) < Duration::hours(24) {
            return *percent;
        }
    }
    
    let new_percent = (rand::random::<u8>() % 100) + 1;
    cache.insert(key, (new_percent, now));
    
    new_percent
}