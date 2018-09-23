extern crate libc;
extern crate libloading;

//use libc;
//use libloading;
use std::path::PathBuf;
use std::thread;

#[cfg(debug_assertions)]
const CARGO_PROFILE: &str = "debug";

#[cfg(not(debug_assertions))]
const CARGO_PROFILE: &str = "release";

/// Dynamic link library prefix
#[cfg(unix)]
const PLATFORM_FILE_PREFIX: &str = "lib";
/// Dynamic link library prefix
#[cfg(windows)]
const PLATFORM_FILE_PREFIX: &str = "";
/// Dynamic link library file extension specific to the platform
#[cfg(any(target_os = "macos", target_os = "ios"))]
const PLATFORM_FILE_EXTENSION: &str = "dylib";
/// Dynamic link library file extension specific to the platform
#[cfg(all(unix, not(any(target_os = "macos", target_os = "ios"))))]
const PLATFORM_FILE_EXTENSION: &str = "so";
/// Dynamic link library file extension specific to the platform
#[cfg(windows)]
const PLATFORM_FILE_EXTENSION: &str = "dll";

/// Creates a platform-specific file path
fn create_library_path(name: &str) -> PathBuf {
    let mut path = PathBuf::new();
    path.push("target");
    path.push(CARGO_PROFILE);
    path.push("deps");
    path.push(PLATFORM_FILE_PREFIX.to_string() + name);
    path.set_extension(PLATFORM_FILE_EXTENSION);
    path
}

const ENTRYPOINT: &str = "entrypoint";
type Entrypoint = unsafe extern "C" fn();

fn main() {
    let num_threads = 1;
    let num_iters = 10000;
    println!(
        "spawn {} threads for {} iteractions",
        num_threads, num_iters
    );
    let mut threads = Vec::new();
    for _t in 0..num_threads {
        threads.push(thread::spawn(move || {
            let _tid = thread::current().id();
            for _i in 0..num_iters {
                {
                    //println!("{:#5} {:?}", _i, _tid);
                    let path = create_library_path("noop");
                    let os_lib = libloading::os::unix::Library::open(
                        Some(path),
                        /* libc::RTLD_NODELETE | */ libc::RTLD_NOW,
                    ).unwrap();
                    let library = libloading::Library::from(os_lib);
                    unsafe {
                        let entrypoint: libloading::Symbol<Entrypoint> =
                            match library.get(ENTRYPOINT.as_bytes()) {
                                Ok(s) => s,
                                Err(e) => panic!(
                                    "{:?} Unable to find {:?} in program {}",
                                    e, ENTRYPOINT, "noop"
                                ),
                            };
                        entrypoint();
                    }
                }
            }
        }));
    }

    for thread in threads {
        thread.join().unwrap();
    }
    println!("Done");
}