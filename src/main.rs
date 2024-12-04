use game::actor::LocalPlayer;
use game::actor::NPC;
use game::table::TexasHoldem;

use crate::cards::card::*;
use crate::cards::card_stack::*;

mod cards;
mod game;
mod util;

fn main() {
    let mut table = TexasHoldem::new(10, 2000, 20000);

    let bots = vec!["Alice", "Bob", "Charlie", "David", "Eve"];
    bots.iter()
        .for_each(|&name| table.add_player(name.to_string(), Box::new(NPC::new(name.to_string()))));

    table.add_player("You".to_string(), Box::new(LocalPlayer));

    table.play();
}
