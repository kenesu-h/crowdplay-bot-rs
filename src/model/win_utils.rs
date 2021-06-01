use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use winapi::um::winuser::{GetForegroundWindow, GetWindowTextW};

// If I didn't find the StackOverflow posts for these two functions, I don't
// think there's any way I would have been able to do something as simple as
// retrieve the title of the currently focused window, so all credits go to
// those that asked and answered them.

// Copied from https://stackoverflow.com/a/48587463.
unsafe fn u16_ptr_to_string(ptr: *const u16) -> OsString {
  let len: usize = (0..).take_while(|&i| *ptr.offset(i) != 0).count();
  let slice: &[u16] = std::slice::from_raw_parts(ptr, len);

  return OsString::from_wide(slice);
}

// Effectively copied from https://stackoverflow.com/q/54962557
pub fn get_focused_window() -> OsString {
  let name: Vec<u16> = Vec::with_capacity(1024);
  let ptr: *const u16 = name.as_ptr();

  unsafe {
    GetWindowTextW(GetForegroundWindow(), ptr as *mut u16, 1024);
    return u16_ptr_to_string(ptr);
  };
}