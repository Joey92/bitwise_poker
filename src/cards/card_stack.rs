use crate::*;
use std::fmt::Display;

use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Hand {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
    RoyalFlush,
}

#[derive(Debug, Default, Clone)]
pub struct CardStack {
    pub max_cards: usize,
    pub cards: Vec<Card>,
}

impl Display for CardStack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cards = self
            .cards
            .iter()
            .map(|&c| display_card(c))
            .collect::<Vec<String>>();
        f.write_str(&cards.join(", "))
    }
}

impl Ord for CardStack {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_value = self.value();
        let other_value = other.value();

        self_value.cmp(&other_value)
    }
}

impl PartialOrd for CardStack {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for CardStack {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl Eq for CardStack {}

impl Into<Vec<Card>> for CardStack {
    fn into(self) -> Vec<Card> {
        self.cards
    }
}

impl From<Vec<Card>> for CardStack {
    fn from(cards: Vec<Card>) -> Self {
        CardStack {
            max_cards: cards.len(),
            cards,
        }
    }
}

const LOW_STRAIGHT: Card = ACE | TWO | THREE | FOUR | FIVE;
const STRAIGHT_1: Card = TWO | THREE | FOUR | FIVE | SIX;
const STRAIGHT_2: Card = THREE | FOUR | FIVE | SIX | SEVEN;
const STRAIGHT_3: Card = FOUR | FIVE | SIX | SEVEN | EIGHT;
const STRAIGHT_4: Card = FIVE | SIX | SEVEN | EIGHT | NINE;
const STRAIGHT_5: Card = SIX | SEVEN | EIGHT | NINE | TEN;
const STRAIGHT_6: Card = SEVEN | EIGHT | NINE | TEN | JACK;
const STRAIGHT_7: Card = EIGHT | NINE | TEN | JACK | QUEEN;
const STRAIGHT_8: Card = NINE | TEN | JACK | QUEEN | KING;
const HIGH_STRAIGHT: Card = TEN | JACK | QUEEN | KING | ACE;

pub fn get_hand(cards: &Vec<Card>) -> Hand {
    let mut cards = cards.clone();
    sort_cards(&mut cards);

    let cards_mask = cards.iter().fold(0, |acc, &c| acc | c);
    let card_value = cards_mask & VALUE_MASK;

    let is_flush = (cards_mask & SUIT_MASK).count_ones() == 1;
    let is_straight = card_value & LOW_STRAIGHT == LOW_STRAIGHT
        || card_value & STRAIGHT_1 == STRAIGHT_1
        || card_value & STRAIGHT_2 == STRAIGHT_2
        || card_value & STRAIGHT_3 == STRAIGHT_3
        || card_value & STRAIGHT_4 == STRAIGHT_4
        || card_value & STRAIGHT_5 == STRAIGHT_5
        || card_value & STRAIGHT_6 == STRAIGHT_6
        || card_value & STRAIGHT_7 == STRAIGHT_7
        || card_value & STRAIGHT_8 == STRAIGHT_8;

    let is_high_straight = card_value & HIGH_STRAIGHT == HIGH_STRAIGHT;

    if is_flush && is_high_straight {
        return Hand::RoyalFlush;
    }

    if is_flush && is_straight {
        return Hand::StraightFlush;
    }

    if is_straight {
        return Hand::Straight;
    }

    if cards.len() > 5 {
        cards.truncate(5);
    }

    let count = count_cards(&cards);

    let is_four_of_a_kind = count.iter().any(|&c| c == 4);
    if is_four_of_a_kind {
        return Hand::FourOfAKind;
    }

    let is_full_house = count.iter().any(|&c| c == 3) && count.iter().any(|&c| c == 2);
    if is_full_house {
        return Hand::FullHouse;
    }

    let is_three_of_a_kind = count.iter().any(|&c| c == 3);
    if is_three_of_a_kind {
        return Hand::ThreeOfAKind;
    }

    let is_two_pair = count.iter().filter(|&c| *c == 2).count() == 2;
    if is_two_pair {
        return Hand::TwoPair;
    }

    let is_pair = count.iter().any(|&c| c == 2);
    if is_pair {
        return Hand::Pair;
    }

    Hand::HighCard
}

pub fn value(cards: &Vec<Card>) -> Card {
    let mut value = 0;

    for card in cards {
        value += get_value(*card);
    }

    for &count in count_cards(cards).iter() {
        value += (count as u32).pow(count as u32);
    }

    value
}

pub fn shuffle(cards: &mut Vec<Card>) {
    cards.shuffle(&mut thread_rng());
}

impl CardStack {
    pub fn new(max_cards: usize) -> CardStack {
        CardStack {
            max_cards,
            cards: Vec::with_capacity(max_cards),
        }
    }

