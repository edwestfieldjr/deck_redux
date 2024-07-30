/*! Data Structure for a deck of 52 playing cards */

use rand::seq::SliceRandom;
use rand::thread_rng;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct SuitVal {
    pub id: char,
    pub rank: u8,
}

#[derive(Debug, Clone, Copy)]
pub enum Suit {
    Hearts(SuitVal),
    Diamonds(SuitVal),
    Clubs(SuitVal),
    Spades(SuitVal),
}

impl Suit {
    pub fn get_suit(&self) -> &SuitVal {
        match self {
            Suit::Hearts(sval) => sval,
            Suit::Diamonds(sval) => sval,
            Suit::Clubs(sval) => sval,
            Suit::Spades(sval) => sval,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct Value<'a> {
    pub id: &'a str,
    pub rank: u8,
    pub points: u32,
}

#[derive(Debug, Clone, Copy)]
pub enum Rank<'a> {
    Two(Value<'a>),
    Three(Value<'a>),
    Four(Value<'a>),
    Five(Value<'a>),
    Six(Value<'a>),
    Seven(Value<'a>),
    Eight(Value<'a>),
    Nine(Value<'a>),
    Ten(Value<'a>),
    Jack(Value<'a>),
    Queen(Value<'a>),
    King(Value<'a>),
    Ace(Value<'a>),
}

impl<'a> Rank<'a> {
    pub fn get_value(&self) -> &'a Value {
        match self {
            Rank::Two(value) => value,
            Rank::Three(value) => value,
            Rank::Four(value) => value,
            Rank::Five(value) => value,
            Rank::Six(value) => value,
            Rank::Seven(value) => value,
            Rank::Eight(value) => value,
            Rank::Nine(value) => value,
            Rank::Ten(value) => value,
            Rank::Jack(value) => value,
            Rank::Queen(value) => value,
            Rank::King(value) => value,
            Rank::Ace(value) => value,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Card<'a> {
    pub suit: Suit,
    pub rank: Rank<'a>,
}

#[derive(Debug, Clone)]
pub struct Deck<'a> {
    pub cards: Vec<Card<'a>>,
}

impl<'a> Deck<'a> {
    pub fn new() -> Deck<'a> {
        let suits = [
            Suit::Spades(SuitVal { id: '♠', rank: 3 }),
            Suit::Hearts(SuitVal { id: '♡', rank: 2 }),
            Suit::Diamonds(SuitVal { id: '♢', rank: 1 }),
            Suit::Clubs(SuitVal { id: '♣', rank: 0 }),
        ];
        let ranks = [
            Rank::Two(Value {
                id: "2",
                rank: 0,
                points: 2,
            }),
            Rank::Three(Value {
                id: "3",
                rank: 1,
                points: 3,
            }),
            Rank::Four(Value {
                id: "4",
                rank: 2,
                points: 4,
            }),
            Rank::Five(Value {
                id: "5",
                rank: 3,
                points: 5,
            }),
            Rank::Six(Value {
                id: "6",
                rank: 4,
                points: 6,
            }),
            Rank::Seven(Value {
                id: "7",
                rank: 5,
                points: 7,
            }),
            Rank::Eight(Value {
                id: "8",
                rank: 6,
                points: 8,
            }),
            Rank::Nine(Value {
                id: "9",
                rank: 7,
                points: 9,
            }),
            Rank::Ten(Value {
                id: "10",
                rank: 8,
                points: 10,
            }),
            Rank::Jack(Value {
                id: "J",
                rank: 9,
                points: 10,
            }),
            Rank::Queen(Value {
                id: "Q",
                rank: 10,
                points: 12,
            }),
            Rank::King(Value {
                id: "K",
                rank: 11,
                points: 10,
            }),
            Rank::Ace(Value {
                id: "A",
                rank: 12,
                points: 11,
            }),
        ];

        let mut cards = Vec::new();
        for &suit in &suits {
            for &rank in &ranks {
                cards.push(Card { suit, rank });
            }
        }

        Deck { cards }
    }

    pub fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }

    pub fn draw(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    pub fn len(&self) -> usize {
        self.cards.len()
    }

    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }
}
