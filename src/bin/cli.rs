use clap::{Arg, Command};
use crate_registry::command::{create_user, delete_user, list_users};

extern crate crate_registry;

fn main() {
    let matches = Command::new("crate_reg")
        .version("1.0")
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("users")
                .about("User administration commands")
                .arg_required_else_help(true)
                .subcommand(
                    Command::new("create")
                        .about("Create a new user")
                        .arg(Arg::new("username").required(true))
                        .arg(Arg::new("password").required(true))
                        .arg(
                            Arg::new("roles")
                                .required(true)
                                .num_args(1..)
                                .value_delimiter(','),
                        ),
                )
                .subcommand(Command::new("list").about("List existing users"))
                .subcommand(
                    Command::new("delete").about("Delete an existing user").arg(
                        Arg::new("id")
                            .required(true)
                            .value_parser(clap::value_parser!(i32)),
                    ),
                ),
        )
        .get_matches();

    if let Some(("users", matches)) = matches.subcommand() {
        match matches.subcommand() {
            Some(("create", matches)) => create_user(
                matches.get_one::<String>("username").unwrap().to_owned(),
                matches.get_one::<String>("password").unwrap().to_owned(),
                matches
                    .get_many::<String>("roles")
                    .unwrap()
                    .map(|v| v.to_string())
                    .collect(),
            ),
            Some(("list", _)) => list_users(),
            Some(("delete", matches)) => delete_user(*matches.get_one::<i32>("id").unwrap()),
            _ => {}
        }
    }
}
