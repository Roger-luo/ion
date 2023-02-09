use crate::commands::pkg::package_spec_list;
use clap::parser::ArgMatches;
use clap::{arg, Command};
use ion::config::Config;
use ion::errors::CliResult;
use ion::utils::Julia;

pub fn cli() -> Command {
    Command::new("remove")
        .visible_alias("rm")
        .about("Remove dependencies in the current environment")
        .arg(arg!([PACKAGE] ... "The package to remove"))
        .arg(arg!(-g --global "Remove the package in the global environment"))
        .arg_required_else_help(true)
}

pub fn exec(matches: &ArgMatches) -> CliResult {
    format!("using Pkg; Pkg.rm([{}])", package_spec_list(matches))
        .julia_exec(&Config::read()?, matches.get_flag("global"))?;
    Ok(())
}
