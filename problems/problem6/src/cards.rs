use std::fmt;

use rand;
use rand::prelude::{SliceRandom, StdRng};
use rand::SeedableRng;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub struct Deck {
    cards: Vec<Card>,
}

#[derive(Copy, Clone, Debug)]
#[derive(PartialEq, Eq)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

#[derive(Copy, Clone, EnumIter, Debug)]
#[derive(PartialEq, Eq)]
pub enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Copy, Clone, EnumIter, Debug)]
#[derive(PartialEq, Eq)]
pub enum Suit {
    Spades,
    Hearts,
    Diamonds,
    Clubs,
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // *** IMPLEMENT THIS METHOD ***
        // It should output one of these unicode strings
        // as appropriate for the suit: "♠" "♥" "♦" "♣"
        // see: https://doc.rust-lang.org/std/fmt/trait.Display.html
        use Suit::*;
        match self {
            Spades => write!(f, "{}", "♠"),
            Hearts => write!(f, "{}", "♥"),
            Diamonds => write!(f, "{}", "♦"),
            Clubs => write!(f, "{}", "♣"),
        }
    }
}

impl Rank {
    pub fn value(&self) -> u8 {
        use Rank::*;
        match self {
            Two => 2,
            Three => 3,
            Four => 4,
            Five => 5,
            Six => 6,
            Seven => 7,
            Eight => 8,
            Nine => 9,
            Ten => 10,
            Jack => 10,
            Queen => 10,
            King => 10,
            Ace => 11,
        }
    }
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // *** IMPLEMENT THIS METHOD ***
        // It should output one of these string values, as appropriate
        // for the given rank:  2 3 4 5 6 7 8 9 10 J Q K A
        // see: https://doc.rust-lang.org/std/fmt/trait.Display.html
        use Rank::*;
        match self {
            Two => write!(f, "{}", "2"),
            Three => write!(f, "{}", "3"),
            Four => write!(f, "{}", "4"),
            Five => write!(f, "{}", "5"),
            Six => write!(f, "{}", "6"),
            Seven => write!(f, "{}", "7"),
            Eight => write!(f, "{}", "8"),
            Nine => write!(f, "{}", "9"),
            Ten => write!(f, "{}", "10"),
            Jack => write!(f, "{}", "J"),
            Queen => write!(f, "{}", "Q"),
            King => write!(f, "{}", "K"),
            Ace => write!(f, "{}", "A"),
        }
    }
}

impl Card {
    pub fn new(suit: Suit, rank: Rank) -> Self {
        Self { suit, rank }
    }

    pub fn default() -> Card {
        Card {
            rank: Rank::Ace,
            suit: Suit::Spades,
        }
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}{}]", self.rank, self.suit)
    }
}

impl Deck {
    pub fn new() -> Self {
        let mut deck = Self {
            cards: Vec::new()
        };
        deck.load();
        deck
    }

    pub fn reset(&mut self) {
        self.cards.clear();
        self.load();
    }

