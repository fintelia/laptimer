use std::time::Duration;
use std::{sync::Mutex, time::Instant};

lazy_static::lazy_static! {
    static ref START: Mutex<Option<Instant>> = Mutex::new(None);
}

pub fn println<'a>(s: &'a str) {
    let mut start = START.lock().unwrap();

    let elapsed;
    match *start {
        Some(ref mut instant) => {
            elapsed = instant.elapsed();
            *instant = instant.checked_add(elapsed).unwrap();
        }
        None => {
            *start = Some(Instant::now());
            elapsed = Duration::from_secs(0);
        }
    }
    println!("[{:?}]: {}", elapsed, s);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
