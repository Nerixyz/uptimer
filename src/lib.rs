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
//!     println!("{:?}", uptimer::get());
//! }
//! ```
//!
//! ## Features
//!
//! * `async` enables the `get_async` function.
//!

#[cfg(windows)]
use bindings::Windows::Win32::{
    Foundation::{FILETIME, SYSTEMTIME},
    System::{
        SystemInformation::GetSystemTime,
        Threading::{GetCurrentProcess, GetProcessTimes},
        Time::SystemTimeToFileTime,
    },
};
use std::time::Duration;

/// Returns the uptime of the current process in a blocking way (on unix).
#[cfg(windows)]
pub fn get() -> Option<Duration> {
    let proc = unsafe { GetCurrentProcess() };
    if proc.is_null() {
        return None;
    }

    let start = unsafe {
        let mut a = FILETIME::default();
        let mut b = FILETIME::default();
        let mut c = FILETIME::default();
        let mut d = FILETIME::default();
        let ok = GetProcessTimes(
            proc,
            &mut a as *mut _ as _,
            &mut b as *mut _ as _,
            &mut c as *mut _ as _,
            &mut d as *mut _ as _,
        );
        if ok.as_bool() {
            Some(a)
        } else {
            None
        }
    }
    .map(|t| ((t.dwHighDateTime as u64) << 32) | (t.dwLowDateTime as u64))?;
    let now = unsafe {
        let mut sys_time = SYSTEMTIME::default();
        GetSystemTime(&mut sys_time);
        let mut filetime = FILETIME::default();
        let ok = SystemTimeToFileTime(&mut sys_time, &mut filetime);
        if ok.as_bool() {
            Some(filetime)
        } else {
            None
        }
    }
    .map(|t| ((t.dwHighDateTime as u64) << 32) | (t.dwLowDateTime as u64))?;

    let diff = now - start;
    let millis = diff / 10000;

    Some(Duration::from_millis(millis))
}

#[cfg(unix)]
pub fn get() -> Option<Duration> {
    let pid = std::process::id();
    let output = std::process::Command::new("ps")
        .arg("-o")
        .arg("etimes")
        .arg("-p")
        .arg(pid.to_string())
        .arg("--no-headers")
        .output()
        .ok()?;
    let output = String::from_utf8(output.stdout).ok()?;
    let elapsed_time_sec = output.trim();

    let secs = elapsed_time_sec.parse::<u64>().ok()?;

    Some(Duration::from_secs(secs))
}

/// Returns the uptime of the current process asynchronously (on unix).
#[cfg(all(windows, feature = "async"))]
pub fn get_async() -> std::future::Ready<Option<Duration>> {
    std::future::ready(get())
}

#[cfg(all(unix, feature = "async"))]
pub async fn get_async() -> Option<Duration> {
    let pid = std::process::id();
    let output = tokio::process::Command::new("ps")
        .arg("-o")
        .arg("etimes")
        .arg("-p")
        .arg(pid.to_string())
        .arg("--no-headers")
        .output()
        .await
        .ok()?;
    let output = String::from_utf8(output.stdout).ok()?;
    let elapsed_time_sec = output.trim();

    let secs = elapsed_time_sec.parse::<u64>().ok()?;

    Some(Duration::from_secs(secs))
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
