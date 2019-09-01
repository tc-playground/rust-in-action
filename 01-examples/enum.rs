#[derive(Copy, Clone, Debug)]
enum Suit {
    Clubs,
    Spades,
    Diamonds,
    Hearts,
}

#[derive(Copy, Clone, Debug)]
enum Card {
    King(Suit),
    Queen(Suit),
    Jack(Suit),
    Number(Suit, usize),
    Ace(Suit),
}

fn main() {
    let suits: [Suit; 4] = [Suit::Clubs, Suit::Spades, Suit::Diamonds, Suit::Hearts];
    let mut deck = Vec::new();
    for i in 0..suits.len() {
        let suit: Suit = suits[i];
        deck.push(Card::King(suit));
        deck.push(Card::Queen(suit));
        deck.push(Card::Jack(suit));
        for j in (2..11).rev() {
            deck.push(Card::Number(suit, j));
        } 
        deck.push(Card::Ace(suit));
    }

    for card in &deck {
        println!("{:?}", card);
    }

    println!("Deck size: {} cards", deck.len());
}