use crate::gui::Button;

pub struct WinButton;

impl Button for WinButton {
    fn press(&self) {
        println!("windows button pressed!!");
    }
}
