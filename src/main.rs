#![allow(unused_variables)]
mod arguments;
mod error;
mod interactive_online;

use arguments::*;

// const HIBP_TOTAL: usize = 16usize.pow(5);

fn main() {
    let args = handle_arguments();

    match args.command {
        Commands::InteractiveFile {
            password_type,
            file,
        } => todo!(),
        Commands::InteractiveOnline => interactive_online::interactive(),
        Commands::Downloader { output } => todo!(),
        Commands::FileCheck {
            password_type,
            password_file,
            file,
            print_passwords,
        } => todo!(),
        Commands::CreateFilter { input, output } => todo!(),
    };
}
