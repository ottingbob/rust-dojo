
rc --crate-type=lib 10-crates/10-1-lib.rs

$ ls lib*
> lib10_1_lib.rlib

rc 10-crates/10-2-extern-crate.rs --extern lib=lib10_1_lib.rlib
