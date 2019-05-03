use std::os::raw::c_double;
use std::ffi::CString;
use std::ffi::CStr;
use std::os::raw::c_schar;

#[link(name = "readline")]
extern "C" {
    fn cos(x: c_double) -> c_double;
    fn readline(prompt: *const c_schar) -> *mut c_schar;
}

fn main() {
    unsafe {
        println!("{}", cos(1.5));
        let prompt = CString::new(">> ").unwrap();
        loop {
            let input = CStr::from_ptr(readline(prompt.as_ptr()));
            let input = input.to_str().expect("input contains invalid unicode");
            if input == "exit" {
                break;
            }

            println!("入力結果: {}", input);
        }
    }
}
