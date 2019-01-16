use crate::api::Color;
pub trait Surface {
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn data_mut(&mut self) -> &mut [Color];
}