extern crate winapi;

fn main() {
    let title: Vec<u16> = "title\0".encode_utf16().collect();
    let message: Vec<u16> = "message\0".encode_utf16().collect();

    // is this necessary
    unsafe {
        winapi::um::winuser::MessageBoxW(std::ptr::null_mut(), message.as_ptr(), title.as_ptr(), winapi::um::winuser::MB_OK | winapi::um::winuser::MB_ICONINFORMATION);
    }
}
