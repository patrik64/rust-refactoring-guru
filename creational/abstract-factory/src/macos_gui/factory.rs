//use crate::gui::GuiFactory;
use crate::gui::{Button, Checkbox, GuiFactoryDynamic};
use crate::macos_gui::button::MacButton;
use crate::macos_gui::checkbox::MacCheckbox;

pub struct MacFactory;

// impl GuiFactory for MacFactory {
//     type B = MacButton;
//     type C = MacCheckbox;

//     fn create_button(&self) -> MacButton {
//         println!("created macos button!!");
//         MacButton
//     }

//     fn create_checkbox(&self) -> MacCheckbox {
//         println!("created macos checkbox!!");
//         MacCheckbox
//     }
// }

impl GuiFactoryDynamic for MacFactory {
    fn create_button(&self) -> Box<dyn Button> {
        println!("created dynamic mac button!!");
        Box::new(MacButton)
    }

    fn create_checkbox(&self) -> Box<dyn Checkbox> {
        println!("created dynamic mac checkbox!!");
        Box::new(MacCheckbox)
    }
}
