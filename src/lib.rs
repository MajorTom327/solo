use std::{ io::Stdout, time::Duration };

use anyhow::Result;
use termion::{ event::Key, raw::RawTerminal };

use ratatui::{
    backend::TermionBackend,
    Terminal,
    widgets::{ Paragraph, Block, Borders, ListItem, List },
    prelude::*,
};

mod event;
mod deck;
mod board;

use event::{ Event, Events };
use board::Board;
use deck::Suit;

struct Game {
    board: Board,
    pub should_quit: bool,

    pub selected: Option<(usize, usize)>,
    pub cursor: (usize, usize),
    pub objective_selected: u16,
}

impl Game {
    fn new() -> Game {
        Game {
            board: Board::new(),
            should_quit: false,
            selected: None,
            cursor: (0, 0),
            objective_selected: 0,
        }
    }

    fn on_key(&mut self, key: Key) {
        match key {
            Key::Up | Key::Char('k') => self.on_up(),
            Key::Down | Key::Char('j') => self.on_down(),
            Key::Left | Key::Char('h') => self.on_left(),
            Key::Right | Key::Char('l') => self.on_right(),
            Key::Char('\t') => self.on_tab(),
            Key::Char('\n') => self.on_enter(),
            Key::Char('w') => self.on_draw_card(),
            Key::Char('r') => self.on_retrieve_card(),
            Key::BackTab => self.on_backtab(),
            Key::Char(' ') => self.on_select(),
            _ => {/* do nothing */}
        }
    }

    fn on_up(&mut self) {
        if self.cursor.1 > 0 {
            self.cursor.1 -= 1;
        }
    }

    fn on_down(&mut self) {
        let col = self.board.game_cols.get(self.cursor.0);
        if let Some(col) = col {

            if self.cursor.1 + 1 < col.len() {
                self.cursor.1 += 1;
            }
        }
    }

    fn on_left(&mut self) {
        if self.cursor.0 > 0 {
            let col = self.board.game_cols.get(self.cursor.0 - 1);

            if let Some(col) = col {
                if col.len() <= self.cursor.1 {
                    self.cursor.1 = if col.len() > 0 { col.len() - 1 } else { 0 };
                }
            }
            self.cursor.0 -= 1;
        }
    }

    fn on_right(&mut self) {
        if self.cursor.0 < self.board.game_cols.len() - 1 {
            let col = self.board.game_cols.get(self.cursor.0 + 1);

            if let Some(col) = col {
                if col.len() <= self.cursor.1 {
                    self.cursor.1 = if col.len() > 0 { col.len() - 1 } else { 0 };
                }
            }

            self.cursor.0 += 1;
        }
    }

    fn on_tab(&mut self) {
        if self.objective_selected < 3 {
            self.objective_selected += 1;
        } else {
            self.objective_selected = 0;
        }
    }

    fn on_backtab(&mut self) {
        if self.objective_selected > 0 {
            self.objective_selected -= 1;
        } else {
            self.objective_selected = 3;
        }
    }

    fn on_select(&mut self) {
        if self.selected.is_none() {
            self.selected = Some(self.cursor);
        } else {
            let selected = self.selected.unwrap();

            let card_from = self.board.get_card(selected.0, selected.1);
            let card_to = self.board.get_card(self.cursor.0, self.cursor.1);

            if let Some(card_from) = card_from {
                if let Some(card_to) = card_to {
                    if card_from.can_move_over(&card_to) {
                        self.board.move_card(selected, self.cursor);
                    }
                } else {
                    // Move to an empty column
                    self.board.move_card(selected, self.cursor);
                }
            } else {
                self.board.move_card(selected, self.cursor);
            }
            self.selected = None;
        }
    }

    fn on_enter(&mut self) {
        self.board.add_to_objective(self.cursor, self.objective_selected as usize);

        let col = self.board.game_cols.get(self.cursor.0);
        if let Some(col) = col {
            if col.len() > 0 && self.cursor.1 > col.len() - 1 {
                self.cursor.1 = if col.len() > 0 { col.len() - 1 } else { 0 };
            }
        }
    }

