use std::process::Command;  // Run programs
use assert_cmd::prelude::*; // Add methods on commands

#[test]
fn hello_world() -> Result<(), Box<std::error::Error>> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.arg("samples/hello.bf");
    cmd.assert()
        .success()
        .stdout("Hello World!\n");

    Ok(())
}