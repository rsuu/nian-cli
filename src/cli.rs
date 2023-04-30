use lexopt::{Arg::*, Parser};

#[derive(Debug)]
pub enum Cli {
    Unknown,

    /// note <arg | opt>
    Note {},

    /// todo <arg | opt>
    Todo {
        text: Option<String>,

        /// <-d | --dne> <id>
        opt_done: Option<i64>,
        /// <-l | --ls> <id>
        opt_ls: Option<i64>,
    },
}

impl Cli {
    pub fn try_from_env() -> crate::Res<Self> {
        let v = &mut lexopt::Parser::from_env();

        while let Some(arg) = v.next()? {
            match arg {
                Short('h') | Long("help") => {
                    todo!()
                }
                Value(cmd) => {
                    let cmd = cmd.to_str().unwrap().to_lowercase();

                    match cmd.as_str() {
                        "todo" => return Self::match_todo(v),

                        _ => unreachable!(),
                    }
                }
                _ => unreachable!(),
            }
        }

        Ok(Self::Unknown)
    }

    pub fn match_todo(v: &mut Parser) -> crate::Res<Self> {
        let (mut text, mut opt_done, mut opt_ls) = (None, None, None);

        while let Some(arg) = v.next()? {
            match arg {
                Short('l') | Long("ls") => {
                    let Some(Value(tmp)) = v.next()? else {anyhow::bail!("")};
                    let id: i64 = tmp.to_str().unwrap().parse().unwrap();

                    opt_ls = Some(id);
                }
                Short('d') | Long("done") => {
                    let Some(Value(tmp)) = v.next()? else {anyhow::bail!("")};
                    let id: i64 = tmp.to_str().unwrap().parse().unwrap();

                    opt_done = Some(id);
                }
                Value(tmp) => {
                    text = Some(tmp.to_str().unwrap().to_string());
                }
                _ => unreachable!(),
            }
        }

        Ok(Self::Todo {
            text,
            opt_done,
            opt_ls,
        })
    }

    pub fn run(&self) -> crate::Res<()> {
        match self {
            Self::Todo { .. } => return self.cmd_todo(),
            _ => unreachable!(),
        }

        Ok(())
    }

    pub fn cmd_todo(&self) -> crate::Res<()> {
        let Self::Todo {
            text,
            opt_done,
            opt_ls,
        } = self
         else {
            anyhow::bail!("")
        };

        match (text, opt_done, opt_ls) {
            (Some(text), None, None) => {
                dbg!(text);
            }

            (None, Some(id), None) => {
                dbg!(id);
            }

            (None, None, Some(id)) => {
                dbg!(id);
            }
            _ => unreachable!(),
        }

        Ok(())
    }
}
