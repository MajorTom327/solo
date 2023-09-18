use solo::run;

use std::{
  error::Error,
  io::{self, stdout, Write},
  time::Duration,
};

use termion::{
  input::MouseTerminal,
  raw::IntoRawMode,
  screen::{AlternateScreen, ToMainScreen},
};

use ratatui::{backend::TermionBackend, Terminal};

/// This is a bare minimum example. There are many approaches to running an application loop, so
/// this is not meant to be prescriptive. It is only meant to demonstrate the basic setup and
/// teardown of a terminal application.
///
/// A more robust application would probably want to handle errors and ensure that the terminal is
/// restored to a sane state before exiting. This example does not do that. It also does not handle
/// events or update the application state. It just draws a greeting and exits when the user
/// presses 'q'.
fn main() -> Result<(), Box<dyn Error>> {
    std::panic::set_hook(Box::new(move |x| {
      stdout()
        .into_raw_mode()
        .unwrap()
        .suspend_raw_mode()
        .unwrap();
      write!(stdout().into_raw_mode().unwrap(), "{}", ToMainScreen).unwrap();
      print!("{:?}", x);
    }));

    let stdout = io::stdout().into_raw_mode()?;
    // let stdout = MouseTerminal::from(stdout);
    // let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;


    run(&mut terminal);

    // restore_terminal(&mut terminal).context("restore terminal failed")?;
    Ok(())
}