    pub fn get_hand(&self) -> Hand {
        get_hand(&self.cards)
    }

    pub fn value(&self) -> Card {
        value(&self.cards)
    }

    pub fn push(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn pop(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    pub fn shuffle(&mut self) {
        shuffle(&mut self.cards)
    }

    pub fn sort(&mut self) {
        sort_cards(&mut self.cards)
    }

    pub fn standard_deck() -> Self {
        let mut deck = CardStack::new(52);
        for &suit in &[HEART, DIAMOND, CLUB, SPADE] {
            for &value in &[
                ACE, TWO, THREE, FOUR, FIVE, SIX, SEVEN, EIGHT, NINE, TEN, JACK, QUEEN, KING,
            ] {
                deck.push(suit | value);
            }
        }
        deck.shuffle();
        deck
    }
}

pub fn sort_cards(cards: &mut Vec<Card>) {
    // group by card value and larger groups should be at the front
    let count = count_cards(&cards);
    cards.sort_by(|a, b| {
        let a = get_value(*a);
        let b = get_value(*b);

        let a_count = count[a.trailing_zeros() as usize];
        let b_count = count[b.trailing_zeros() as usize];

        // compare by value if no group
        if a_count != b_count {
            return b_count.cmp(&a_count);
        }

        return b.cmp(&a);
    });
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use crate::*;

    #[test]
    fn test_card_stack_ordering() {
        let mut stack1 = CardStack::new(7);
        stack1.push(HEART | ACE);
        stack1.push(HEART | TWO);
        stack1.push(HEART | THREE);

        let mut stack2 = CardStack::new(7);
        stack2.push(HEART | TWO);
        stack2.push(HEART | THREE);
        stack2.push(HEART | FOUR);

        assert_eq!(stack1.cmp(&stack2), Ordering::Greater);
    }

    #[test]
    fn test_card_stack_order_equal() {
        let mut stack1 = CardStack::new(7);
        stack1.push(HEART | TWO);
        stack1.push(DIAMOND | TWO);
        stack1.push(HEART | THREE);

        let mut stack2 = CardStack::new(7);
        stack2.push(CLUB | TWO);
        stack2.push(SPADE | TWO);
        stack2.push(SPADE | THREE);

        assert_eq!(stack1.cmp(&stack2), Ordering::Equal);
    }

    #[test]
    fn test_card_stack_ordering_full_house() {
        let mut stack1 = CardStack::new(7);
        stack1.push(HEART | TWO);
        stack1.push(DIAMOND | TWO);
        stack1.push(HEART | THREE);
        stack1.push(DIAMOND | THREE);
        stack1.push(SPADE | THREE);

        let mut stack2 = CardStack::new(7);
        stack2.push(HEART | THREE);
        stack2.push(DIAMOND | THREE);
        stack2.push(SPADE | THREE);
        stack2.push(HEART | FOUR);
        stack2.push(DIAMOND | FOUR);

        assert_eq!(stack1.cmp(&stack2), Ordering::Less);
    }

    #[test]
    fn test_card_stack_ordering_kind() {
        let mut stack1 = CardStack::new(7);
        stack1.push(HEART | TWO);
        stack1.push(DIAMOND | TWO);
        stack1.push(CLUB | TWO);
        stack1.push(SPADE | TWO);
        stack1.push(SPADE | THREE);
        stack1.push(HEART | ACE);
        stack1.push(CLUB | TWO);

        let mut stack2 = CardStack::new(7);
        stack2.push(HEART | THREE);
        stack2.push(CLUB | THREE);
        stack2.push(CLUB | TWO);
        stack2.push(SPADE | TWO);
        stack2.push(SPADE | THREE);
        stack2.push(HEART | ACE);
        stack2.push(CLUB | TWO);

        assert_eq!(
            stack1.cmp(&stack2),
            Ordering::Greater,
            "{} should be greater than {}",
            stack1.value(),
            stack2.value()
        );
    }

    #[test]
    fn test_card_deck_creation() {
        let deck = CardStack::standard_deck();
        assert_eq!(deck.cards.len(), 52);
    }

    #[test]
    fn test_card_count() {
        let mut stack = CardStack::new(7);
        stack.push(HEART | KING);
        stack.push(HEART | TWO);
        stack.push(HEART | THREE);
        stack.push(SPADE | TWO);
        stack.push(HEART | FIVE);
        stack.push(DIAMOND | TWO);

        let counts = count_cards(&stack.cards);

        assert_eq!(counts[0], 0, "counting none");
        assert_eq!(counts[1], 3, "counting Twos");
        assert_eq!(counts[2], 1, "counting Threes");
        assert_eq!(counts[3], 0, "counting Fours");
        assert_eq!(counts[4], 1, "counting Fives");
        assert_eq!(counts[5], 0, "counting Sixes");
        assert_eq!(counts[6], 0, "counting Sevens");
        assert_eq!(counts[7], 0, "counting Eights");
        assert_eq!(counts[8], 0, "counting Nines");
        assert_eq!(counts[9], 0, "counting Tens");
        assert_eq!(counts[10], 0, "counting Jacks");
        assert_eq!(counts[11], 0, "counting Queens");
        assert_eq!(counts[12], 1, "counting Kings");
        assert_eq!(counts[13], 0, "counting Aces");
    }

    #[test]
    fn test_card_stack_sort() {
        let mut stack = CardStack::new(7);
        stack.push(HEART | ACE);
        stack.push(HEART | TWO);
        stack.push(HEART | THREE);
        stack.push(DIAMOND | ACE);
        stack.push(SPADE | TWO);
        stack.push(HEART | QUEEN);
        stack.push(DIAMOND | TWO);

        stack.sort();

        let mut expected = CardStack::new(7);
        expected.push(HEART | TWO);
        expected.push(SPADE | TWO);
        expected.push(DIAMOND | TWO);
        expected.push(HEART | ACE);
        expected.push(DIAMOND | ACE);
        expected.push(HEART | QUEEN);
        expected.push(HEART | THREE);

        assert_eq!(
            format!("{}", stack),
            format!("{}", expected),
            "stack is not sorted right"
        );

        let mut stack = CardStack::new(7);
        stack.push(HEART | TEN);
        stack.push(HEART | JACK);
        stack.push(HEART | QUEEN);
        stack.push(HEART | KING);
        stack.push(HEART | ACE);
        stack.push(HEART | TWO);
        stack.push(HEART | THREE);

        stack.sort();

        let mut expected = CardStack::new(7);
        expected.push(HEART | ACE);
        expected.push(HEART | KING);
        expected.push(HEART | QUEEN);
        expected.push(HEART | JACK);
        expected.push(HEART | TEN);
        expected.push(HEART | THREE);
        expected.push(HEART | TWO);

        assert_eq!(format!("{}", stack), format!("{}", expected));

        let mut stack = CardStack::new(7);
        stack.push(HEART | TWO);
        stack.push(DIAMOND | TWO);
        stack.push(HEART | THREE);
        stack.push(DIAMOND | THREE);
        stack.push(SPADE | THREE);
        stack.push(HEART | ACE);
        stack.push(CLUB | TWO);

        stack.sort();

        let mut expected = CardStack::new(7);
        expected.push(HEART | THREE);
        expected.push(DIAMOND | THREE);
        expected.push(SPADE | THREE);
        expected.push(HEART | TWO);
        expected.push(DIAMOND | TWO);
        expected.push(CLUB | TWO);
        expected.push(HEART | ACE);

        assert_eq!(format!("{}", stack), format!("{}", expected));
    }

    #[test]
    fn test_is_flush() {
        let c1 = HEART | ACE;
        let c2 = HEART | TWO;
        let c3 = HEART | THREE;
        let c4 = HEART | FOUR;
        let c5 = HEART | FIVE;

        let cards = c1 | c2 | c3 | c4 | c5;

        assert_eq!(cards & SUIT_MASK, HEART);
        assert_ne!(cards & SUIT_MASK, SPADE);
        assert_ne!(cards & SUIT_MASK, CLUB);
        assert_ne!(cards & SUIT_MASK, DIAMOND);
    }

    #[test]
    fn test_is_not_flush() {
        let c1 = HEART | ACE;
        let c2 = HEART | TWO;
        let c3 = DIAMOND | THREE;
        let c4 = HEART | FOUR;
        let c5 = HEART | FIVE;

        let cards = c1 | c2 | c3 | c4 | c5;

        assert_ne!(cards & SUIT_MASK, SUIT_MASK);
        assert_ne!(cards & SUIT_MASK, HEART | SPADE);
        assert_eq!(cards & SUIT_MASK, HEART | DIAMOND);
    }

    #[test]
    fn test_card_stack_shuffle() {
        let mut stack = CardStack::new(7);
        stack.push(HEART | ACE);
        stack.push(HEART | TWO);
        stack.push(HEART | THREE);
        stack.push(SPADE | TWO);
        stack.push(HEART | FIVE);
        stack.push(DIAMOND | TWO);

        let original_stack = stack.cards.clone();
        stack.shuffle();

        assert_ne!(stack.cards, original_stack);
    }

    #[test]
    fn test_full_house() {
        let mut stack = CardStack::new(7);
        stack.push(HEART | TWO);
        stack.push(DIAMOND | TWO);
        stack.push(HEART | THREE);
        stack.push(DIAMOND | THREE);
        stack.push(SPADE | THREE);
        stack.push(HEART | ACE);
        stack.push(CLUB | TWO);

        assert_eq!(Hand::FullHouse, stack.get_hand())
    }

    #[test]
    fn test_four_of_a_kind() {
        let mut stack = CardStack::new(7);
        stack.push(HEART | TWO);
        stack.push(DIAMOND | TWO);
        stack.push(CLUB | TWO);
        stack.push(SPADE | TWO);
        stack.push(SPADE | THREE);
        stack.push(HEART | ACE);
        stack.push(CLUB | TEN);

        assert_eq!(Hand::FourOfAKind, stack.get_hand())
    }

    #[test]
    fn test_straight_flush() {
        let mut stack = CardStack::new(7);
        stack.push(HEART | TWO);
        stack.push(HEART | THREE);
        stack.push(HEART | FOUR);
        stack.push(HEART | FIVE);
        stack.push(HEART | SIX);
        stack.push(HEART | SEVEN);
        stack.push(HEART | EIGHT);

        assert_eq!(Hand::StraightFlush, stack.get_hand())
    }

    #[test]
    fn test_royal_flush() {
        let mut stack = CardStack::new(7);
        stack.push(HEART | TEN);
        stack.push(HEART | JACK);
        stack.push(HEART | QUEEN);
        stack.push(HEART | KING);
        stack.push(HEART | ACE);
        stack.push(HEART | TWO);
        stack.push(HEART | THREE);

        assert_eq!(Hand::RoyalFlush, stack.get_hand())
    }
}
