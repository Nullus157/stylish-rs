#[cfg(not(feature = "_tests"))]
compile_error!("please test with --all-features");

#[cfg(feature = "_tests")]
#[path = "std/mod.rs"]
mod std;
