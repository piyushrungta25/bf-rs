use std::process::Command;  // Run programs
use assert_cmd::prelude::*; // Add methods on commands
use std::fs::File;
use std::io::Read;

fn test_helper(file_name: &str) -> Result<(), Box<std::error::Error>> {
    let mut target_str = String::new();
    let _ = File::open(format!("{}.out", file_name))?.read_to_string(&mut target_str);

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.arg(file_name);
    cmd.assert()
        .success()
        .stdout(target_str);
	Ok(())
}

#[test]
fn test_hello_world() -> Result<(), Box<std::error::Error>> {
    test_helper("samples/hello_world.bf")
}

#[test]
fn test_bang_bang() -> Result<(), Box<std::error::Error>> {
    test_helper("samples/bangbang.bf")
}

#[test]
fn test_99_bottles() -> Result<(), Box<std::error::Error>> {
    test_helper("samples/bottles.bf")
}

// run this one with relase build, it takes a long time with debug build
#[ignore]
#[test]
fn test_mandelbrot() -> Result<(), Box<std::error::Error>> {
    test_helper("samples/mandelbrot.bf")
}

#[test]
fn test_quine() -> Result<(), Box<std::error::Error>> {
    test_helper("samples/quine.bf")
}

#[test]
fn test_square_number() -> Result<(), Box<std::error::Error>> {
    test_helper("samples/square_number.bf")
}