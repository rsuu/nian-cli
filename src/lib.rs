pub type Res<T> = anyhow::Result<T>;

pub mod bind;
pub mod cli;

pub use {bind::Bind, cli::Cli};
