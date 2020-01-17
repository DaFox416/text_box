extern crate termion;
extern crate read_input;

use termion::{clear, cursor, color};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use std::io::{Write, stdout};
use std::fmt;

use read_input::prelude::*;

struct TextBox {
    x: u8,
    y: u8,
    width: u8,
    height: u8,
    border: u8,
    title: String,
    lines: Vec<String>
}

impl fmt::Display for TextBox {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut stdout = stdout()
            .into_raw_mode()
            .unwrap();
        let x = self.x as u16;
        let y = self.y as u16;
        let width = self.width as u16;
        let height = self.height as u16;

        let mut b: Vec<char> = Vec::new();
        if self.border == 1 {
            b = vec!['─', '│', '┌', '┐', '┘', '└'];
        } else if self.border == 2 {
            b = vec!['═', '║', '╔', '╗', '╝', '╚'];
        } else {
            b = vec![' ', ' ', ' ', ' ', ' ', ' '];
        }

        write!(stdout, "{}", cursor::Goto(x, y)).unwrap();
        write!(f, "{}", b[2])?;
        write!(stdout, "{}", cursor::Goto(x + width + 1, y)).unwrap();
        write!(f, "{}", b[3])?;
        write!(stdout, "{}", cursor::Goto(x + width + 1, y + height + 1)).unwrap();
        write!(f, "{}", b[4])?;
        write!(stdout, "{}", cursor::Goto(x, y + height + 1)).unwrap();
        write!(f, "{}", b[5])?;

        for i in 0..width {
            write!(stdout, "{}", cursor::Goto(x + 1 + i, y)).unwrap();
            write!(f, "{}", b[0])?;
            write!(stdout, "{}", cursor::Goto(x + 1 + i, y + height + 1)).unwrap();
            write!(f, "{}", b[0])?;
        }
        
        for i in 0..height {
            write!(stdout, "{}", cursor::Goto(x, y + 1 + i)).unwrap();
            write!(f, "{}", b[1])?;
            if (i as usize) < self.lines.len() {
                write!(f, "{}", self.lines[i as usize])?;
            }
            write!(stdout, "{}", cursor::Goto(x + width + 1, y + 1 + i)).unwrap();
            write!(f, "{}", b[1])?;
        }
        write!(stdout, "{}", cursor::Goto(x + 1, y)).unwrap();
        write!(f, "{}", self.title)
    }
}

impl TextBox {
    /// Creates a new TextBox with the specified params.
    /// 
    /// # Example
    /// 
    /// ```
    /// let textbox = TextBox::new(
    ///   10, 10,
    ///   15, 6,
    ///   2,
    ///   "DANGER",
    ///   "Some children are playing with dangerous weapons."
    /// ).unwrap();
    /// ```
    /// 
    /// This creates a text box like the following:
    /// ```
    /// ╔DANGER═════════╗
    /// ║Some children  ║
    /// ║are playing    ║
    /// ║with dangerous ║
    /// ║weapons.       ║
    /// ║               ║
    /// ║               ║
    /// ╚═══════════════╝
    /// ```
    /// at the console coordinates (10, 10).
    /// 
    /// *Note: Termion use one-based coordinates, this means that the first point is (1, 1) at upside left corner.*
    pub fn new(x: u8, y: u8, width: u8, height: u8, border: u8, title: &str, text: &str) -> Option<TextBox> {
        if title.len() as u8 > width {
            eprintln!("ERROR: Title '{}' is too long for given width!", title);
            return None;
        }

        let mut lines: Vec<String> = Vec::new();
        let mut line = String::new();
        for word in text.split(' ') {
            if word.len() as u8 > width {
                eprintln!("ERROR: Word '{}' is too long for given width!", word);
                return None;
            } else {
                if (line.len() + word.len()) as u8 > width || word == "\n"  {
                    lines.push(line);
                    line = String::new();
                    if word != "\n" {
                        line.push_str(word);
                        line.push(' ');
                    }
                } else {
                    line.push_str(word);
                    line.push(' ');
                }
            }
        }
        lines.push(line);
        if lines.len() as u8 > height {
            eprintln!("ERROR: Total lines are greater than box height {} > {}!", lines.len(), height);
            return None;
        }

        Some( TextBox {
            x,
            y,
            width,
            height,
            border,
            title: title.to_string(),
            lines })
    }
}