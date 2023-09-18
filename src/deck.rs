use std::fmt::{Display, Formatter};

use colorize::AnsiColor;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Suit {
  Spades,
  Hearts,
  Clubs,
  Diamonds,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Value {
  Ace = 1,
  Two = 2,
  Three = 3,
  Four = 4,
  Five = 5,
  Six = 6,
  Seven = 7,
  Eight = 8,
  Nine = 9,
  Ten = 10,
  Jack = 11,
  Queen = 12,
  King = 13,
}

#[derive(Debug, Copy, Clone)]
pub struct Card {
  pub suit: Suit,
  pub value: Value,
  pub face_up: bool,
}

impl Card {
  pub fn new(suit: Suit, value: Value) -> Card {
    Card { suit, value, face_up: false }
  }

  pub fn is_red(&self) -> bool {
    match self.suit {
      Suit::Hearts | Suit::Diamonds => true,
      _ => false,
    }
  }

  pub fn set_visible(&mut self) {
    self.face_up = true;
  }

  pub fn can_move_over(&self, other: &Card) -> bool {
    if !other.face_up || !self.face_up {
      return false;
    }

    if self.is_red() == other.is_red() {
      return false;
    }


    if self.value as u8 == other.value as u8 - 1 {
      return true;
    }

    false
  }

  pub fn to_string(&self) -> String {
    if !self.face_up {
      return String::from("** *");
    }

    let value = match self.value {
      Value::Ace => "A",
      Value::Two => "2",
      Value::Three => "3",
      Value::Four => "4",
      Value::Five => "5",
      Value::Six => "6",
      Value::Seven => "7",
      Value::Eight => "8",
      Value::Nine => "9",
      Value::Ten => "10",
      Value::Jack => "J",
      Value::Queen => "Q",
      Value::King => "K",
    };

    let suit = match self.suit {
      Suit::Spades => "♠️",
      Suit::Hearts => "♥️",
      Suit::Clubs => "♣️",
      Suit::Diamonds => "♦️",
    };

    format!("{:>2} {}", value, suit)
  }
}

impl Display for Card {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {

    if !self.face_up {
      return write!(f, "{}", "[ ** * ]".b_grey());
    }

    let value = match self.value {
      Value::Ace => "A",
      Value::Two => "2",
      Value::Three => "3",
      Value::Four => "4",
      Value::Five => "5",
      Value::Six => "6",
      Value::Seven => "7",
      Value::Eight => "8",
      Value::Nine => "9",
      Value::Ten => "10",
      Value::Jack => "J",
      Value::Queen => "Q",
      Value::King => "K",
    };

    let suit = match self.suit {
      Suit::Spades => "♠️",
      Suit::Hearts => "♥️",
      Suit::Clubs => "♣️",
      Suit::Diamonds => "♦️",
    };

    if self.is_red() {
      return write!(f, "{}", format!("[ {:>2} {} ]", value, suit).red());
    } else {
      return write!(f, "{}", format!("[ {:>2} {} ]", value, suit).black());
    }
  }
}

#[derive(Debug)]
pub struct Deck {
  cards: Vec<Card>,
}

impl Deck {
  pub fn new() -> Deck {
    let mut cards = Vec::new();

    for suit in [Suit::Spades, Suit::Hearts, Suit::Clubs, Suit::Diamonds].iter() {
      for value in [
        Value::Ace,
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
      ]
      .iter()
      {
        cards.push(Card::new(*suit, *value));
      }
    }

    Deck { cards }
  }

  pub fn shuffle(&mut self) {
    use rand::seq::SliceRandom;
    let mut rng = rand::thread_rng();
    self.cards.shuffle(&mut rng);
  }

  pub fn deal(&mut self) -> Option<Card> {
    self.cards.pop()
  }

  pub fn len(&self) -> usize {
    self.cards.len()
  }

  pub fn is_empty(&self) -> bool {
    self.len() == 0
  }

  pub fn rotate_cards(&mut self) {
    self.cards.rotate_right(1);
    self.cards.last_mut().unwrap().set_visible();
  }

  pub fn last(&self) -> Option<&Card> {
    self.cards.last()
  }
}

impl Display for Deck {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    let mut output = String::new();

    for card in &self.cards {
      output.push_str(&format!("{} ", card));
    }

    write!(f, "{}", output)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn new_deck() {
    let deck = Deck::new();
    assert_eq!(deck.len(), 52);
  }

  #[test]
  fn deal_deck() {
    let mut deck = Deck::new();
    let card = deck.deal();
    assert_eq!(deck.len(), 51);
    assert_eq!(card.is_some(), true);
  }

  #[test]
  fn shuffle_deck() {
    let mut deck = Deck::new();
    let original_deck = deck.cards.clone();
    deck.shuffle();

    let mut same = true;
    for (i, card) in deck.cards.iter().enumerate() {
      if card.suit != original_deck[i].suit || card.value != original_deck[i].value {
        same = false;
        break;
      }
    }
    assert_eq!(same, false);
  }

  #[test]
  fn red_card() {
    let card = Card::new(Suit::Hearts, Value::Ace);
    assert_eq!(card.is_red(), true);
  }

  #[test]
  fn black_card() {
    let card = Card::new(Suit::Spades, Value::Ace);
    assert_eq!(card.is_red(), false);
  }

  #[test]
  fn can_move_over() {

    let mut card = Card::new(Suit::Spades, Value::Nine);
    let mut other = Card::new(Suit::Hearts, Value::Ten);

    card.set_visible();
    other.set_visible();

    assert_eq!(card.can_move_over(&other), true);
  }

  #[test]
  fn cannot_move_over() {
    let mut card = Card::new(Suit::Spades, Value::Ace);
    let mut other = Card::new(Suit::Hearts, Value::Three);

    card.set_visible();
    other.set_visible();

    assert_eq!(card.can_move_over(&other), false);
  }

  #[test]
  fn cannot_move_if_some_are_hidden() {
    let card = Card::new(Suit::Spades, Value::Two);
    let other = Card::new(Suit::Hearts, Value::Ace);

    assert_eq!(card.can_move_over(&other), false);
  }

  #[test]
  fn cannot_move_over_same_color() {
    let mut card = Card::new(Suit::Spades, Value::Two);
    let mut other = Card::new(Suit::Clubs, Value::Ace);

    card.set_visible();
    other.set_visible();

    assert_eq!(card.can_move_over(&other), false);
  }
}
