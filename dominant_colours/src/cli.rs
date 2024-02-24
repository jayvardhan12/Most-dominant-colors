use clap::{Arg, ArgAction, Command};

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn app() -> clap::Command {
    Command::new("dominant_colours")
        .version(VERSION)
        .author("Alex Chan <alex@alexwlchan.net>")
        .about("Find the dominant colours in an image")
        .arg(
            Arg::new("PATH")
                .help("path to the image to inspect")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("MAX-COLOURS")
                .long("max-colours")
                .help("how many colours to find")
                .value_parser(value_parser!(usize))
                .default_value("5"),
        )
        .arg(
            Arg::new("no-palette")
                .long("no-palette")
                .help("Just print the hex values, not colour previews")
                .action(ArgAction::SetTrue),
        )
}

#[cfg(test)]
mod tests {
    use crate::cli::app;

    // See https://github.com/clap-rs/clap/blob/master/CHANGELOG.md#300---2021-12-31
    #[test]
    fn verify_app() {
        app().debug_assert();
    }
}
