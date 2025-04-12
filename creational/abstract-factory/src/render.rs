//! The code demonstrates that it doesn't depend on a concrete
//! factory implementation.
//use crate::gui::GuiFactory;

// Renders GUI. Factory object must be passed as a parameter to such the
// generic function with factory invocation to utilize static dispatch.
// pub fn render(factory: impl GuiFactory) {
//     let button1 = factory.create_button();
//     let button2 = factory.create_button();
//     let checkbox1 = factory.create_checkbox();
//     let checkbox2 = factory.create_checkbox();

//     use crate::gui::{Button, Checkbox};

//     button1.press();
//     button2.press();
//     checkbox1.switch();
//     checkbox2.switch();
// }

use crate::gui::GuiFactoryDynamic;

pub fn render(factory: &dyn GuiFactoryDynamic) {
    let button1 = factory.create_button();
    let button2 = factory.create_button();
    let checkbox1 = factory.create_checkbox();
    let checkbox2 = factory.create_checkbox();

    button1.press();
    button2.press();
    checkbox1.switch();
    checkbox2.switch();
}
