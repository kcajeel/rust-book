// This doesn't work on my computer. 
// Windows is generally unfriendly to C so it might be a mingw issue

#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}
