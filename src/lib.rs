#![deny(missing_docs)]

//! This library returns the uptime of the current process on windows and unix-like systems.
//!
//! # Example
//!
//! ```
//! use std::thread::sleep;
//! use std::time::Duration;
//!
//! fn main() {
//!     sleep(Duration::from_secs(2));
//!     assert!(uptimer::get().unwrap() >= Duration::from_secs(2))
//! }
//! ```

use std::time::Duration;
#[cfg(windows)]
use windows::Win32::{
    Foundation::FILETIME,
    System::{
        SystemInformation::GetSystemTime,
        Threading::{GetCurrentProcess, GetProcessTimes},
        Time::SystemTimeToFileTime,
    },
};

/// Returns the uptime of the current process in a blocking way (on unix).
#[cfg(windows)]
pub fn get() -> Option<Duration> {
    let proc = unsafe { GetCurrentProcess() };
    // Here, we don't want to call `is_invalid()` since it checks if it's -1 (the expected pseudo handle).
    if proc.0 == 0 {
        return None;
    }

    let start = unsafe {
        let mut creation_time = FILETIME::default();
        let mut b = FILETIME::default();
        let mut c = FILETIME::default();
        let mut d = FILETIME::default();
        GetProcessTimes(proc, &mut creation_time, &mut b, &mut c, &mut d).ok()?;

        ((creation_time.dwHighDateTime as u64) << 32) | (creation_time.dwLowDateTime as u64)
    };

    let now = unsafe {
        let sys_time = GetSystemTime();
        let mut filetime = FILETIME::default();
        SystemTimeToFileTime(&sys_time, &mut filetime).ok()?;
        ((filetime.dwHighDateTime as u64) << 32) | (filetime.dwLowDateTime as u64)
    };

    let diff = now - start;
    let millis = diff / 10000;

    Some(Duration::from_millis(millis))
}

#[cfg(unix)]
pub fn get() -> Option<Duration> {
    let created = std::fs::metadata("/proc/self").ok()?.modified().ok()?;
    let now = std::time::SystemTime::now();

    now.duration_since(created).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let time = get();
        println!("{:?}", time);
        assert!(time.is_some());
    }
}
