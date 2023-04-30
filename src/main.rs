use nian_cli::{bind::Collect, Bind, Cli, Res};
use rusqlite::Connection;

fn main() -> Res<()> {
    let path = "./f/nian.db";
    let conn = Connection::open(path)?;

    let bind = Bind::read(&conn)?;
    let len = bind.len();
    let c = bind[len - 2].try_into_collect()?;

    dbg!(c);

    let cli = Cli::try_from_env()?;
    cli.run()?;

    dbg!(cli);

    Ok(())
}
