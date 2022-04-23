# Rust songs

This is a basic CLI application that allows you to store your favourite songs as a URL to then be opened and played in the your default browser (macOS tested only so far).

This repo is a project extending Rust knowledge and exploring Rust idioms.

## Disclaimer

There are obvious best practices missing here from this repo such as extensive test coverage (there are basics tests but could be improved), CI pipeline, pre-commit linting checks, storing data in a local CSV relative to the application is not production suitable etc. At the time of reading this comment, this repo is not intended to cover those practices that would be default checks on systems going to production / being used by others.

## Running

If you are interested in running this locally, the following requirements are

- Install Rust
- Storing a song via `create-song` subcommand
- Playing a song via `play-song` subcommand

### Rust

Ensure that you have rust + cargo installed - https://www.rust-lang.org/tools/install

### Storing a song

Be in the workspace of this repository, and run

```rust
cargo run -- --csv <csv_name.csv> create-song -t <song_name> -u "<song_url>"
```

The quotes around "song_url" are required

### Playing a song

```rust
cargo run -- --csv <csv_name.csv> play-song -t <song_name>
```

## Structure

Currently there is a lib called `songs` that stores the domain model of the application and `song-cli` that is the
entrance to interacting with the domain. There are improvements that could be made here (e.g. having the inbound / outbound ports, traits in Rust, be part of the lib and then having the binary import + implement these traits rather than having to redefine per application).

## Known issues

Currently the `has_headers` in the `build_writer` method has to be set to true for the first song save, then subsequently set to false for further records for the CSV format to be correct. It's annonying and should be fixed.
