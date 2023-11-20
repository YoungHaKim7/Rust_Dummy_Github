use clap::Subcommand;

#[derive(Subcommand)]
pub enum Pr {
    /// Create a pull request
    Create,
    /// List pull request
    List,
}

impl Pr {
    pub fn exec(&self) {
        match self {
            Pr::Create => {
                println!("Pr Created")
            }
            Pr::List => {
                println!("List Pressed")
            }
        }
    }
}
