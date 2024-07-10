use std::io::{self, Write};
use crossterm::{
    cursor, event::{self, Event}, execute, queue, style::{self, Stylize}, terminal::{self, ClearType}
};

type Uint = u16;

pub struct Pos {
    pub x: Uint,
    pub y: Uint,
}

impl From<(Uint, Uint)> for Pos {
    fn from((x ,y): (Uint, Uint)) -> Self {
        Self { x, y }
    }
} 

fn draw_border(out: &mut io::Stdout, min: Pos, max: Pos) -> io::Result<()> {
    for y in 0..max.y {
        for x in 0..max.x {
            if (y == min.y || y == max.y - 1) || (x == min.x || x == max.x - 1) {
                queue!(out, cursor::MoveTo(x,y), style::PrintStyledContent("$".yellow()))?;
            }
        }
    }
    out.flush()?;
        
    Ok(())
}

fn main() -> io::Result<()> {
    let mut out = io::stdout();
    terminal::enable_raw_mode()?;
    execute!(out, terminal::EnterAlternateScreen, terminal::Clear(ClearType::All))?;

    loop {
        let (width, height) = terminal::size()?;
        draw_border(&mut out, (0,0).into(), (width, height).into())?;
        match event::read()? {
            Event::Key(_event) => { break }
            _ => {}
        }
    }

    execute!(out, terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    Ok(())
}
