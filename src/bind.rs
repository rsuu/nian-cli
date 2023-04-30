use crate::Cli;
use miniserde::{json, Deserialize, Serialize};
use rusqlite::{params, Connection};

#[derive(Debug)]
pub struct Bind {
    id: i64,
    dream_id: Option<i64>,
    images: Option<String>,
    content: Option<String>,
    create_at: Option<i64>,
    update_at: Option<i64>,
    at_top: i32,
    hide: i32,
    typ: i32,
    too_big: i32,
    comment_count: Option<i32>,
    b_ext1: i32,
    b_ext2: i32,
    l_ext1: Option<i32>,
    l_ext2: Option<i32>,
    s_ext1: Option<String>,
    s_ext2: Option<String>,
    s_ext3: Option<String>,
    s_ext4: Option<String>,
    i_ext1: i32,
    i_ext2: i32,
}

impl Bind {
    pub fn from_cli(v: &Cli) -> crate::Res<Self> {
        todo!();
    }

    pub fn write(&self, conn: &Connection) -> crate::Res<()> {
        conn.execute(
            "
CREATE TABLE IF NOT EXISTS STEP (
  _id INTEGER PRIMARY KEY,
  DREAM_ID INTEGER,
  IMAGES TEXT,
  CONTENT TEXT,
  CREATE_AT INTEGER,
  UPDATE_AT INTEGER,
  AT_TOP INTEGER NOT NULL,
  HIDE INTEGER NOT NULL,
  TYPE INTEGER NOT NULL,
  TOO_BIG INTEGER NOT NULL,
  COMMENT_COUNT INTEGER,
  B_EXT1 INTEGER NOT NULL,
  B_EXT2 INTEGER NOT NULL,
  L_EXT1 INTEGER,
  L_EXT2 INTEGER,
  S_EXT1 TEXT,
  S_EXT2 TEXT,
  S_EXT3 TEXT,
  S_EXT4 TEXT,
  I_EXT1 INTEGER NOT NULL,
  I_EXT2 INTEGER NOT NULL
)",
            [],
        )?;

        conn.execute(
            "
INSERT INTO STEP (
  _id,
  DREAM_ID,
  IMAGES,
  CONTENT,
  CREATE_AT,
  UPDATE_AT,
  AT_TOP,
  HIDE,
  TYPE,
  TOO_BIG,
  COMMENT_COUNT,
  B_EXT1,
  B_EXT2,
  L_EXT1,
  L_EXT2,
  S_EXT1,
  S_EXT2,
  S_EXT3,
  S_EXT4,
  I_EXT1,
  I_EXT2
) VALUES (
  ?0,
  ?1,
  ?2,
  ?3,
  ?4,
  ?5,
  ?6,
  ?7,
  ?8,
  ?9,
  ?10,
  ?11,
  ?12,
  ?13,
  ?14,
  ?15,
  ?16,
  ?17,
  ?18,
  ?19,
  ?20
)",
            params![
                self.id,
                self.dream_id,
                self.images,
                self.content,
                self.create_at,
                self.update_at,
                self.at_top,
                self.hide,
                self.typ,
                self.too_big,
                self.comment_count,
                self.b_ext1,
                self.b_ext2,
                self.l_ext1,
                self.l_ext2,
                self.s_ext1,
                self.s_ext2,
                self.s_ext3,
                self.s_ext4,
                self.i_ext1,
                self.i_ext2,
            ],
        )?;

        Ok(())
    }

    pub fn read(conn: &Connection) -> crate::Res<Vec<Self>> {
        let mut stmt = conn.prepare("SELECT * FROM STEP")?;
        let step_iter = stmt.query_map([], |row| {
            Ok(Self {
                id: row.get(0)?,
                dream_id: row.get(1)?,
                images: row.get(2)?,
                content: row.get(3)?,
                create_at: row.get(4)?,
                update_at: row.get(5)?,
                at_top: row.get(6)?,
                hide: row.get(7)?,
                typ: row.get(8)?,
                too_big: row.get(9)?,
                comment_count: row.get(10)?,
                b_ext1: row.get(11)?,
                b_ext2: row.get(12)?,
                l_ext1: row.get(13)?,
                l_ext2: row.get(14)?,
                s_ext1: row.get(15)?,
                s_ext2: row.get(16)?,
                s_ext3: row.get(17)?,
                s_ext4: row.get(18)?,
                i_ext1: row.get(19)?,
                i_ext2: row.get(20)?,
            })
        })?;

        let mut res = vec![];
        for f in step_iter.into_iter() {
            res.push(f?);
        }

        Ok(res)
    }

    pub fn try_from_collect(v: Collect) -> crate::Res<Self> {
        let mut res = Self::default();
        res.id = v.id;
        res.content = Some(json::to_string(&v.inner));
        res.create_at = v.create_at;
        res.update_at = v.update_at;

        Ok(res)
    }

    pub fn try_into_collect(&self) -> crate::Res<Collect> {
        let tmp = self.clone();
        let j = tmp.content.clone().unwrap();
        let inner: InnerCollect = json::from_str(&j)?;
        Ok(Collect {
            images: tmp.images.clone(),
            id: tmp.id,
            typ: tmp.typ,
            create_at: tmp.create_at,
            update_at: tmp.update_at,
            inner,
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Ty {
    Card,
    DialogL,
    DialogR,
    Ledger,
    Note,
    Timer,
    Todo,
    Url,
    Other,
    Collect,
}

#[derive(Debug)]
pub struct Collect {
    inner: InnerCollect,

    id: i64,
    typ: i32,
    images: Option<String>,
    create_at: Option<i64>,
    update_at: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InnerCollect {
    date: Date,
    imageLink: String,
    jumpLink: String,
    pageId: u8,
    rate: f32,
    review: String,
    tags: Vec<String>,
    title: String,
    top: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Date {
    year: u16,
    month: u8,
    day: u8,
}

impl Ty {
    fn from_u32(v: &u32) -> Self {
        match *v {
            0 => Self::Note,
            101 => Self::Todo,
            401 => Self::Ledger,
            405 => Self::DialogL, // left
            406 => Self::DialogR, // right
            408 => Self::Url,
            501 => Self::Card,
            701 => Self::Timer,
            711 => Self::Collect,
            _ => Self::Other,
        }
    }
}

impl Default for Bind {
    fn default() -> Self {
        Self {
            id: -1,
            dream_id: None,
            images: None,
            content: None,
            create_at: None,
            update_at: None,
            at_top: -1,
            hide: -1,
            typ: -1,
            too_big: -1,
            comment_count: None,
            b_ext1: -1,
            b_ext2: -1,
            l_ext1: None,
            l_ext2: None,
            s_ext1: None,
            s_ext2: None,
            s_ext3: None,
            s_ext4: None,
            i_ext1: -1,
            i_ext2: -1,
        }
    }
}
