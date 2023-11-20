use clap::Subcommand;

#[derive(Subcommand)]
pub enum Ghk {
    /// Manage pull requests
    Pr,
    /// Login or Logout
    Auth,
}