    fn on_draw_card(&mut self) {
        self.board.draw_card()
    }

    fn on_retrieve_card(&mut self) {
        let cursor = self.cursor;

        let current_card = self.board.get_card(cursor.0, cursor.1);
        if let Some(current_card) = current_card {
            if current_card.face_up {
                let card = self.board.deck.last();
                if let Some(card) = card {
                    if card.can_move_over(&current_card) {
                        let card = self.board.deck.deal().unwrap();
                        self.board.game_cols[cursor.0].push(card);
                    }
                }
            }
        } else {
            let card = self.board.deck.deal().unwrap();
            self.board.game_cols[cursor.0].push(card);
        }
    }

    fn on_tick(&mut self) {
        // eprintln!("Cursor: {:?}", self.cursor);
        // self.board.tick();
    }

    fn render(&self, frame: &mut ratatui::Frame<TermionBackend<RawTerminal<Stdout>>>) {
        let objective_length = (self.board.objectives_cols.len() as u16) * 12;
        for x in 0..self.board.objectives_cols.len() {
            let objective = &self.board.objectives_cols.clone()[x];

            let card = objective.last();
            let card = if let Some(card) = card { card.to_string() } else { String::from("  ") };

            let x: u16 = x as u16;

            let card = Paragraph::new(card)
                .style(Style::default().fg(Color::White))
                .alignment(Alignment::Center)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(
                            Style::default().fg(
                                if self.objective_selected == x {
                                    Color::Yellow
                                } else {
                                    Color::White
                                }
                            )
                        )
                );

            frame.render_widget(card, Rect::new(x * 12, 0, 10, 3));
        }

        let deck_offset = objective_length + 4;
        let deck = Paragraph::new(format!("Deck: {}", self.board.deck.len()))
            .style(Style::default().fg(Color::White))
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::White))
            );

        frame.render_widget(deck, Rect::new(deck_offset, 0, 10, 3));

        if !self.board.deck.is_empty() {
            let last_card = self.board.deck.last();
            if let Some(last_card) = last_card {
                let deck_card = Paragraph::new(last_card.to_string())
                    .style(Style::default().fg(Color::White))
                    .alignment(Alignment::Center)
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .border_style(Style::default().fg(Color::White))
                    );

                frame.render_widget(deck_card, Rect::new(deck_offset + 12, 0, 10, 3));
            }
        }

        let board_offset = 4;

        // Game Board
        let nb_cols = self.board.game_cols.len();
        for x in 0..nb_cols {
            let game_col = self.board.game_cols[x].clone();

            let nb_cards = game_col.len();
            for y in 0..nb_cards {
                let card = game_col[y];
                let card_color = if card.face_up {
                    if self.selected == Some((x, y)) {
                        Color::LightBlue
                    } else {
                        match card.suit {
                            Suit::Spades | Suit::Clubs => Color::DarkGray,
                            Suit::Hearts | Suit::Diamonds => Color::Red,
                        }
                    }
                } else {
                    Color::Gray
                };

                let paragraph_style = if self.cursor == (x, y) {
                    Style::default().fg(card_color).bg(Color::LightGreen)
                } else {
                    Style::default().fg(card_color)
                };

                let card = Paragraph::new(card.to_string())
                    .style(paragraph_style)
                    .alignment(Alignment::Center)
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .border_style(Style::default().fg(card_color))
                    );
                frame.render_widget(
                    card,
                    Rect::new((x as u16) * 12, (y as u16) * 3 + board_offset, 10, 3)
                );
            }
        }

        // Keys binding tooltip
        self.render_tooltip(frame);
    }

    fn render_tooltip(&self, frame: &mut ratatui::Frame<TermionBackend<RawTerminal<Stdout>>>) {
        let items = [
            ListItem::new("q to quit"),
            ListItem::new("h/j/k/l to move"),
            ListItem::new("space to select"),
            ListItem::new("enter to move to objective"),
            ListItem::new("tab to change objective"),
            ListItem::new("tab+shift to change objective backwards"),
            ListItem::new("w to draw a card"),
            ListItem::new("r to retrieve the drawn card after cursor"),
        ];

        let nb_items = (items.len() as u16) + 2;
        let list = List::new(items)
            .block(Block::default().title("Keys").borders(Borders::ALL))
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
            .highlight_symbol(">>");

        let tooltip_size = 64;

        frame.render_widget(
            list,
            Rect::new(frame.size().width - tooltip_size, 0, tooltip_size, nb_items)
        );
    }
}

