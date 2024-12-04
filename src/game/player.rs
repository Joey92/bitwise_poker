use crate::CardStack;

#[derive(Debug, Clone)]
pub struct Player {
    pub name: String,
    pub eliminated: bool,
    pub hand: CardStack,
    pub chips: usize,
    pub folded: bool,
    pub bet: usize,
}

impl Player {
    pub fn new(name: String, chips: usize) -> Self {
        Self {
            name,
            hand: CardStack::new(0),
            eliminated: false,
            chips,
            folded: false,
            bet: 0,
        }
    }
}
