use clap::{Args, Subcommand};

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Create a song
    CreateSong(CreateSong),
    // To be added in future
    // UpdateSong(UpdateSong),
    // DeleteSong(DeleteSong),
    PlaySong(PlaySong),
}

#[derive(Args, Debug, Clone)]
pub struct CreateSong {
    /// The title of the song
    #[clap(short, long)]
    pub title: String,

    /// The url of the song
    #[clap(short, long)]
    pub url: String,
}

// These are to be added in future
// #[derive(Args, Debug)]
// pub struct UpdateSong {
//     /// The title of the song
//     #[clap(short, long)]
//     pub title: String,

//     /// The url of the song
//     #[clap(short, long)]
//     pub url: String,
// }

// #[derive(Args, Debug)]
// pub struct DeleteSong {
//     /// The title of the song
//     #[clap(short, long)]
//     pub title: String,
// }

#[derive(Args, Debug)]
pub struct PlaySong {
    /// The title of the song
    #[clap(short, long)]
    pub title: String,
}
