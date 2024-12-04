use crate::{game::actor::Actionable, CardStack};

use super::{actor::PlayerAction, context::GameContext, player::Player};

pub enum GameState {
    PreFlop,
    Flop,
    Turn,
    River,
    Showdown,
}

pub struct TexasHoldem {
    pub players: Vec<Player>,
    pub actors: Vec<Box<dyn Actionable>>,
    pub blinds: usize,
    pub buy_in: usize,
    pub state: GameState,
    pub dealer: usize,
}

impl TexasHoldem {
    pub fn new(players: usize, blinds: usize, buy_in: usize) -> Self {
        return Self {
            players: Vec::with_capacity(players),
            blinds,
            buy_in,
            actors: Vec::with_capacity(players),
            state: GameState::PreFlop,
            dealer: 0,
        };
    }

    pub fn add_player(&mut self, name: String, p: Box<dyn Actionable>) {
        self.players.push(Player::new(name, self.buy_in));
        self.actors.push(p);
    }

    pub fn play(&mut self) {
        loop {
            self.play_round();
            self.dealer += 1;

            if self.players.iter().filter(|p| !p.eliminated).count() == 1 {
                // player won!
                break;
            }
        }

        println!("Game over");
    }

    fn play_round(&mut self) {
        let mut community_cards: CardStack = CardStack::new(5);
        let mut deck = CardStack::standard_deck();

        let mut rounds = 0;

        loop {
            let active_player_count: usize = self.players.iter().filter(|p| !p.folded).count();

            self.state = match self.state {
                GameState::PreFlop => {
                    for _ in 0..2 {
                        for player in self.players.iter_mut() {
                            let card = deck
                                .pop()
                                .expect("Could not deal cards because the deck is empty");

                            player.hand.push(card)
                        }
                    }
                    GameState::Flop
                }
                GameState::Flop => {
                    for _ in 0..3 {
                        let card = deck
                            .pop()
                            .expect("Could not deal cards because the deck is empty");
                        community_cards.push(card);
                    }

                    GameState::Turn
                }
                GameState::Turn => {
                    let card = deck
                        .pop()
                        .expect("Could not deal cards because the deck is empty");
                    community_cards.push(card);
                    GameState::River
                }
                GameState::River => {
                    let card = deck
                        .pop()
                        .expect("Could not deal cards because the deck is empty");
                    community_cards.push(card);
                    GameState::Showdown
                }
                GameState::Showdown => {
                    // get all active players cards
                    // compare against each other
                    // get winner
                    // move pot to winner
                    todo!()
                }
            };

            if rounds == 1 {
                current_player.bet = self.blinds;
                println!(
                    "{}: Paying small blind: {}",
                    current_player.name,
                    self.blinds / 2
                );
                continue;
            }

            if rounds == 2 {
                current_player.bet = self.blinds / 2;
                println!(
                    "{}: Paying large blind: {}",
                    current_player.name, self.blinds
                );
                continue;
            }

            // betting round
            loop {
                rounds += 1;

                let current_player_idx = (self.dealer + rounds) % self.players.len();
                let bets: Vec<usize> = self.players.iter().map(|p| p.bet).collect();

                let pot: usize = bets.iter().sum();
                let current_bet = *bets.iter().max().expect("No bets?");

                let current_player = self
                    .players
                    .get(current_player_idx)
                    .expect("Some player is missing");

                let context = GameContext {
                    pot,
                    current_bet: current_bet,
                    call_amount: current_bet - current_player.bet,
                    min_raise: current_bet * 2,
                    contribution: current_player.bet,
                    side_pots: vec![],
                    community: community_cards.clone(),
                    hand: current_player.hand.clone(),
                    chips: current_player.chips,
                    players: self.players.clone().into_iter().map(|p| p.into()).collect(),
                    player_contributions: bets.clone(),
                };

                let current_player = self
                    .players
                    .get_mut(current_player_idx)
                    .expect("Some player is missing");

                // break the loop when all active players have contributed the same amount
                if self
                    .players
                    .iter()
                    .filter(|p| !p.folded && !p.eliminated)
                    .all(|p| p.bet == current_bet)
                {
                    println!("All players have contributed the same amount, time to move on");
                    break;
                }

                if current_player.folded || current_player.eliminated || current_player.chips == 0 {
                    println!("{}: I'm out", current_player.name);
                    continue;
                }

                let action = self.actors[current_player_idx].action(&context);

                match action {
                    PlayerAction::Call => {
                        let call_amount = current_bet - current_player.bet;

                        if call_amount == 0 {
                            println!("{}: I Check", current_player.name);
                        } else {
                            println!("{}: I Call {}", current_player.name, call_amount);
                        }

                        if current_player.chips - call_amount <= 0 {
                            println!("{}: I'm all in", current_player.name);
                            current_player.bet += current_player.chips;
                            current_player.chips = 0;
                            // handle side pots
                            continue;
                        }

                        current_player.bet += call_amount;
                        current_player.chips -= call_amount;
                    }
                    PlayerAction::Raise(amount) => {
                        println!("{}: I Raise {}", current_player.name, amount);

                        if current_player.chips - amount <= 0 {
                            println!("{}: I'm all in", current_player.name);
                            current_player.bet += current_player.chips;
                            current_player.chips = 0;
                            // handle side pots
                            continue;
                        }

                        current_player.bet += amount;
                        current_player.chips -= amount;
                    }
                    PlayerAction::Fold => {
                        println!("{}: I Fold", current_player.name);
                        current_player.folded = true;
                        continue;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::*;

    #[test]
    fn test_adding_users() {
        let mut game = TexasHoldem::new(10, 2000, 20000);

        game.add_player("Alice".to_string(), Box::new(NPC::new("Alice".to_string())));
        game.add_player("Bob".to_string(), Box::new(NPC::new("Bob".to_string())));
        game.add_player(
            "Charlie".to_string(),
            Box::new(NPC::new("Charlie".to_string())),
        );

        assert!(game.players.len() == 3);
    }

    #[test]
    fn test_basic_rules() {
        let mut game = TexasHoldem::new(10, 2000, 20000);

        game.add_player("Alice".to_string(), Box::new(NPC::new("Alice".to_string())));
        game.add_player("Bob".to_string(), Box::new(NPC::new("Bob".to_string())));
        game.add_player(
            "Charlie".to_string(),
            Box::new(NPC::new("Charlie".to_string())),
        );

        game.play();
    }
}
