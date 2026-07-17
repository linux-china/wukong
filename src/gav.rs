use crate::gav_cli::clap_app::build_gav_app;

mod gav_cli;

fn main() {
    let mut app = build_gav_app();
    let matches = app.clone().get_matches();
    if let Some((command, command_matches)) = matches.subcommand() {
        match command {
            "add" => gav_cli::gav_add(command_matches),
            "remove" => gav_cli::gav_remove(command_matches),
            "tree" => gav_cli::dependency_tree(command_matches),
            &_ => println!("Unknown command"),
        }
    } else {
        app.print_help().unwrap();
    }
}