    pub fn deal_card(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    fn load(&mut self) {
        // *** IMPLEMENT THIS METHOD ***
        // Given an empty vector in self.cards, populate it with all 52 of the
        // available unique cards.
        for s in Suit::iter() {
            for r in Rank::iter() {
                self.cards.push(Card { rank: r, suit: s })
            }
        }
    }

    pub fn shuffle(&mut self) {
        self.shuffle_seed(None)
    }

    pub fn shuffle_seed(&mut self, seed: Option<u64>) {
        // *** IMPLEMENT THIS METHOD ***
        // Fully randomize the order of the cards in the self.cards vector.
        let mut rng = match seed {
            None => StdRng::from_entropy(),
            Some(s) => StdRng::seed_from_u64(s)
        };
        self.cards.shuffle(&mut rng);
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_suit_display() {
        assert_eq!(format!("{}", Suit::Spades), "♠");
        assert_eq!(format!("{}", Suit::Hearts), "♥");
        assert_eq!(format!("{}", Suit::Diamonds), "♦");
        assert_eq!(format!("{}", Suit::Clubs), "♣");
    }

    #[test]
    fn test_rank_display() {
        assert_eq!(format!("{}", Rank::Two), "2");
        assert_eq!(format!("{}", Rank::Three), "3");
        assert_eq!(format!("{}", Rank::Four), "4");
        assert_eq!(format!("{}", Rank::Five), "5");
        assert_eq!(format!("{}", Rank::Six), "6");
        assert_eq!(format!("{}", Rank::Seven), "7");
        assert_eq!(format!("{}", Rank::Eight), "8");
        assert_eq!(format!("{}", Rank::Nine), "9");
        assert_eq!(format!("{}", Rank::Ten), "10");
        assert_eq!(format!("{}", Rank::Jack), "J");
        assert_eq!(format!("{}", Rank::Queen), "Q");
        assert_eq!(format!("{}", Rank::King), "K");
        assert_eq!(format!("{}", Rank::Ace), "A");
    }

    #[test]
    fn test_load_deck() {
        use Rank::*;
        use Suit::*;
        let mut deck = Deck {
            cards: Vec::new()
        };
        deck.load();
        assert_eq!(deck.cards.len(), 52);
        assert!(exists(&deck, Ace, Spades));
        assert!(exists(&deck, Two, Spades));
        assert!(exists(&deck, Three, Spades));
        assert!(exists(&deck, Four, Spades));
        assert!(exists(&deck, Five, Spades));
        assert!(exists(&deck, Six, Spades));
        assert!(exists(&deck, Seven, Spades));
        assert!(exists(&deck, Eight, Spades));
        assert!(exists(&deck, Nine, Spades));
        assert!(exists(&deck, Ten, Spades));
        assert!(exists(&deck, Jack, Spades));
        assert!(exists(&deck, Queen, Spades));
        assert!(exists(&deck, King, Spades));
        assert!(exists(&deck, Ace, Hearts));
        assert!(exists(&deck, Two, Hearts));
        assert!(exists(&deck, Three, Hearts));
        assert!(exists(&deck, Four, Hearts));
        assert!(exists(&deck, Five, Hearts));
        assert!(exists(&deck, Six, Hearts));
        assert!(exists(&deck, Seven, Hearts));
        assert!(exists(&deck, Eight, Hearts));
        assert!(exists(&deck, Nine, Hearts));
        assert!(exists(&deck, Ten, Hearts));
        assert!(exists(&deck, Jack, Hearts));
        assert!(exists(&deck, Queen, Hearts));
        assert!(exists(&deck, King, Hearts));
        assert!(exists(&deck, Ace, Diamonds));
        assert!(exists(&deck, Two, Diamonds));
        assert!(exists(&deck, Three, Diamonds));
        assert!(exists(&deck, Four, Diamonds));
        assert!(exists(&deck, Five, Diamonds));
        assert!(exists(&deck, Six, Diamonds));
        assert!(exists(&deck, Seven, Diamonds));
        assert!(exists(&deck, Eight, Diamonds));
        assert!(exists(&deck, Nine, Diamonds));
        assert!(exists(&deck, Ten, Diamonds));
        assert!(exists(&deck, Jack, Diamonds));
        assert!(exists(&deck, Queen, Diamonds));
        assert!(exists(&deck, King, Diamonds));
        assert!(exists(&deck, Ace, Clubs));
        assert!(exists(&deck, Two, Clubs));
        assert!(exists(&deck, Three, Clubs));
        assert!(exists(&deck, Four, Clubs));
        assert!(exists(&deck, Five, Clubs));
        assert!(exists(&deck, Six, Clubs));
        assert!(exists(&deck, Seven, Clubs));
        assert!(exists(&deck, Eight, Clubs));
        assert!(exists(&deck, Nine, Clubs));
        assert!(exists(&deck, Ten, Clubs));
        assert!(exists(&deck, Jack, Clubs));
        assert!(exists(&deck, Queen, Clubs));
        assert!(exists(&deck, King, Clubs));
    }

    #[test]
    fn test_deck_shuffle() {
        use Rank::*;
        use Suit::*;

        let mut deck = Deck {
            cards: vec![
                Card { rank: Ace, suit: Spades },
                Card { rank: Six, suit: Clubs },
                Card { rank: Jack, suit: Diamonds },
                Card { rank: Five, suit: Hearts },
            ]
        };

        deck.shuffle_seed(Some(1));
        assert_eq!(deck.cards,
                   vec![
                       Card { rank: Five, suit: Hearts },
                       Card { rank: Jack, suit: Diamonds },
                       Card { rank: Ace, suit: Spades },
                       Card { rank: Six, suit: Clubs }
                   ]
        );
    }


    fn exists(deck: &Deck, rank: Rank, suit: Suit) -> bool {
        deck.cards.contains(&Card { rank, suit })
    }
}
