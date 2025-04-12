//use crate::gui::GuiFactory;
use crate::gui::{Button, Checkbox, GuiFactoryDynamic};
use crate::windows_gui::button::WinButton;
use crate::windows_gui::checkbox::WinCheckbox;

pub struct WindowsFactory;

// impl GuiFactory for WindowsFactory {
//     type B = WinButton;
//     type C = WinCheckbox;

//     fn create_button(&self) -> WinButton {
//         println!("created windows button!!");
//         WinButton
//     }

//     fn create_checkbox(&self) -> WinCheckbox {
//         println!("created windows checkbox!!");
//         WinCheckbox
//     }
// }

impl GuiFactoryDynamic for WindowsFactory {
    fn create_button(&self) -> Box<dyn Button> {
        println!("created dynamic windows button!!");
        Box::new(WinButton)
    }

    fn create_checkbox(&self) -> Box<dyn Checkbox> {
        println!("created dynamic windows checkbox!!");
        Box::new(WinCheckbox)
    }
}
