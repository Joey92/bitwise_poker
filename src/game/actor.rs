use crate::util::read_user_input;

use super::context::GameContext;

pub enum PlayerAction {
    Call,
    Raise(usize),
    Fold,
}

pub trait Actionable {
    fn action(&self, game: &GameContext) -> PlayerAction {
        PlayerAction::Fold
    }

    fn name(&self) -> &str {
        "Player"
    }
}

pub struct LocalPlayer;
impl Actionable for LocalPlayer {
    fn action(&self, game: &GameContext) -> PlayerAction {
        println!("Your turn");
        println!("{} in pot, your cards: {}", game.pot, game.hand);
        println!("Community cards: {}", game.community);

        if game.contribution == game.current_bet {
            println!("1. Check");
        } else {
            println!("1. Call ({})", game.call_amount);
        }
        println!("2. Raise ({} minimum)", game.min_raise);
        println!("3. Fold");

        match read_user_input() {
            1 => PlayerAction::Call,
            2 => {
                println!("Enter raise amount");
                PlayerAction::Raise(read_user_input())
            }
            3 => PlayerAction::Fold,
            _ => {
                println!("Invalid choice");
                self.action(game)
            }
        }
    }

    fn name(&self) -> &str {
        "You"
    }
}

pub struct NPC {
    name: String,
}

impl NPC {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

impl Actionable for NPC {
    fn action(&self, game: &GameContext) -> PlayerAction {
        if game.contribution == 0 || game.contribution == game.current_bet {
            return PlayerAction::Call;
        }

        PlayerAction::Fold
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }
}