pub fn run(terminal: &mut Terminal<TermionBackend<RawTerminal<Stdout>>>) -> Result<()> {
    let mut app = Game::new();
    let events = Events::new(Duration::from_millis(33));
    terminal.clear()?;
    loop {
        terminal.draw(|f| app.render(f))?;
        match events.next()? {
            Event::Input(key) =>
                match key {
                    Key::Ctrl('c') | Key::Char('q') => {
                        app.should_quit = true;
                    }
                    _ => app.on_key(key),
                }
            Event::Tick => {
                app.on_tick();
            }
        }

        if app.should_quit {
            break;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use deck::{ Card, Suit, Value };

    mod test_select_objective {
        use super::*;

        #[test]
        fn set_next_objective() {
            let mut game = Game::new();
            game.objective_selected = 0;
            game.on_tab();
            assert_eq!(game.objective_selected, 1);
        }

        #[test]
        fn set_next_objective_when_on_last() {
            let mut game = Game::new();
            game.objective_selected = 3;
            game.on_tab();
            assert_eq!(game.objective_selected, 0);
        }

        #[test]
        fn set_previous_objective() {
            let mut game = Game::new();
            game.objective_selected = 1;
            game.on_backtab();
            assert_eq!(game.objective_selected, 0);
        }

        #[test]
        fn set_previous_objective_when_on_first() {
            let mut game = Game::new();
            game.objective_selected = 0;
            game.on_backtab();
            assert_eq!(game.objective_selected, 3);
        }

        #[test]
        fn add_ace_to_first_objective() {
            let mut game = Game::new();
            game.cursor = (0, 1);
            game.board.game_cols[0].push(Card::new(Suit::Spades, Value::Ace));

            game.on_enter();

            let objective_col: Vec<Card> = game.board.objectives_cols
                .get(game.objective_selected as usize)
                .unwrap()
                .clone()
                .into();

            assert_eq!(objective_col.len(), 1);

            assert_eq!(objective_col[0].value, Value::Ace);
            assert_eq!(objective_col[0].suit, Suit::Spades);
        }

        #[test]
        fn add_to_objective_set_empty_column() {
            let mut game = Game::new();
            game.cursor = (0, 0);
            game.board.game_cols[0].pop();
            game.board.game_cols[0].push(Card::new(Suit::Spades, Value::Ace));

            game.on_enter();

            let objective_col: Vec<Card> = game.board.objectives_cols
                .get(game.objective_selected as usize)
                .unwrap()
                .clone()
                .into();

            assert_eq!(objective_col.len(), 1);

            assert_eq!(objective_col[0].value, Value::Ace);
            assert_eq!(objective_col[0].suit, Suit::Spades);
        }
    }

    mod test_cursor_moving {
        use super::*;

        #[test]
        fn moving_up() {
            let mut game = Game::new();
            game.cursor = (0, 1);
            game.on_up();
            assert_eq!(game.cursor, (0, 0));
        }

        #[test]
        fn moving_up_when_on_top() {
            let mut game = Game::new();
            game.cursor = (0, 0);
            game.on_up();
            assert_eq!(game.cursor, (0, 0));
        }

        #[test]
        fn moving_down() {
            let mut game = Game::new();
            // Add a card for being able to move down
            game.board.game_cols[0].push(Card::new(Suit::Spades, Value::Ace));

            game.cursor = (0, 0);
            game.on_down();
            assert_eq!(game.cursor, (0, 1));
        }

        #[test]
        fn moving_down_when_on_bottom() {
            let mut game = Game::new();
            game.cursor = (0, 0);
            game.on_down();
            assert_eq!(game.cursor, (0, 0));
        }

        #[test]
        fn moving_left() {
            let mut game = Game::new();
            game.cursor = (1, 0);
            game.on_left();
            assert_eq!(game.cursor, (0, 0));
        }

        #[test]
        fn moving_left_when_on_left() {
            let mut game = Game::new();
            game.cursor = (0, 0);
            game.on_left();
            assert_eq!(game.cursor, (0, 0));
        }

        #[test]
        fn moving_left_to_shorter_column() {
            let mut game = Game::new();
            game.cursor = (2, 2);
            game.on_left();
            assert_eq!(game.cursor, (1, 1));
        }

        #[test]
        fn moving_left_to_shorter_base_column() {
            let mut game = Game::new();
            game.cursor = (1, 1);
            game.on_left();
            assert_eq!(game.cursor, (0, 0));
        }

        #[test]
        fn moving_left_hover_empty_column() {
            let mut game = Game::new();
            game.cursor = (2, 0);
            game.board.game_cols[1].clear();
            game.on_left();
            assert_eq!(game.cursor, (1, 0));
        }

        #[test]
        fn moving_right() {
            let mut game = Game::new();
            game.cursor = (0, 0);
            game.on_right();
            assert_eq!(game.cursor, (1, 0));
        }

        #[test]
        fn moving_right_when_on_right() {
            let mut game = Game::new();
            game.cursor = (6, 0);
            game.on_right();
            assert_eq!(game.cursor, (6, 0));
        }

        #[test]
        fn moving_right_to_shorter_column() {
            let mut game = Game::new();
            game.cursor = (1, 1);

            game.board.game_cols[2].pop();
            game.board.game_cols[2].pop();

            game.on_right();
            assert_eq!(game.cursor, (2, 0));
        }

        #[test]
        fn moving_right_hover_empty_column() {
            let mut game = Game::new();
            game.cursor = (0, 0);
            game.board.game_cols[1].clear();
            game.on_right();
            assert_eq!(game.cursor, (1, 0));
        }
    }

    mod test_move_card {
        use super::*;

        #[test]
        fn select_a_card() {
            let mut game = Game::new();
            game.cursor = (1, 1);
            game.on_select();
            assert_eq!(game.selected, Some((1, 1)));
        }

        #[test]
        fn unselect_a_card() {
            let mut game = Game::new();
            game.cursor = (1, 1);
            game.on_select();
            assert_eq!(game.selected, Some((1, 1)));
            game.on_select();
            assert_eq!(game.selected, None);
        }

        #[test]
        fn move_a_card() {
            let mut game = Game::new();

            let mut card_to_move = Card::new(Suit::Clubs, Value::Three);
            let mut card_to_receive = Card::new(Suit::Hearts, Value::Four);

            card_to_move.face_up = true;
            card_to_receive.face_up = true;

            game.board.game_cols[0].push(card_to_move);
            game.board.game_cols[1].push(card_to_receive);

            // Check select a card
            game.cursor = (0, 1);
            game.on_select();

            assert_eq!(game.selected, Some((0, 1)));

            // Move to the destination and move the card
            game.cursor = (1, 2);
            game.on_select();

            assert_eq!(game.selected, None);
            assert_eq!(game.board.game_cols[0].len(), 1);
            assert_eq!(game.board.game_cols[1].len(), 4);
        }

        #[test]
        fn move_a_card_to_empty_col() {
            let mut game = Game::new();

            let mut card_to_move = Card::new(Suit::Clubs, Value::King);

            card_to_move.face_up = true;

            game.board.game_cols[0].clear();
            game.board.game_cols[1].push(card_to_move);

            // Select the card
            game.cursor = (1, 2);
            game.on_select();

            assert_eq!(game.selected, Some((1, 2)));

            // Move to the empty column and move the card
            game.cursor = (0, 0);
            game.on_select();

            assert_eq!(game.selected, None);
            assert_eq!(game.board.game_cols[0].len(), 1);
            assert_eq!(game.board.game_cols[1].len(), 2);
        }
    }
}
