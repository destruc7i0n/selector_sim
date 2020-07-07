use std::collections::HashMap;

use serde::{Serialize, Deserialize};

type Score = HashMap<String, i32>;
pub type SelectorScores = HashMap<String, (i32, i32)>;

#[derive(Debug)]
pub struct Scoreboard {
  pub scoreboard_id_to_id: HashMap<i64, i64>,
  pub entity_id_to_scores: HashMap<i64, Score>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RawScoreboard {
  #[serde(rename = "LastUniqueID")] pub last_unique_id: i64,
  #[serde(rename = "Entries")] pub entries: Vec<RawScoreboardEntry>,
  #[serde(rename = "Objectives")] pub objectives: Vec<RawScoreboardObjective>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RawScoreboardEntry {
  #[serde(rename = "ScoreboardId")] pub scoreboard_id: i64,
  #[serde(rename = "EntityID")] pub entity_id: Option<i64>,
  #[serde(rename = "PlayerID")] pub player_id: Option<i64>,
  #[serde(rename = "IdentityType")] pub identity_type: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RawScoreboardObjective {
  #[serde(rename = "Scores")] pub scores: Vec<RawScoreboardScore>,
  #[serde(rename = "Name")] pub name: String,
  #[serde(rename = "Criteria")] pub criteria: String,
  #[serde(rename = "DisplayName")] pub display_name: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RawScoreboardScore {
  #[serde(rename = "Score")] pub score: i32,
  #[serde(rename = "ScoreboardId")] pub scoreboard_id: i64
}

pub fn extract_scores (args: &str) -> (String, SelectorScores) {
  let mut map: SelectorScores = HashMap::new();

  let scores_re = regex::Regex::new(r",?scores=\{(.*)\}").unwrap();

  let rest = scores_re.replace_all(&args, "").into_owned();

  let scores: String = match scores_re.captures(args) {
    Some(cap) => cap.get(1).unwrap().as_str(),
    None => return (rest, map) 
  }.to_string();

  let selector_scores = scores.split(",");
  for score in selector_scores {
    let score_parts = score.split("=").collect::<Vec<_>>();

    let (min, max);

    let score_value = score_parts[1];

    if score_value.contains(&"..") {
      let min_max = score_value.split("..").collect::<Vec<_>>();
      if min_max.len() == 2 {
        min = min_max[0];
        max = min_max[1];
      } else {
        panic!("Invalid selector!");
      }
    } else {
      min = score_parts[1];
      max = min;
    }

    map.insert(
      score_parts[0].to_string(),
      (
        min.parse().unwrap_or(-2147483648), // default min and max values
        max.parse().unwrap_or(2147483647)
      )
    );
  }

  (rest, map)
}
