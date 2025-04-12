use crate::gui::Checkbox;

pub struct WinCheckbox;

impl Checkbox for WinCheckbox {
    fn switch(&self) {
        println!("windows checkbox pressed!!");
    }
}
