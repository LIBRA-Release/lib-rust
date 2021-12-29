/*
 * author: A. Yasuda
 */

extern crate libc;

use libc::c_char;
use std::ffi::{CString,CStr};
use std::str;

pub mod libra;
    
extern "C" {
    fn ValidateTokenExp(p0: *const c_char) -> *const c_char;
    fn GeneratePassphrase() -> *const c_char;
    fn RefreshPassphrase();
}

/*fn main() {

    println!("the hash: {}", generate_passphrase());
    sched_refresh_passphrase();
    println!("the uuid ({}) of a dummy token",validate_bearer_token("dummy token"));         
    
}*/

fn str2char(_s: &str) -> *const c_char {
    let _tk = CString::new(_s).unwrap();
    _tk.as_ptr() as *const c_char
}

fn char2str(_c: *const c_char) -> String {
    let c_str: &CStr = unsafe {CStr::from_ptr(_c)};
    //to_owned is to convrt &str -> String ay
    //str_slice.to_owned()

    c_str.to_str().unwrap().to_owned()
}

pub fn generate_passphrase() -> String {
    let c_buf: *const c_char = unsafe { GeneratePassphrase()};
    char2str(c_buf)
}

pub fn validate_bearer_token(tk: &str) -> String {
    let c_tk = str2char(tk);
    let c_buf: *const c_char = unsafe { ValidateTokenExp(c_tk) };
    char2str(c_buf)
}

pub fn sched_refresh_passphrase() {
    unsafe {
        RefreshPassphrase()        
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
