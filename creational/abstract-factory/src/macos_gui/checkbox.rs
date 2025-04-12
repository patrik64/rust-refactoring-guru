use crate::gui::Checkbox;

pub struct MacCheckbox;

impl Checkbox for MacCheckbox {
    fn switch(&self) {
        println!("macos checkbox pressed!!");
    }
}
