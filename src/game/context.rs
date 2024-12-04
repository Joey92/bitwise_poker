use crate::{CardStack, CONCEALED};

use super::player::Player;

pub struct GameContextPlayer {
    // get name from actor somehow
    pub chips: usize,
    pub hand: CardStack,
    pub eliminated: bool,
}

impl Into<GameContextPlayer> for Player {
    fn into(self) -> GameContextPlayer {
        GameContextPlayer {
            chips: self.chips,
            hand: CardStack::from(vec![CONCEALED, CONCEALED]),
            eliminated: self.eliminated,
        }
    }
}

pub struct GameContext {
    pub pot: usize,
    pub current_bet: usize,
    pub call_amount: usize,
    pub min_raise: usize,
    pub side_pots: Vec<usize>,
    pub community: CardStack,
    pub hand: CardStack,
    pub chips: usize,
    pub contribution: usize,
    pub players: Vec<GameContextPlayer>,
    pub player_contributions: Vec<usize>,
}
