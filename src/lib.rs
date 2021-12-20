use std::time::Duration;
use std::{sync::Mutex, time::Instant};

lazy_static::lazy_static! {
    static ref START: Mutex<Option<Instant>> = Mutex::new(None);
}

/// Print a message to stdout containing the file and line number, plus the number of
/// milliseconds elapsed since the last call to this macro (or 0 if this is the first call).
#[macro_export]
macro_rules! print {
    () => {
        {
            println!("[{: >8.3}ms] {}:{}", $crate::lap().as_nanos() as f64 * 1e-6, file!(), line!());
        }
    };
}

#[doc(hidden)]
pub fn lap() -> std::time::Duration {
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
    elapsed
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        crate::print!();
        crate::print!();
    }
}
