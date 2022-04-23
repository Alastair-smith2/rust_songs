use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::{fs::remove_file, process::Command}; // Run programs

fn create_song_command_options(cmd: &mut Command, path: &str) {
    cmd.arg("--csv")
        .arg(path)
        .arg("create-song")
        .arg("-t")
        .arg("Edm3")
        .arg("-u")
        .arg("https://www.youtube.com/watch?v=m9c9WkmVfXY&t=8565s");
}

#[test]
fn song_was_created() -> Result<(), Box<dyn std::error::Error>> {
    let test_path = "test.csv";
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;

    create_song_command_options(&mut cmd, &test_path);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("The song \"Edm3\" was created"));

    remove_file(&test_path)?;
    Ok(())
}

#[test]
fn song_was_played() -> Result<(), Box<dyn std::error::Error>> {
    let test_path = "test.csv";
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;

    create_song_command_options(&mut cmd, &test_path);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("The song \"Edm3\" was created"));

    let mut play_cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    play_cmd
        .arg("--csv")
        .arg(&test_path)
        .arg("play-song")
        .arg("-t")
        .arg("Edm3");
    play_cmd
        .assert()
        .success()
        .stdout(predicate::str::contains("status:"));

    remove_file(&test_path)?;
    Ok(())
}
