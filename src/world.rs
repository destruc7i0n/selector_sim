use serde::{Serialize, Deserialize};

use std::collections::HashMap;

use nbt::{de, Endianness};

use rusty_leveldb::{DB, LdbIterator};

use crate::scoreboard::{RawScoreboard, Scoreboard};

pub type EntityMap = HashMap<i64, Entity>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Entity {
  #[serde(rename = "CustomName")] pub name: Option<String>,
  #[serde(rename = "UniqueID")] pub id: i64,
  #[serde(rename = "identifier")] pub entity_type: String,
  #[serde(rename = "Pos")] pub pos: Vec<f32>,
  #[serde(rename = "Tags")] pub tags: Option<Vec<String>>,
  #[serde(rename = "definitions")] pub definitions: Vec<String>
}

#[allow(dead_code)]
pub struct World {
  db: DB,
  pub entities: EntityMap,
  pub scoreboard: Scoreboard
}

impl World {
  pub fn new (world_name: &str) -> World {
    let mut db = World::get_db(world_name);

    let entities = World::get_entities(&mut db);
    let scoreboard = World::get_scoreboard(&mut db, &entities);

    World {
      db,
      entities,
      scoreboard,
    }
  }

  fn get_db (world_name: &str) -> DB {
    let mut opt = rusty_leveldb::Options::default();
    opt.write_buffer_size = 4 * 1024 * 1024;
    opt.compression_type = rusty_leveldb::CompressionType::CompressionZlibRaw;

    let dir = World::get_world_dir(world_name);

    let db = rusty_leveldb::DB::open(dir, opt).unwrap();

    db
  }

  pub fn close_db (&self) {
    std::mem::drop(&self.db);
  }

  fn get_world_dir (world_name: &str) -> String {
    let dir: String;
  
    let local = app_dirs::get_data_root(app_dirs::AppDataType::UserData).unwrap().into_os_string().into_string().unwrap();
  
    match std::env::consts::OS {
      "macos" | "linux" => {
        dir = format!("{}/mcpelauncher/games/com.mojang/minecraftWorlds/{}/db", local, world_name);
      }
      _ => {
        dir = format!("{}\\Packages\\Microsoft.MinecraftUWP_8wekyb3d8bbwe\\LocalState\\games\\com.mojang\\minecraftWorlds\\{}\\db", local, world_name);
      }
    }
  
    // println!("{}", dir);
    dir
  }

  fn get_entities (db: &mut rusty_leveldb::DB) -> EntityMap {
    let mut entities_map: EntityMap = HashMap::new();
  
    let mut it = db.new_iter().unwrap();
  
    let (mut k, mut v) = (vec![], vec![]);
    while it.advance() {
      it.current(&mut k, &mut v);
  
      if k.len() == 9 && k[8] == 50 {
        let len = v.len();
        let mut entity_cursor = std::io::Cursor::new(&mut v);
  
        let mut cur_offset = 0;
        while cur_offset < len {
          // println!("{:?} {:?}", b"player_", &k[..7]);
          let entity: Entity = de::from_reader(&mut entity_cursor, Endianness::LittleEndian).unwrap();
  
          entities_map.insert(entity.id, entity);
  
          cur_offset = entity_cursor.position() as usize;
        }
      }
    }
  
    entities_map
  }
  
  fn get_scoreboard (db: &mut rusty_leveldb::DB, entities: &EntityMap) -> Scoreboard {
    let mut board = Scoreboard {
      scoreboard_id_to_id: HashMap::new(),
      entity_id_to_scores: HashMap::new(),
    };

    let mut scoreboard = match db.get(b"scoreboard") {
      Some(data) => data,
      _ => return board
    };
    let mut scoreboard_cursor = std::io::Cursor::new(&mut scoreboard);
  
    let raw_scoreboard: RawScoreboard = de::from_reader(&mut scoreboard_cursor, Endianness::LittleEndian).unwrap();
  
    for entry in raw_scoreboard.entries {
      if entry.identity_type == 2 {
        board.scoreboard_id_to_id.insert(entry.scoreboard_id, entry.entity_id.unwrap());
      }
    }
  
    for objective in raw_scoreboard.objectives {
      for score in objective.scores {
        if board.scoreboard_id_to_id.contains_key(&score.scoreboard_id) {
          let entity_id = board.scoreboard_id_to_id.get(&score.scoreboard_id).unwrap();
          if entities.contains_key(&entity_id) {
            let entity = board.entity_id_to_scores.entry(entity_id.clone()).or_insert(HashMap::new());
            entity.insert(objective.name.to_owned(), score.score);
          }
        }
      }
    }
  
    board
  }
}
