pub type Card = u32;

pub const CONCEALED: Card = 0;
pub const TWO: Card = 1 << 1;
pub const THREE: Card = 1 << 2;
pub const FOUR: Card = 1 << 3;
pub const FIVE: Card = 1 << 4;
pub const SIX: Card = 1 << 5;
pub const SEVEN: Card = 1 << 6;
pub const EIGHT: Card = 1 << 7;
pub const NINE: Card = 1 << 8;
pub const TEN: Card = 1 << 9;
pub const JACK: Card = 1 << 10;
pub const QUEEN: Card = 1 << 11;
pub const KING: Card = 1 << 12;
pub const ACE: Card = 1 << 13;
pub const HEART: Card = 1 << 14;
pub const DIAMOND: Card = 1 << 15;
pub const CLUB: Card = 1 << 16;
pub const SPADE: Card = 1 << 17;

pub const VALUES: [Card; 13] = [
    ACE, TWO, THREE, FOUR, FIVE, SIX, SEVEN, EIGHT, NINE, TEN, JACK, QUEEN, KING,
];
pub const SUITS: [Card; 4] = [HEART, DIAMOND, CLUB, SPADE];

pub const SUIT_MASK: Card = HEART | DIAMOND | CLUB | SPADE;
pub const VALUE_MASK: Card =
    ACE | TWO | THREE | FOUR | FIVE | SIX | SEVEN | EIGHT | NINE | TEN | JACK | QUEEN | KING;

pub fn get_suit(card: Card) -> Card {
    card & SUIT_MASK
}

pub fn get_value(card: Card) -> Card {
    card & VALUE_MASK
}

pub fn display_cards(cards: &Vec<Card>) -> String {
    cards
        .iter()
        .map(|&c| display_card(c))
        .collect::<Vec<String>>()
        .join(", ")
}

pub fn display_card(card: Card) -> String {
    let suit = get_suit(card);
    let value = get_value(card);
    let suit_str: &str = match suit {
        HEART => "H",
        DIAMOND => "D",
        CLUB => "C",
        SPADE => "S",
        _ => "Unknown",
    };
    let value_str = match value {
        ACE => "A",
        TWO => "2",
        THREE => "3",
        FOUR => "4",
        FIVE => "5",
        SIX => "6",
        SEVEN => "7",
        EIGHT => "8",
        NINE => "9",
        TEN => "10",
        JACK => "J",
        QUEEN => "Q",
        KING => "K",
        _ => "Unknown",
    };

    format!("{}{}", value_str, suit_str)
}

pub fn count_cards(cards: &Vec<Card>) -> [u8; 14] {
    let mut counts = [0; 14];
    for (idx, val) in counts.iter_mut().enumerate() {
        *val += cards.iter().filter(|&c| c & (1 << idx) != 0).count() as u8;
    }
    counts
}

// test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_card() {
        let card = HEART | ACE;
        assert_eq!(display_card(card), "AH");
    }

    #[test]
    fn test_zeros() {
        assert_eq!(TWO.trailing_zeros(), 1, "Two trailing ones");
        assert_eq!(THREE.trailing_zeros(), 2, "Three trailing ones");
        assert_eq!(ACE.trailing_zeros(), 13, "Ace trailing ones");
    }

    #[test]
    fn test_card_rank() {
        let val = get_value(HEART | QUEEN);
        let val2 = get_value(HEART | THREE);

        assert!(val > val2, "Queen is greater than three");
    }
}
