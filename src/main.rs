use std::time::{SystemTime, UNIX_EPOCH};


fn main() {
    let time = SystemTime::now();
let duration = time.duration_since(UNIX_EPOCH).unwrap();
let seconds = duration.as_secs();
let nanos = duration.subsec_nanos();

// Ручне форматування
let days = seconds / 86400;
let hours = (seconds % 86400) / 3600;
let minutes = (seconds % 3600) / 60;
let secs = seconds % 60;

println!("{} днів {} годин {} хвилин {} секунд {} наносекунд", 
         days, hours, minutes, secs, nanos);
}
