Recreate segmentation fault when loading dynamic objects on Linux
===

The purpose of this repo is to show two things

1. Differences in behavior between MacOS and Linux
  - On MacOS the init/fini are called each time the thread load/unloads the module
  -  On Linux init is called at least once on start and at least once on test ext but also sporatically
    during test execution
2. Segmentation fault when running on Linux
  - Note: segmentation fault does not occur 100% of the time
 
Building and running
===

This project contains a dynamic module built as a member of the workspace.  This module is not built by `cargo run` so you must always issue `cargo build` first.

```sh
cargo build
cargo run
```

Sample output
===

Linux:
```
cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s                                                                                                
     Running `target/debug/dynamic_module_segfault`
spawn 100 threads for 10000 iteractions
.init
.fini
.init
Segmentation fault (core dumped)
```
