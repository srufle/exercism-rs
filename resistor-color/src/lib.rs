use enum_iterator::IntoEnumIterator;
use int_enum::IntEnum;
// Better Be Right Or Your Great Big Values Go Wrong.
#[repr(usize)]
#[derive(Debug, PartialEq, IntEnum, IntoEnumIterator, Copy, Eq, Clone)]
pub enum ResistorColor {
    Black = 0,
    Brown = 1,
    Red = 2,
    Orange = 3,
    Yellow = 4,
    Green = 5,
    Blue = 6,
    Violet = 7,
    Grey = 8,
    White = 9,
}

pub fn color_to_value_1(color: ResistorColor) -> usize {
    color as usize
}

pub fn color_to_value(color: ResistorColor) -> usize {
    let value = ResistorColor::into_enum_iter().find(|rc| *rc == color);
    value.unwrap().int_value() as usize
}

pub fn value_to_color_string(value: usize) -> String {
    match ResistorColor::from_int(value) {
        Ok(resistor) => format!("{:?}", resistor),
        Err(_) => String::from("value out of range"),
    }
}

pub fn colors() -> Vec<ResistorColor> {
    ResistorColor::into_enum_iter().collect()
}
