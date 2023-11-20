use clap::Subcommand;

use crate::pr::Pr;

#[derive(Subcommand)]
pub enum Ghk {
    /// Manage pull requests
    Pr {
        #[command(subcommand)]
        command: Pr,
    },
    /// Login or Logout
    Auth,
}
