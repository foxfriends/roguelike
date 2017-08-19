use std::str::from_utf8;
use pancurses::{Window};
use engine::{Outputter,State};

pub struct Output<'a> {
    window: &'a Window,
}
impl<'a> Output<'a> {
    pub fn new(window: &'a Window) -> Output<'a> {
        Output{ window }
    }
}
impl<'a> Outputter for Output<'a> {
    fn render(&self, state: &State) {
        // TODO: better way to identify the Player
        let ref player = *state.map.tiles.iter()
            .clone()
            .flat_map(|t| t.contents().iter())
            .find(|a| a.symbol() == '@')
            .expect("There must be a Player (Actor with symbol '@')!");
        let map_str: String = state.map.tiles.iter()
            .map(|ref tile| tile.symbol())
            .collect();
        if state.map.width > 0 {
            // TODO: large map panning
            self.window.mvaddstr(0, 1, &String::from_utf8(vec![b'-'; state.map.width]).unwrap());
            for (i, row) in map_str.as_bytes().chunks(state.map.width).map(|row| from_utf8(row).unwrap()).enumerate() {
                self.window.mvaddstr(i as i32 + 1, 1, &(row.to_owned()));
            }
            self.window.mvaddstr(state.map.height as i32 + 1, 1, &String::from_utf8(vec![b'-'; state.map.width]).unwrap());
            for i in 0..state.map.height as i32 + 2 {
                self.window.mvaddstr(i, 0, "|");
                self.window.mvaddstr(i, state.map.width as i32 + 1, "|");
            }
        }
        self.window.mvaddstr(state.map.height as i32 + 3, 0, &format!("Money: {}", player.money()));
        self.window.refresh();
    }
}
