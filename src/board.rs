use crate::deck::{Deck, Card, Value};
use std::fmt::{Display, Formatter, Result};

pub struct Board {
  pub game_cols: Vec<Vec<Card>>,
  pub deck: Deck,

  pub objectives_cols: Vec<Vec<Card>>
}

impl Board {
  pub fn new() -> Board {
    let mut deck = Deck::new();
    deck.shuffle();

    let game_cols = 7;
    let mut game: Vec<Vec<Card>> = Vec::new();

    for _ in 0..game_cols {
      game.push(Vec::new());
    }

    for i in 0..game_cols {
      for j in 0..i + 1 {
        let card = deck.deal();
        if let Some(card) = card {
          let mut card = card;
          if j == i {
            card.set_visible();
          }
          game[i].push(card);
        }
      }
    }

    Board {
      game_cols: game,
      deck,
      objectives_cols: vec![vec![]; 4]
    }
  }

  pub fn get_card(&self, x: usize, y: usize) -> Option<&Card> {
    let col = self.game_cols.get(x);

    if let Some(col) = col {
      col.get(y)
    }
    else {
      None
    }
  }

    pub fn move_card(&mut self, from: (usize, usize), to: (usize, usize)) {

    let (from_x, from_y) = from;
    let (to_x, _) = to;


    while self.game_cols[from_x].len() > from_y {
      let card = self.game_cols[from_x].remove(from_y);

      self.game_cols[to_x].push(card);
    }
    let last_card = self.game_cols[from_x].last_mut();

    if let Some(card) = last_card {
      card.set_visible();
    }
  }

    pub fn add_to_objective(&mut self, cursor: (usize, usize), objective_selected: usize) {
      // let card = self.game_cols[cursor.0][cursor.1].clone();
      let card = self.get_card(cursor.0, cursor.1);
      if card.is_none() {
        return;
      }
      let card = card.unwrap().clone();

      if self.objectives_cols[objective_selected].is_empty() {
        if card.value == Value::Ace {
          self.objectives_cols[objective_selected].push(card);
          self.game_cols[cursor.0].remove(cursor.1);
        }
      } else {
        let last_card = self.objectives_cols[objective_selected].last().unwrap();
        let card_value = card.value as i8;
        let last_card_value = last_card.value as i8;

        if card.suit == last_card.suit && card_value == last_card_value + 1 {
          self.objectives_cols[objective_selected].push(card);
          self.game_cols[cursor.0].remove(cursor.1);
        }
      }

      let last_card = self.game_cols[cursor.0].last_mut();
      if let Some(card) = last_card {
        card.set_visible();
      }
    }

    pub fn draw_card(&mut self) {
      self.deck.rotate_cards();
    }

}

impl Display for Board {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    let mut output = String::new();

    output.push_str(&format!("Deck: {} left", self.deck.len()));
    output.push_str("\n");

    for col in &self.objectives_cols {
      let card = col.last();
      if let Some(card) = card {
        output.push_str(&format!("{} ", card));
      }
    }
    output.push_str("\n\n");

    let max_columns = self.game_cols.iter().map(|col| col.len()).max().unwrap_or(0);
    for i in 0..max_columns {
      for col in &self.game_cols {
        let card = col.get(i);
        if let Some(card) = card {
          output.push_str(&format!("{} ", card));
        } else {
          output.push_str("         ");
        }
      }
      output.push_str("\n");
    }

    write!(f, "{}", output)
  }

}
