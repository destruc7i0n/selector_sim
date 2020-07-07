use structopt::StructOpt;

mod world;
mod scoreboard;
mod selector;

use crate::world::{World};
use crate::selector::{Selector};

#[derive(Debug, StructOpt)]
struct Cli {
  #[structopt(help = "The name of the world folder")]
  world: String,
  #[structopt(help = "The entity selector")]
  selector: String,
}

fn main() {
  let args = Cli::from_args();

  let world = World::new(&args.world);

  let World { scoreboard, entities, .. } = &world;

  let selector = Selector::new(&args.selector);

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
