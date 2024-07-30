/*! Draws two cards: You vs computer */

mod deck;
use deck::Deck;

fn main() {
    let mut deck = Deck::new();
    deck.shuffle();

    // while deck.len() > 1 {
    let cc1 = deck.draw().unwrap();
    let cc1_val = cc1.rank.get_value().rank.clone();

    let cc1_suit = cc1.suit.get_suit().rank.clone();

    println!(
        " Your card: {}{}",
        cc1.rank.get_value().id,
        cc1.suit.get_suit().id
    );

    let cc2 = deck.draw().unwrap();
    let cc2_val = cc2.rank.get_value().rank.clone();
    let cc2_suit = cc2.suit.get_suit().rank.clone();

    println!(
        "CPU's card: {}{}",
        cc2.rank.get_value().id,
        cc2.suit.get_suit().id
    );

    if (cc1_val > cc2_val) || ((cc1_val == cc2_val) && (cc1_suit > cc2_suit)) {
        println!("You win!");
    } else if (cc2_val > cc1_val) || ((cc2_val == cc1_val) && (cc2_suit > cc1_suit)) {
        println!("CPU wins!");
    }
}
