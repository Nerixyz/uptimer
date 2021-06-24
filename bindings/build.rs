fn main() {
    windows::build! {
        Windows::Win32::System::Threading::{GetCurrentProcess, GetProcessTimes},
        Windows::Win32::System::SystemInformation::GetSystemTime,
        Windows::Win32::System::Time::SystemTimeToFileTime,
        Windows::Win32::Foundation::FILETIME,
    }
}
