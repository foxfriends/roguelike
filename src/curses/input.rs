use pancurses::{Window,Input as UInput};
use engine::{Direction,Inputter,Action};

pub struct Input<'a> {
    window: &'a Window,
}
impl<'a> Input<'a> {
    pub fn new(window: &'a Window) -> Input<'a> {
        Input{ window }
    }
}
impl<'a> Inputter for Input<'a> {
    fn get(&self) -> Action {
        let a = self.window.getch();
        match a {
            Some(UInput::Character('q')) => Action::Quit,
            Some(UInput::Character('z')) => Action::Interact(Direction::N),
            Some(UInput::Character('x')) => Action::Attack(Direction::N),
            Some(UInput::KeyUp) => Action::Move(Direction::N),
            Some(UInput::KeyDown) => Action::Move(Direction::S),
            Some(UInput::KeyLeft) => Action::Move(Direction::W),
            Some(UInput::KeyRight) => Action::Move(Direction::E),
            Some(_) => self.get(), // hope this gets tail call optimized...
            None => Action::Idle,
        }
    }
}
