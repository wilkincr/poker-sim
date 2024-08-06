use std::fmt;


const PRIMES: [u32; 13] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41];

#[derive(Debug, Copy, Clone)]
pub enum Value {
    /// 2
    Two = 0,
    /// 3
    Three = 1,
    /// 4
    Four = 2,
    /// 5
    Five = 3,
    /// 6
    Six = 4,
    /// 7
    Seven = 5,
    /// 8
    Eight = 6,
    /// 9
    Nine = 7,
    /// T
    Ten = 8,
    /// J
    Jack = 9,
    /// Q
    Queen = 10,
    /// K
    King = 11,
    /// A
    Ace = 12,
}

const VALUES: [Value; 13] = [
    Value::Two,
    Value::Three,
    Value::Four,
    Value::Five,
    Value::Six,
    Value::Seven,
    Value::Eight,
    Value::Nine,
    Value::Ten,
    Value::Jack,
    Value::Queen,
    Value::King,
    Value::Ace,
];

impl Value {
    pub fn values() -> [Self; 13] {
        VALUES
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Two => write!(f, "2"),
            Value::Three => write!(f, "3"),
            Value::Four => write!(f, "4"),
            Value::Five => write!(f, "5"),
            Value::Six => write!(f, "6"),
            Value::Seven => write!(f, "7"),
            Value::Eight => write!(f, "8"),
            Value::Nine => write!(f, "9"),
            Value::Ten => write!(f, "10"),
            Value::Jack => write!(f, "J"),
            Value::Queen => write!(f, "Q"),
            Value::King => write!(f, "K"),
            Value::Ace => write!(f, "A"),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Suit {
    /// Spades
    Club = 0,
    /// Clubs
    Diamond = 1,
    /// Hearts
    Heart = 2,
    /// Diamonds
    Spade = 3,
}

const SUITS: [Suit; 4] = [Suit::Spade, Suit::Club, Suit::Heart, Suit::Diamond];

impl Suit {
    pub fn suits() -> [Self; 4] {
        SUITS
    }
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Suit::Heart => write!(f, "h"),
            Suit::Diamond => write!(f, "d"),
            Suit::Spade => write!(f, "s"),
            Suit::Club => write!(f, "c"),
        }
    }
}

pub struct Card {
    /// The face value of this card.
    pub value: Value,
    /// The suit of this card.
    pub suit: Suit,
    // The integer encoding of this card.
    pub enc: u32
}

impl Card {
    pub fn new(value: Value, suit: Suit) -> Self {
        let suit_enc = 0x8000 >> (suit as u32);
        let p_idx = value as usize;
        let v_idx = value as u32;
        let enc = PRIMES[p_idx] | (v_idx << 8) | suit_enc | (1 << (16 + v_idx));
        Self {
            value: value,
            suit: suit,
            enc: enc
        }
    }
}