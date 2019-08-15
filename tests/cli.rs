use assert_cmd::prelude::*; // Add methods on commands
use std::fs::File;
use std::io::Read;
use std::process::Command; // Run programs

fn test_helper(file_name: &str, has_input: bool) -> Result<(), Box<std::error::Error>> {
    let mut target_buf = String::new();
    let _ = File::open(format!("{}.out", file_name))?.read_to_string(&mut target_buf)?;

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.arg(file_name);

    let assert = if has_input {
        cmd.with_stdin().path(format!("{}.in", file_name))?.assert()
    } else {
        cmd.assert()
    };

    assert.success().stdout(target_buf);
    Ok(())
}

#[test]
fn test_hello_world() -> Result<(), Box<std::error::Error>> {
    test_helper("samples/hello_world.bf", false)
}

#[test]
fn test_bang_bang() -> Result<(), Box<std::error::Error>> {
    test_helper("samples/bangbang.bf", false)
}

#[test]
fn test_99_bottles() -> Result<(), Box<std::error::Error>> {
    test_helper("samples/bottles.bf", false)
}

// run this one with relase build, it takes a long time with debug build
#[ignore]
#[test]
fn test_mandelbrot() -> Result<(), Box<std::error::Error>> {
    test_helper("samples/mandelbrot.bf", false)
}

#[test]
fn test_quine() -> Result<(), Box<std::error::Error>> {
    test_helper("samples/quine.bf", false)
}

#[test]
fn test_square_number() -> Result<(), Box<std::error::Error>> {
    test_helper("samples/square_number.bf", false)
}

#[test]
fn test_cat() -> Result<(), Box<std::error::Error>> {
    test_helper("samples/cat.bf", true)
}

#[test]
fn test_factor() -> Result<(), Box<std::error::Error>> {
    test_helper("samples/factor.bf", true)
}
