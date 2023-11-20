use clap::Subcommand;

#[derive(Subcommand)]
pub enum Pr {
    create,
    List,
}
