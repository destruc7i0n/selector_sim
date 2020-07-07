use std::collections::HashMap;

use crate::world::Entity;
use crate::scoreboard::{Scoreboard, SelectorScores, extract_scores};

#[derive(Debug)]
pub struct Selector {
  name: String,
  entity_type: String,
  tags: Vec<String>,
  scores: SelectorScores,
}

impl Selector {
  pub fn new (selector: &str) -> Selector {
    Selector {
      ..Selector::parse_selector(selector)
    }
  }

  fn parse_selector (selector: &str) -> Selector {
    let mut sel = Selector {
      name: "".to_string(),
      entity_type: "".to_string(),
      tags: Vec::new(),
      scores: HashMap::new()
    };

    if selector.len() > 2 && &selector[0..2] == "@e" {
      // let entity_selector = &selector[..2];

      let args = &selector[3..selector.len() - 1];
  
      let (value_based_selectors, scores) = extract_scores(&args);
      sel.scores = scores;
  
      let others = value_based_selectors.split(",");
      for selector in others {
        let parts = selector.split("=").collect::<Vec<_>>();
        match parts[0] {
          "type" => sel.entity_type = parts[1].to_string(),
          "name" => sel.name = parts[1].to_string(),
          "tag" => sel.tags.push(parts[1].to_string()),
          _ => {
            println!("Unknown key \"{}\" found - ignoring.", parts[0]);
          }
        };
      }
    } else {
      panic!("Invalid selector.");
    }
  
    sel
  }
  
  pub fn entity_matches (&self, entity: &Entity, scoreboard: &Scoreboard) -> bool {
    if self.entity_type != "" {
      let normal_type = &entity.entity_type.replace("minecraft:", "");
      let normal_own_type = &self.entity_type.replace("minecraft:", "");

      if normal_own_type.starts_with("!") {
        let ent = normal_own_type[1..].to_string();
        if normal_type == &ent { return false; }
      } else {
        if normal_type != normal_own_type { return false; }
      }
    }

    if self.name != "" {
      let correct_name = match &entity.name {
        Some(name) => name == &self.name,
        _ => false,
      };
      if !correct_name { return false; }
    }
  
    if self.tags.len() > 0 {
      let mut is_valid = true;
  
      for tag in self.tags.iter() {
        if tag.starts_with("!") {
          let tag_name = &tag[1..].to_string();
          is_valid = is_valid && !entity.tags.contains(tag_name);
        } else {
          is_valid = is_valid && entity.tags.contains(tag);
        }
      }
  
      if !is_valid { return false; }
    }
  
    if self.scores.len() > 0 {
      for (score_id, score_value) in self.scores.iter() {
        let mut is_valid = false;

        if let Some(entity_scores) = scoreboard.entity_id_to_scores.get(&entity.id) {
          if let Some(score) = entity_scores.get(score_id) {
            // println!("{:?} {:?} {:?} {}", entity, entity_scores, score_value, score);
            is_valid = (score_value.0..=score_value.1).contains(score);
          }
        }

        if !is_valid { return false; }
      }
    }
  
    true
  }
}
