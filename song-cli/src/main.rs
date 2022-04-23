mod cli;
mod infra;
use std::process::{Command, Output};

use clap::Parser;
use cli::parser::Cli;
use songs::domain::song::Song;

use crate::infra::song_repository::{CliRepository, SongRepository};

fn execute_command(song: Option<Song>) -> Option<Output> {
    Some(
        Command::new("open")
            .arg(song?.get_url())
            .output()
            .expect("Unable to execute open chrome for song"),
    )
}
fn main() {
    let args = Cli::parse();

    println!("The args {:?}", &args);
    let repo = CliRepository::new(args.csv.to_owned());
    match args.command {
        cli::commands::Command::CreateSong(song) => {
            println!("The song {:?}", song);
            match repo.save_song(song) {
                Ok(created_song) => {
                    println!("The song {:?} was created", created_song.get_title());
                }
                Err(err) => {
                    eprintln!(
                        "An error occurred while creating the song {:?}",
                        err.to_string()
                    );
                }
            }
        }
        cli::commands::Command::PlaySong(song_to_play) => {
            let output_result = repo
                .load_song(&song_to_play.title)
                .and_then(|song| Ok(execute_command(song)));

            match output_result {
                Ok(potential_output) => match potential_output {
                    Some(output) => println!("status: {}", output.status),
                    None => eprintln!("No song was found with title {:?}", &song_to_play.title),
                },
                Err(err) => {
                    eprintln!(
                        "An error occurred while playing the song {:?}",
                        err.to_string()
                    );
                }
            }
        } // To be added in future
          // Command::UpdateSong(song) => {
          //     println!("The song to update {:?}", song);
          // }
          // Command::DeleteSong(song) => {
          //     println!("The song to delete {:?}", song);
          // }
    }
}
