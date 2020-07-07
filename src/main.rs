mod world;
mod scoreboard;
mod selector;

use crate::world::{World};
use crate::selector::{Selector};

fn main() {
  let args: Vec<String> = std::env::args().collect();

  let world = World::new(args.get(1).unwrap());

  let World { scoreboard, entities, .. } = &world;

  let selector = Selector::new(args.get(2).unwrap());

  println!("parsed selector: {:?}", selector); 

  let mut count = 0;
  for (_, entity) in entities.iter() {
    if selector.entity_matches(entity, scoreboard) {
      count += 1;
      if let Some(scores) = scoreboard.entity_id_to_scores.get(&entity.id) {
        println!("-----\nentity: {:?}\nscores: {:?}\n-----", entity, scores);
      } else {
        println!("-----\nentity: {:?}\nscores: none\n-----", entity);
      }
    }
  }

  println!("found {} entities", count);

  world.close_db();
}
