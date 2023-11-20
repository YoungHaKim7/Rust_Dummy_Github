use clap::Subcommand;

#[derive(Subcommand)]
pub enum Ghk {
    Pr,
    Auth,
}
