use termgame::{Controller, CharChunkMap, GameSettings, Game, GameEvent, SimpleEvent, KeyCode, run_game};

use std::collections::HashMap;
use std::error::Error;
use std::time::Duration;


/// This is a single "buffer".
struct Buffer {
    text: String,
    file: Option<String>,
}

impl Buffer {
    /// This creates a new Buffer, to use it you should run:
    /// ```rust
    /// Buffer::new()
    /// ```
    fn new(file: Option<String>) -> Buffer {
        Buffer {
            text: String::new(),
            file
        }
    }

    /// A [`CharChunkMap`] is how termgame stores characters.
    /// This converts a buffer into something which can be shown on screen.
    /// You will likely not need to change this function.
    fn chunkmap_from_textarea(&mut self, map: &mut CharChunkMap) {
        let (mut line, mut col) = (0, 0);
        for c in self.text.chars() {
            map.insert(col, line, c.into());
            col += 1;
            if c == '\n' {
                line += 1;
                col = 0;
            }
        }
    }

    /// Adds a char to the end of the buffer.
    fn push_char(&mut self, c: char) {
        self.text.push(c);
    }

    /// Removes the last char in the buffer.
    fn pop_char(&mut self) -> Option<char> {
        self.text.pop()
    }

    // /// This is an example of a function that takes the Buffer as owned,
    // /// as well as another text area; and returns a new Buffer.
    // /// You would either need to return a `Buffer`, or be sure that
    // /// the user will not want the `Buffer` anymore.
    // fn example_owned(self, another_arg: Buffer) -> Buffer {
    //     todo!()
    // }

    // /// This is an example of a function that takes the Buffer by
    // /// mutable reference.
    // fn example_ref_mut(&mut self, another_arg: i32) {
    //     todo!()
    // }

    // /// This is an example of a function that takes the Buffer by
    // /// reference.
    // fn example_ref(&self) -> i32 {
    //     todo!()
    // }

}

/// This struct implements all the
/// logic for how the editor should work. It
/// implements "Controller", which defines how
/// something should interact with the terminal.
struct BufferEditor {
    buffer: Buffer,
}

impl Controller for BufferEditor {
    /// This gets run once, you can probably ignore it.
    fn on_start(&mut self, _game: &mut Game) {
    }

    /// Any time there's a keypress, you'll get this
    /// function called.
    fn on_event(&mut self, game: &mut Game, event: GameEvent) {
        match event.into() {
            SimpleEvent::Just(KeyCode::Char(c)) => {
                self.buffer.push_char(c)
            },
            SimpleEvent::Just(KeyCode::Enter) => {
                self.buffer.push_char('\n')
            },
            SimpleEvent::Just(KeyCode::Esc) => {
                game.end_game();
            },
            SimpleEvent::Just(KeyCode::Up) => {
                let mut viewport = game.get_viewport();
                if viewport.y > 0 {
                    viewport.y -= 1;
                }
                game.set_viewport(viewport)
            },
            SimpleEvent::Just(KeyCode::Down) => {
                let mut viewport = game.get_viewport();
                viewport.y += 1;
                game.set_viewport(viewport)
            },
            SimpleEvent::WithControl(KeyCode::Char('s')) => {
                // Implement
                println!("Saving...");
                // todo!()
            },
            // This is bad binding
            SimpleEvent::WithControl(KeyCode::Char('f')) => {
                // Clear entire line
                loop {
                    match self.buffer.pop_char() {
                        Some('\n') => break,
                        Some(_) => {},
                        None => break,
                    }
                }
            },
            SimpleEvent::Just(KeyCode::Backspace) => {
                self.buffer.pop_char();
            },

            _ => {}
        }
        let mut chunkmap = CharChunkMap::new();
        self.buffer.chunkmap_from_textarea(&mut chunkmap);
        game.swap_chunkmap(&mut chunkmap);

    }

    /// This function gets called regularly, so you can use it
    /// for logic that's independent of key-presses like
    /// implementing a "mouse".
    fn on_tick(&mut self, _game: &mut Game) {}
}

fn run_command(cmd: &str, buffers: &mut HashMap<String, Buffer>)  -> Result<(), Box<dyn Error>> {

    let mut editors: HashMap<String, BufferEditor> = HashMap::new();

    if cmd.starts_with("open") {
        run_game(
            fetch_editor(&mut editors, &cmd),
            GameSettings::new()
                .tick_duration(Duration::from_millis(25))
        )?;
    } else {
        println!("Command not recognised!");
    }

    Ok(())
}

fn fetch_editor(editors: &mut HashMap<String, BufferEditor>, cmd: &str) -> &mut BufferEditor {
    let mut args = cmd.split_whitespace();
    args.next();
    let file = args.next().unwrap();
    if !editors.contains_key(file) {
        editors.insert(file.to_string(), BufferEditor {
            buffer: Buffer::new(Some(file.to_string()))
        });
    }
    editors.get_mut(file).unwrap()
}


use rustyline::error::ReadlineError;
use rustyline::Editor;

fn main() -> Result<(), Box<dyn Error>> {

    println!("Welcome to BuffeRS. ");

    let mut buffers: HashMap<String, Buffer> = HashMap::new();

    // `()` can be used when no completer is required
    let mut rl = Editor::<()>::new()?;
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                run_command(&line, &mut buffers)?;
                rl.add_history_entry(line.as_str());
            },
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }


    Ok(())
}
