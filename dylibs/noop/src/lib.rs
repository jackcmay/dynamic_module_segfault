

#[no_mangle]
pub extern fn init() {
    println!(".init");
}

#[cfg(target_os = "macos")]
#[link_section = "__DATA,__mod_init_func"]
pub static CONSTRUCTOR: extern fn() = init;

#[cfg(all(unix, not(target_os = "macos")))]
#[link_section = ".ctors"]
pub static CONSTRUCTOR: extern fn() = init;

#[no_mangle]
pub extern fn fini() {
    println!(".fini");
}
#[cfg(target_os = "macos")]
#[link_section = "__DATA,__mod_term_func"]
pub static DESTRUCTOR: extern fn() = fini;

#[cfg(all(unix, not(target_os = "macos")))]
#[link_section = ".dtors"]
pub static DESTRUCTOR: extern fn() = fini;

#[no_mangle]
pub extern "C" fn entrypoint() {
    //println!("noop");
}
