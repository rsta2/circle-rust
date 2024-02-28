use core::ffi::c_char;
use core::mem::transmute;

// See: https://github.com/rust-lang/rust/issues/101674#issue-1368845166
pub fn str_as_cstr_to_buf(s: &str, buf: &mut [c_char]) {
    if s.contains('\0') {
        panic!("&str contains null terminator");
    }

    // Safety: c_char is u8 or i8, which are both always valid
    let char_buf: &[c_char] = unsafe { transmute(s.as_bytes()) };
    buf[0..s.len()].copy_from_slice(char_buf);

    buf[s.len()] = b'\0' as c_char;
}
