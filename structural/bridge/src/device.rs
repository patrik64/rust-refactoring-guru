mod radio;
mod tv;

pub use radio::Radio;
pub use tv::Tv;

#[allow(dead_code)]
pub trait Device {
    fn is_enabled(&self) -> bool;
    fn enable(&mut self);
    fn disable(&mut self);
    fn set_volume(&mut self, percent: u8);
    fn volume(&self) -> u8;
    fn channel(&self) -> u16;
    fn set_channel(&mut self, channel: u16);
    fn print_status(&self);
}
