use rand::{seq::SliceRandom, thread_rng};

#[derive(Debug, Clone, Copy)]
pub enum Card {
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

impl Card {
    fn value(&self) -> usize {
        use Card::*;

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

    fn iterator() -> impl Iterator<Item = Card> {
        use Card::*;

        [
            Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King, Ace,
        ]
        .iter()
        .copied()
    }
}

#[derive(Clone, Copy)]
pub enum Suit {
    Clubs,
    Hearts,
    Deamonds,
    Spades,
}

impl Suit {
    fn iterator() -> impl Iterator<Item = Suit> {
        use Suit::*;
        [Clubs, Hearts, Deamonds, Spades].iter().copied()
    }
}

pub struct Dealer {
    desk: Vec<Card>,
}

pub struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    pub fn new() -> Hand {
        Hand { cards: vec![] }
    }

    pub fn get_card(&mut self) -> Vec<Card> {
        self.cards.clone()
    }

    pub fn add(&mut self, card: Card) {
        self.cards.push(card);
    }

    fn value(&self) -> usize {
        use Card::*;

        let mut subtotal = 0;
        let mut aces = 0;

        for card in self.cards.iter() {
            subtotal += match card {
                Ace => {
                    aces += 1;
                    0
                }
                _ => card.value(),
            };
        }

        for _ in 0..aces {
            let ace_value = if subtotal <= 10 { 11 } else { 1 };
            subtotal += ace_value;
        }

        subtotal
    }

    pub fn is_loosing(&self) -> bool {
        self.value() > 21
    }
}

impl Dealer {
    pub fn new() -> Dealer {
        // ...
        let mut desk = vec![];

        for _suit in Suit::iterator() {
            for card in Card::iterator() {
                desk.push(card)
            }
        }

        Dealer { desk: desk }
    }

    pub fn get_desk(&mut self) -> Vec<Card> {
        let mut rng = thread_rng();
        self.desk.shuffle(&mut rng);

        self.desk.clone()
    }
}

#[test]
fn empty_hand() {
    let hand = Hand::new();

    assert_eq!(hand.value(), 0);
}

#[test]
fn strong_hand() {
    let mut hand = Hand::new();
    hand.add(Card::Queen);
    hand.add(Card::Ace);

    assert_eq!(hand.value(), 21);
}

#[test]
fn risky_hand() {
    let mut hand = Hand::new();
    hand.add(Card::King);
    hand.add(Card::Queen);
    hand.add(Card::Ace);

    assert_eq!(hand.value(), 21);
}

#[test]
fn oops() {
    let mut hand = Hand::new();
    hand.add(Card::King);
    hand.add(Card::Seven);
    hand.add(Card::Five);

    assert!(hand.is_loosing());
    assert_eq!(hand.value(), 22);
}
