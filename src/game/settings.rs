use crate::CardStack;

use super::player::Player;

#[derive(Debug, Clone)]
pub enum GameType {
    TexasHoldem, // 2 cards
    Omaha,       // 4 cards in hand afaik

    // haha can we do all these games too?
    FiveCardDraw,
    SevenCardStud,
    FiveCardStud,
    Razz,
    Pineapple,
    CrazyPineapple,
    OmahaHiLo,
    Courchevel,
    DoubleBoardOmaha,
    Irish,
    Badugi,
    TripleDraw,
    SingleDraw,
    TripleStud,
    DoubleStud,
    EightGame,
    Horse,
    DealerChoice,
}
