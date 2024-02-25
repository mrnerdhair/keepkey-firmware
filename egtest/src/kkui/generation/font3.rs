#![allow(dead_code)]

use embedded_graphics::{image::ImageRaw, pixelcolor::BinaryColor};

pub const SADFACE_9X10: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b01111111, 0b0_0000000,
    0b10000000, 0b1_0000000,
    0b11010101, 0b1_0000000,
    0b10100010, 0b1_0000000,
    0b11010101, 0b1_0000000,
    0b10000000, 0b1_0000000,
    0b10011100, 0b1_0000000,
    0b10100010, 0b1_0000000,
    0b10000000, 0b1_0000000,
    0b01111111, 0b0_0000000,
], 9);

pub const SEGWIT_12X10: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b00001001, 0b0000_0000,
    0b00011101, 0b1000_0000,
    0b00110110, 0b1100_0000,
    0b01100010, 0b0110_0000,
    0b11001011, 0b0011_0000,
    0b11001001, 0b0011_0000,
    0b01100100, 0b0110_0000,
    0b00110110, 0b1100_0000,
    0b00011011, 0b1000_0000,
    0b00001001, 0b0000_0000,
], 12);

pub const UNLOCKED_12X10: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b01110000, 0b0000_0000,
    0b11111000, 0b0000_0000,
    0b10001000, 0b0000_0000,
    0b10001000, 0b0000_0000,
    0b00011111, 0b1100_0000,
    0b00011101, 0b1100_0000,
    0b00011101, 0b1100_0000,
    0b00011000, 0b1100_0000,
    0b00011101, 0b1100_0000,
    0b00011111, 0b1100_0000,
], 12);

pub const LOCKED_12X10: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b00000111, 0b0000_0000,
    0b00001111, 0b1000_0000,
    0b00001000, 0b1000_0000,
    0b00001000, 0b1000_0000,
    0b00011111, 0b1100_0000,
    0b00011101, 0b1100_0000,
    0b00011101, 0b1100_0000,
    0b00011000, 0b1100_0000,
    0b00011101, 0b1100_0000,
    0b00011111, 0b1100_0000,
], 12);

pub const PIN_FONT_0X31: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b0111_0000,
    0b1111_0000,
    0b1111_0000,
    0b0011_0000,
    0b0011_0000,
    0b0011_0000,
    0b0011_0000,
    0b0011_0000,
    0b0011_0000,
    0b0011_0000,
    0b0011_0000,
    0b0011_0000,
], 4);

pub const PIN_FONT_0X32: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b11111100,
    0b11111110,
    0b00000011,
    0b00000011,
    0b00000011,
    0b00111110,
    0b01111100,
    0b11000000,
    0b11000000,
    0b11000000,
    0b11111111,
    0b11111111,
], 8);

pub const PIN_FONT_0X33: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b11111100,
    0b11111110,
    0b00000011,
    0b00000011,
    0b00000011,
    0b01111110,
    0b01111110,
    0b00000011,
    0b00000011,
    0b00000011,
    0b11111110,
    0b11111100,
], 8);

pub const PIN_FONT_0X34: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b00011110,
    0b00011110,
    0b00110110,
    0b00110110,
    0b01100110,
    0b01100110,
    0b11000110,
    0b11111111,
    0b11111111,
    0b00000110,
    0b00000110,
    0b00000110,
], 8);

pub const PIN_FONT_0X35: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b11111111,
    0b11111111,
    0b11000000,
    0b11000000,
    0b11000000,
    0b11111100,
    0b01111110,
    0b00000011,
    0b00000011,
    0b00000011,
    0b11111110,
    0b01111100,
], 8);

pub const PIN_FONT_0X36: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b00111100,
    0b01111110,
    0b11000000,
    0b11000000,
    0b11000000,
    0b11111100,
    0b11111110,
    0b11000011,
    0b11000011,
    0b11000011,
    0b01111110,
    0b00111100,
], 8);

pub const PIN_FONT_0X37: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b11111111,
    0b11111111,
    0b00000011,
    0b00000011,
    0b00000110,
    0b00000110,
    0b00001100,
    0b00001100,
    0b00011000,
    0b00011000,
    0b00110000,
    0b00110000,
], 8);

pub const PIN_FONT_0X38: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b00111100,
    0b01111110,
    0b11000011,
    0b11000011,
    0b11000011,
    0b01111110,
    0b01111110,
    0b11000011,
    0b11000011,
    0b11000011,
    0b01111110,
    0b00111100,
], 8);

pub const PIN_FONT_0X39: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b00111100,
    0b01111110,
    0b11000011,
    0b11000011,
    0b11000011,
    0b01111111,
    0b00111111,
    0b00000011,
    0b00000011,
    0b00000011,
    0b01111110,
    0b00111100,
], 8);

pub const TITLE_FONT_0X20: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b00000_000,
    0b00000_000,
    0b00000_000,
    0b00000_000,
    0b00000_000,
    0b00000_000,
    0b00000_000,
    0b00000_000,
    0b00000_000,
    0b00000_000,
], 5);

pub const TITLE_FONT_0X21: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b000_00000,
    0b110_00000,
    0b110_00000,
    0b110_00000,
    0b110_00000,
    0b110_00000,
    0b000_00000,
    0b110_00000,
    0b000_00000,
    0b000_00000,
], 3);

pub const TITLE_FONT_0X22: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b00000_000,
    0b11110_000,
    0b11110_000,
    0b00000_000,
    0b00000_000,
    0b00000_000,
    0b00000_000,
    0b00000_000,
    0b00000_000,
    0b00000_000,
], 5);

pub const TITLE_FONT_0X23: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b00000000,
    0b01101100,
    0b11111110,
    0b01101100,
    0b01101100,
    0b11111110,
    0b01101100,
    0b00000000,
    0b00000000,
    0b00000000,
], 8);

pub const TITLE_FONT_0X24: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b0011000_0,
    0b0111110_0,
    0b1111000_0,
    0b1111000_0,
    0b0111100_0,
    0b0011110_0,
    0b0011110_0,
    0b1111100_0,
    0b0011000_0,
    0b0000000_0,
], 7);

pub const TITLE_FONT_0X25: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b00000000, 0b0_0000000,
    0b01100011, 0b0_0000000,
    0b11110110, 0b0_0000000,
    0b01101100, 0b0_0000000,
    0b00011000, 0b0_0000000,
    0b00110110, 0b0_0000000,
    0b01101111, 0b0_0000000,
    0b11000110, 0b0_0000000,
    0b00000000, 0b0_0000000,
    0b00000000, 0b0_0000000,
], 9);

pub const TITLE_FONT_0X26: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b00000000,
    0b01110000,
    0b11011000,
    0b11011000,
    0b01110000,
    0b11011110,
    0b11001100,
    0b01111110,
    0b00000000,
    0b00000000,
], 8);

pub const TITLE_FONT_0X27: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b000_00000,
    0b110_00000,
    0b110_00000,
    0b000_00000,
    0b000_00000,
    0b000_00000,
    0b000_00000,
    0b000_00000,
    0b000_00000,
    0b000_00000,
], 3);

pub const TITLE_FONT_0X28: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b0110_0000,
    0b1100_0000,
    0b1100_0000,
    0b1100_0000,
    0b1100_0000,
    0b1100_0000,
    0b1100_0000,
    0b1100_0000,
    0b0110_0000,
    0b0000_0000,
], 4);

pub const TITLE_FONT_0X29: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b1100_0000,
    0b0110_0000,
    0b0110_0000,
    0b0110_0000,
    0b0110_0000,
    0b0110_0000,
    0b0110_0000,
    0b0110_0000,
    0b1100_0000,
    0b0000_0000,
], 4);

pub const TITLE_FONT_0X2A: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b0011000_0,
    0b1111110_0,
    0b0111100_0,
    0b1111110_0,
    0b0011000_0,
    0b0000000_0,
    0b0000000_0,
    0b0000000_0,
    0b0000000_0,
    0b0000000_0,
], 7);

pub const TITLE_FONT_0X2B: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b0000000_0,
    0b0000000_0,
    0b0011000_0,
    0b0011000_0,
    0b1111110_0,
    0b0011000_0,
    0b0011000_0,
    0b0000000_0,
    0b0000000_0,
    0b0000000_0,
], 7);

pub const TITLE_FONT_0X2C: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b0000_0000,
    0b0000_0000,
    0b0000_0000,
    0b0000_0000,
    0b0000_0000,
    0b0000_0000,
    0b1110_0000,
    0b1110_0000,
    0b0110_0000,
    0b1100_0000,
], 4);

pub const TITLE_FONT_0X2D: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b0000000_0,
    0b0000000_0,
    0b0000000_0,
    0b0000000_0,
    0b1111110_0,
    0b0000000_0,
    0b0000000_0,
    0b0000000_0,
    0b0000000_0,
    0b0000000_0,
], 7);

pub const TITLE_FONT_0X2E: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b0000_0000,
    0b0000_0000,
    0b0000_0000,
    0b0000_0000,
    0b0000_0000,
    0b0000_0000,
    0b1110_0000,
    0b1110_0000,
    0b0000_0000,
    0b0000_0000,
], 4);

pub const TITLE_FONT_0X2F: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b00000000, 0b0_0000000,
    0b00000011, 0b0_0000000,
    0b00000110, 0b0_0000000,
    0b00001100, 0b0_0000000,
    0b00011000, 0b0_0000000,
    0b00110000, 0b0_0000000,
    0b01100000, 0b0_0000000,
    0b11000000, 0b0_0000000,
    0b00000000, 0b0_0000000,
    0b00000000, 0b0_0000000,
], 9);

pub const TITLE_FONT_0X30: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b0000000_0,
    0b0111100_0,
    0b1100110_0,
    0b1101110_0,
    0b1111110_0,
    0b1110110_0,
    0b1100110_0,
    0b0111100_0,
    0b0000000_0,
    0b0000000_0,
], 7);

pub const TITLE_FONT_0X31: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b0000_0000,
    0b1110_0000,
    0b0110_0000,
    0b0110_0000,
    0b0110_0000,
    0b0110_0000,
    0b0110_0000,
    0b0110_0000,
    0b0000_0000,
    0b0000_0000,
], 4);

pub const TITLE_FONT_0X32: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b0000000_0,
    0b1111100_0,
    0b0000110_0,
    0b0000110_0,
    0b0111100_0,
    0b1100000_0,
    0b1100000_0,
    0b1111110_0,
    0b0000000_0,
    0b0000000_0,
], 7);

pub const TITLE_FONT_0X33: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b0000000_0,
    0b1111100_0,
    0b0000110_0,
    0b0000110_0,
    0b0111100_0,
    0b0000110_0,
    0b0000110_0,
    0b1111100_0,
    0b0000000_0,
    0b0000000_0,
], 7);

pub const TITLE_FONT_0X34: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b0000000_0,
    0b0001100_0,
    0b0011100_0,
    0b0111100_0,
    0b1101100_0,
    0b1111110_0,
    0b0001100_0,
    0b0001100_0,
    0b0000000_0,
    0b0000000_0,
], 7);

pub const TITLE_FONT_0X35: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b0000000_0,
    0b1111110_0,
    0b1100000_0,
    0b1100000_0,
    0b1111100_0,
    0b0000110_0,
    0b0000110_0,
    0b1111100_0,
    0b0000000_0,
    0b0000000_0,
], 7);

pub const TITLE_FONT_0X36: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b0000000_0,
    0b0111100_0,
    0b1100000_0,
    0b1100000_0,
    0b1111100_0,
    0b1100110_0,
    0b1100110_0,
    0b0111100_0,
    0b0000000_0,
    0b0000000_0,
], 7);

pub const TITLE_FONT_0X37: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b0000000_0,
    0b1111110_0,
    0b0000110_0,
    0b0001100_0,
    0b0001100_0,
    0b0011000_0,
    0b0011000_0,
    0b0110000_0,
    0b0000000_0,
    0b0000000_0,
], 7);

pub const TITLE_FONT_0X38: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b0000000_0,
    0b0111100_0,
    0b1100110_0,
    0b1100110_0,
    0b0111100_0,
    0b1100110_0,
    0b1100110_0,
    0b0111100_0,
    0b0000000_0,
    0b0000000_0,
], 7);

pub const TITLE_FONT_0X39: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b0000000_0,
    0b0111100_0,
    0b1100110_0,
    0b1100110_0,
    0b0111110_0,
    0b0000110_0,
    0b0000110_0,
    0b0111100_0,
    0b0000000_0,
    0b0000000_0,
], 7);

pub const TITLE_FONT_0X3A: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b0000_0000,
    0b0000_0000,
    0b0000_0000,
    0b1110_0000,
    0b1110_0000,
    0b0000_0000,
    0b1110_0000,
    0b1110_0000,
    0b0000_0000,
    0b0000_0000,
], 4);

pub const TITLE_FONT_0X3B: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b0000_0000,
    0b0000_0000,
    0b0000_0000,
    0b1110_0000,
    0b1110_0000,
    0b0000_0000,
    0b1110_0000,
    0b1110_0000,
    0b0110_0000,
    0b1100_0000,
], 4);

pub const TITLE_FONT_0X3C: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b000000_00,
    0b000110_00,
    0b001100_00,
    0b011000_00,
    0b110000_00,
    0b011000_00,
    0b001100_00,
    0b000110_00,
    0b000000_00,
    0b000000_00,
], 6);

pub const TITLE_FONT_0X3D: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b0000000_0,
    0b0000000_0,
    0b0000000_0,
    0b1111110_0,
    0b0000000_0,
    0b1111110_0,
    0b0000000_0,
    0b0000000_0,
    0b0000000_0,
    0b0000000_0,
], 7);

pub const TITLE_FONT_0X3E: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b000000_00,
    0b110000_00,
    0b011000_00,
    0b001100_00,
    0b000110_00,
    0b001100_00,
    0b011000_00,
    0b110000_00,
    0b000000_00,
    0b000000_00,
], 6);

pub const TITLE_FONT_0X3F: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b0000000_0,
    0b0111100_0,
    0b1100110_0,
    0b0000110_0,
    0b0001100_0,
    0b0011000_0,
    0b0000000_0,
    0b0011000_0,
    0b0000000_0,
    0b0000000_0,
], 7);

pub const TITLE_FONT_0X40: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b00111110, 0b0_0000000,
    0b01100011, 0b0_0000000,
    0b11011101, 0b1_0000000,
    0b11000111, 0b1_0000000,
    0b11011111, 0b1_0000000,
    0b11110111, 0b1_0000000,
    0b11011111, 0b0_0000000,
    0b01100000, 0b0_0000000,
    0b00111110, 0b0_0000000,
    0b00000000, 0b0_0000000,
], 9);

pub const TITLE_FONT_0X41: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b0000000_0,
    0b0111100_0,
    0b1100110_0,
    0b1100110_0,
    0b1111110_0,
    0b1100110_0,
    0b1100110_0,
    0b1100110_0,
    0b0000000_0,
    0b0000000_0,
], 7);

pub const TITLE_FONT_0X42: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b0000000_0,
    0b1111100_0,
    0b1100110_0,
    0b1100110_0,
    0b1111100_0,
    0b1100110_0,
    0b1100110_0,
    0b1111100_0,
    0b0000000_0,
    0b0000000_0,
], 7);

pub const TITLE_FONT_0X43: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b0000000_0,
    0b0111100_0,
    0b1100110_0,
    0b1100000_0,
    0b1100000_0,
    0b1100000_0,
    0b1100110_0,
    0b0111100_0,
    0b0000000_0,
    0b0000000_0,
], 7);

pub const TITLE_FONT_0X44: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b0000000_0,
    0b1111100_0,
    0b1100110_0,
    0b1100110_0,
    0b1100110_0,
    0b1100110_0,
    0b1100110_0,
    0b1111100_0,
    0b0000000_0,
    0b0000000_0,
], 7);

pub const TITLE_FONT_0X45: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b0000000_0,
    0b1111110_0,
    0b1100000_0,
    0b1100000_0,
    0b1111100_0,
    0b1100000_0,
    0b1100000_0,
    0b1111110_0,
    0b0000000_0,
    0b0000000_0,
], 7);

pub const TITLE_FONT_0X46: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b0000000_0,
    0b1111110_0,
    0b1100000_0,
    0b1100000_0,
    0b1111100_0,
    0b1100000_0,
    0b1100000_0,
    0b1100000_0,
    0b0000000_0,
    0b0000000_0,
], 7);

pub const TITLE_FONT_0X47: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b0000000_0,
    0b0111100_0,
    0b1100110_0,
    0b1100000_0,
    0b1101110_0,
    0b1100110_0,
    0b1100110_0,
    0b0111110_0,
    0b0000000_0,
    0b0000000_0,
], 7);

pub const TITLE_FONT_0X48: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b0000000_0,
    0b1100110_0,
    0b1100110_0,
    0b1100110_0,
    0b1111110_0,
    0b1100110_0,
    0b1100110_0,
    0b1100110_0,
    0b0000000_0,
    0b0000000_0,
], 7);

pub const TITLE_FONT_0X49: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b00000_000,
    0b11110_000,
    0b01100_000,
    0b01100_000,
    0b01100_000,
    0b01100_000,
    0b01100_000,
    0b11110_000,
    0b00000_000,
    0b00000_000,
], 5);

pub const TITLE_FONT_0X4A: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b0000000_0,
    0b0000110_0,
    0b0000110_0,
    0b0000110_0,
    0b0000110_0,
    0b0000110_0,
    0b1100110_0,
    0b0111100_0,
    0b0000000_0,
    0b0000000_0,
], 7);

pub const TITLE_FONT_0X4B: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b0000000_0,
    0b1100110_0,
    0b1101100_0,
    0b1111000_0,
    0b1110000_0,
    0b1111000_0,
    0b1101100_0,
    0b1100110_0,
    0b0000000_0,
    0b0000000_0,
], 7);

pub const TITLE_FONT_0X4C: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b0000000_0,
    0b1100000_0,
    0b1100000_0,
    0b1100000_0,
    0b1100000_0,
    0b1100000_0,
    0b1100000_0,
    0b1111110_0,
    0b0000000_0,
    0b0000000_0,
], 7);

pub const TITLE_FONT_0X4D: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b00000000, 0b0_0000000,
    0b11000011, 0b0_0000000,
    0b11100111, 0b0_0000000,
    0b11111111, 0b0_0000000,
    0b11011011, 0b0_0000000,
    0b11000011, 0b0_0000000,
    0b11000011, 0b0_0000000,
    0b11000011, 0b0_0000000,
    0b00000000, 0b0_0000000,
    0b00000000, 0b0_0000000,
], 9);

pub const TITLE_FONT_0X4E: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b0000000_0,
    0b1100110_0,
    0b1100110_0,
    0b1110110_0,
    0b1111110_0,
    0b1101110_0,
    0b1100110_0,
    0b1100110_0,
    0b0000000_0,
    0b0000000_0,
], 7);

pub const TITLE_FONT_0X4F: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b0000000_0,
    0b0111100_0,
    0b1100110_0,
    0b1100110_0,
    0b1100110_0,
    0b1100110_0,
    0b1100110_0,
    0b0111100_0,
    0b0000000_0,
    0b0000000_0,
], 7);

pub const TITLE_FONT_0X50: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b0000000_0,
    0b1111100_0,
    0b1100110_0,
    0b1100110_0,
    0b1100110_0,
    0b1111100_0,
    0b1100000_0,
    0b1100000_0,
    0b0000000_0,
    0b0000000_0,
], 7);

pub const TITLE_FONT_0X51: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b0000000_0,
    0b0111100_0,
    0b1100110_0,
    0b1100110_0,
    0b1100110_0,
    0b1100110_0,
    0b1100110_0,
    0b0111100_0,
    0b0000110_0,
    0b0000000_0,
], 7);

pub const TITLE_FONT_0X52: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b0000000_0,
    0b1111100_0,
    0b1100110_0,
    0b1100110_0,
    0b1100110_0,
    0b1111100_0,
    0b1101100_0,
    0b1100110_0,
    0b0000000_0,
    0b0000000_0,
], 7);

pub const TITLE_FONT_0X53: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b0000000_0,
    0b0111100_0,
    0b1100110_0,
    0b1100000_0,
    0b0111100_0,
    0b0000110_0,
    0b1100110_0,
    0b0111100_0,
    0b0000000_0,
    0b0000000_0,
], 7);

pub const TITLE_FONT_0X54: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b0000000_0,
    0b1111110_0,
    0b0011000_0,
    0b0011000_0,
    0b0011000_0,
    0b0011000_0,
    0b0011000_0,
    0b0011000_0,
    0b0000000_0,
    0b0000000_0,
], 7);

pub const TITLE_FONT_0X55: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b0000000_0,
    0b1100110_0,
    0b1100110_0,
    0b1100110_0,
    0b1100110_0,
    0b1100110_0,
    0b1100110_0,
    0b0111100_0,
    0b0000000_0,
    0b0000000_0,
], 7);

pub const TITLE_FONT_0X56: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b0000000_0,
    0b1100110_0,
    0b1100110_0,
    0b1100110_0,
    0b1100110_0,
    0b0111100_0,
    0b0111100_0,
    0b0011000_0,
    0b0000000_0,
    0b0000000_0,
], 7);

pub const TITLE_FONT_0X57: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b00000000, 0b0_0000000,
    0b11011011, 0b0_0000000,
    0b11011011, 0b0_0000000,
    0b11011011, 0b0_0000000,
    0b11011011, 0b0_0000000,
    0b11011011, 0b0_0000000,
    0b11011011, 0b0_0000000,
    0b01111110, 0b0_0000000,
    0b00000000, 0b0_0000000,
    0b00000000, 0b0_0000000,
], 9);

pub const TITLE_FONT_0X58: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b0000000_0,
    0b1100110_0,
    0b1100110_0,
    0b0111100_0,
    0b0011000_0,
    0b0111100_0,
    0b1100110_0,
    0b1100110_0,
    0b0000000_0,
    0b0000000_0,
], 7);

pub const TITLE_FONT_0X59: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b0000000_0,
    0b1100110_0,
    0b1100110_0,
    0b1100110_0,
    0b0111100_0,
    0b0011000_0,
    0b0011000_0,
    0b0011000_0,
    0b0000000_0,
    0b0000000_0,
], 7);

pub const TITLE_FONT_0X5A: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b0000000_0,
    0b1111110_0,
    0b0000110_0,
    0b0001100_0,
    0b0011000_0,
    0b0110000_0,
    0b1100000_0,
    0b1111110_0,
    0b0000000_0,
    0b0000000_0,
], 7);

pub const TITLE_FONT_0X5B: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b00000_000,
    0b11110_000,
    0b11000_000,
    0b11000_000,
    0b11000_000,
    0b11000_000,
    0b11000_000,
    0b11110_000,
    0b00000_000,
    0b00000_000,
], 5);

pub const TITLE_FONT_0X5C: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b00000000, 0b0_0000000,
    0b11000000, 0b0_0000000,
    0b01100000, 0b0_0000000,
    0b00110000, 0b0_0000000,
    0b00011000, 0b0_0000000,
    0b00001100, 0b0_0000000,
    0b00000110, 0b0_0000000,
    0b00000011, 0b0_0000000,
    0b00000000, 0b0_0000000,
    0b00000000, 0b0_0000000,
], 9);

pub const TITLE_FONT_0X5D: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b00000_000,
    0b11110_000,
    0b00110_000,
    0b00110_000,
    0b00110_000,
    0b00110_000,
    0b00110_000,
    0b11110_000,
    0b00000_000,
    0b00000_000,
], 5);

pub const TITLE_FONT_0X5E: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b00000_000,
    0b01100_000,
    0b11110_000,
    0b00000_000,
    0b00000_000,
    0b00000_000,
    0b00000_000,
    0b00000_000,
    0b00000_000,
    0b00000_000,
], 5);

pub const TITLE_FONT_0X5F: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b0000000_0,
    0b0000000_0,
    0b0000000_0,
    0b0000000_0,
    0b0000000_0,
    0b0000000_0,
    0b0000000_0,
    0b1111110_0,
    0b0000000_0,
    0b0000000_0,
], 7);

pub const TITLE_FONT_0X60: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b0000_0000,
    0b1100_0000,
    0b0110_0000,
    0b0000_0000,
    0b0000_0000,
    0b0000_0000,
    0b0000_0000,
    0b0000_0000,
    0b0000_0000,
    0b0000_0000,
], 4);

pub const TITLE_FONT_0X61: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b0000000_0,
    0b0000000_0,
    0b0000000_0,
    0b0111100_0,
    0b0000110_0,
    0b0111110_0,
    0b1100110_0,
    0b0111110_0,
    0b0000000_0,
    0b0000000_0,
], 7);

pub const TITLE_FONT_0X62: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b0000000_0,
    0b1100000_0,
    0b1100000_0,
    0b1111100_0,
    0b1100110_0,
    0b1100110_0,
    0b1100110_0,
    0b1111100_0,
    0b0000000_0,
    0b0000000_0,
], 7);

pub const TITLE_FONT_0X63: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b0000000_0,
    0b0000000_0,
    0b0000000_0,
    0b0111110_0,
    0b1100000_0,
    0b1100000_0,
    0b1100000_0,
    0b0111110_0,
    0b0000000_0,
    0b0000000_0,
], 7);

pub const TITLE_FONT_0X64: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b0000000_0,
    0b0000110_0,
    0b0000110_0,
    0b0111110_0,
    0b1100110_0,
    0b1100110_0,
    0b1100110_0,
    0b0111110_0,
    0b0000000_0,
    0b0000000_0,
], 7);

pub const TITLE_FONT_0X65: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b0000000_0,
    0b0000000_0,
    0b0000000_0,
    0b0111100_0,
    0b1100110_0,
    0b1111110_0,
    0b1100000_0,
    0b0111110_0,
    0b0000000_0,
    0b0000000_0,
], 7);

pub const TITLE_FONT_0X66: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b000000_00,
    0b001110_00,
    0b011000_00,
    0b111110_00,
    0b011000_00,
    0b011000_00,
    0b011000_00,
    0b011000_00,
    0b000000_00,
    0b000000_00,
], 6);

pub const TITLE_FONT_0X67: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b0000000_0,
    0b0000000_0,
    0b0000000_0,
    0b0111110_0,
    0b1100110_0,
    0b1100110_0,
    0b1100110_0,
    0b0111110_0,
    0b0000110_0,
    0b0111100_0,
], 7);

pub const TITLE_FONT_0X68: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b0000000_0,
    0b1100000_0,
    0b1100000_0,
    0b1111100_0,
    0b1100110_0,
    0b1100110_0,
    0b1100110_0,
    0b1100110_0,
    0b0000000_0,
    0b0000000_0,
], 7);

pub const TITLE_FONT_0X69: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b000_00000,
    0b110_00000,
    0b000_00000,
    0b110_00000,
    0b110_00000,
    0b110_00000,
    0b110_00000,
    0b110_00000,
    0b000_00000,
    0b000_00000,
], 3);

pub const TITLE_FONT_0X6A: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b0000_0000,
    0b0110_0000,
    0b0000_0000,
    0b0110_0000,
    0b0110_0000,
    0b0110_0000,
    0b0110_0000,
    0b0110_0000,
    0b0110_0000,
    0b1100_0000,
], 4);

pub const TITLE_FONT_0X6B: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b000000_00,
    0b110000_00,
    0b110000_00,
    0b110110_00,
    0b111100_00,
    0b111000_00,
    0b111100_00,
    0b110110_00,
    0b000000_00,
    0b000000_00,
], 6);

pub const TITLE_FONT_0X6C: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b000_00000,
    0b110_00000,
    0b110_00000,
    0b110_00000,
    0b110_00000,
    0b110_00000,
    0b110_00000,
    0b110_00000,
    0b000_00000,
    0b000_00000,
], 3);

pub const TITLE_FONT_0X6D: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b00000000, 0b0_0000000,
    0b00000000, 0b0_0000000,
    0b00000000, 0b0_0000000,
    0b11111110, 0b0_0000000,
    0b11011011, 0b0_0000000,
    0b11011011, 0b0_0000000,
    0b11011011, 0b0_0000000,
    0b11011011, 0b0_0000000,
    0b00000000, 0b0_0000000,
    0b00000000, 0b0_0000000,
], 9);

pub const TITLE_FONT_0X6E: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b0000000_0,
    0b0000000_0,
    0b0000000_0,
    0b1111100_0,
    0b1100110_0,
    0b1100110_0,
    0b1100110_0,
    0b1100110_0,
    0b0000000_0,
    0b0000000_0,
], 7);

pub const TITLE_FONT_0X6F: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b0000000_0,
    0b0000000_0,
    0b0000000_0,
    0b0111100_0,
    0b1100110_0,
    0b1100110_0,
    0b1100110_0,
    0b0111100_0,
    0b0000000_0,
    0b0000000_0,
], 7);

pub const TITLE_FONT_0X70: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b0000000_0,
    0b0000000_0,
    0b0000000_0,
    0b1111100_0,
    0b1100110_0,
    0b1100110_0,
    0b1100110_0,
    0b1111100_0,
    0b1100000_0,
    0b1100000_0,
], 7);

pub const TITLE_FONT_0X71: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b0000000_0,
    0b0000000_0,
    0b0000000_0,
    0b0111110_0,
    0b1100110_0,
    0b1100110_0,
    0b1100110_0,
    0b0111110_0,
    0b0000110_0,
    0b0000110_0,
], 7);

pub const TITLE_FONT_0X72: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b000000_00,
    0b000000_00,
    0b000000_00,
    0b111110_00,
    0b111000_00,
    0b110000_00,
    0b110000_00,
    0b110000_00,
    0b000000_00,
    0b000000_00,
], 6);

pub const TITLE_FONT_0X73: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b0000000_0,
    0b0000000_0,
    0b0000000_0,
    0b0111110_0,
    0b1100000_0,
    0b0111100_0,
    0b0000110_0,
    0b1111100_0,
    0b0000000_0,
    0b0000000_0,
], 7);

pub const TITLE_FONT_0X74: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b000000_00,
    0b011000_00,
    0b011000_00,
    0b111110_00,
    0b011000_00,
    0b011000_00,
    0b011000_00,
    0b001110_00,
    0b000000_00,
    0b000000_00,
], 6);

pub const TITLE_FONT_0X75: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b0000000_0,
    0b0000000_0,
    0b0000000_0,
    0b1100110_0,
    0b1100110_0,
    0b1100110_0,
    0b1100110_0,
    0b0111110_0,
    0b0000000_0,
    0b0000000_0,
], 7);

pub const TITLE_FONT_0X76: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b0000000_0,
    0b0000000_0,
    0b0000000_0,
    0b1100110_0,
    0b1100110_0,
    0b0111100_0,
    0b0111100_0,
    0b0011000_0,
    0b0000000_0,
    0b0000000_0,
], 7);

pub const TITLE_FONT_0X77: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b00000000, 0b0_0000000,
    0b00000000, 0b0_0000000,
    0b00000000, 0b0_0000000,
    0b11011011, 0b0_0000000,
    0b11011011, 0b0_0000000,
    0b11011011, 0b0_0000000,
    0b11011011, 0b0_0000000,
    0b01111110, 0b0_0000000,
    0b00000000, 0b0_0000000,
    0b00000000, 0b0_0000000,
], 9);

pub const TITLE_FONT_0X78: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b0000000_0,
    0b0000000_0,
    0b0000000_0,
    0b1100110_0,
    0b0111100_0,
    0b0011000_0,
    0b0111100_0,
    0b1100110_0,
    0b0000000_0,
    0b0000000_0,
], 7);

pub const TITLE_FONT_0X79: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b0000000_0,
    0b0000000_0,
    0b0000000_0,
    0b1100110_0,
    0b1100110_0,
    0b1100110_0,
    0b1100110_0,
    0b0111110_0,
    0b0000110_0,
    0b0111100_0,
], 7);

pub const TITLE_FONT_0X7A: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b0000000_0,
    0b0000000_0,
    0b0000000_0,
    0b1111110_0,
    0b0001100_0,
    0b0011000_0,
    0b0110000_0,
    0b1111110_0,
    0b0000000_0,
    0b0000000_0,
], 7);

pub const TITLE_FONT_0X7B: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b000000_00,
    0b001110_00,
    0b011000_00,
    0b011000_00,
    0b110000_00,
    0b011000_00,
    0b011000_00,
    0b001110_00,
    0b000000_00,
    0b000000_00,
], 6);

pub const TITLE_FONT_0X7C: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b000_00000,
    0b110_00000,
    0b110_00000,
    0b110_00000,
    0b110_00000,
    0b110_00000,
    0b110_00000,
    0b110_00000,
    0b000_00000,
    0b000_00000,
], 3);

pub const TITLE_FONT_0X7D: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b000000_00,
    0b111000_00,
    0b001100_00,
    0b001100_00,
    0b000110_00,
    0b001100_00,
    0b001100_00,
    0b111000_00,
    0b000000_00,
    0b000000_00,
], 6);

pub const TITLE_FONT_0X7E: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b0000000_0,
    0b0111110_0,
    0b1111100_0,
    0b0000000_0,
    0b0000000_0,
    0b0000000_0,
    0b0000000_0,
    0b0000000_0,
    0b0000000_0,
    0b0000000_0,
], 7);

pub const BODY_FONT_0X20: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b0000_0000,
    0b0000_0000,
    0b0000_0000,
    0b0000_0000,
    0b0000_0000,
    0b0000_0000,
    0b0000_0000,
    0b0000_0000,
    0b0000_0000,
    0b0000_0000,
], 4);

pub const BODY_FONT_0X21: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b00_000000,
    0b10_000000,
    0b10_000000,
    0b10_000000,
    0b10_000000,
    0b10_000000,
    0b00_000000,
    0b10_000000,
    0b00_000000,
    0b00_000000,
], 2);

pub const BODY_FONT_0X22: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b0000_0000,
    0b1010_0000,
    0b1010_0000,
    0b0000_0000,
    0b0000_0000,
    0b0000_0000,
    0b0000_0000,
    0b0000_0000,
    0b0000_0000,
    0b0000_0000,
], 4);

pub const BODY_FONT_0X23: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b0000000_0,
    0b0100100_0,
    0b1111110_0,
    0b0100100_0,
    0b0100100_0,
    0b1111110_0,
    0b0100100_0,
    0b0000000_0,
    0b0000000_0,
    0b0000000_0,
], 7);

pub const BODY_FONT_0X24: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b001000_00,
    0b011110_00,
    0b101000_00,
    0b101000_00,
    0b011100_00,
    0b001010_00,
    0b001010_00,
    0b111100_00,
    0b001000_00,
    0b000000_00,
], 6);

pub const BODY_FONT_0X25: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b00000000,
    0b01000010,
    0b10100100,
    0b01001000,
    0b00010000,
    0b00100100,
    0b01001010,
    0b10000100,
    0b00000000,
    0b00000000,
], 8);

pub const BODY_FONT_0X26: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b0000000_0,
    0b0110000_0,
    0b1001000_0,
    0b1001000_0,
    0b0110000_0,
    0b1001010_0,
    0b1000100_0,
    0b0111010_0,
    0b0000000_0,
    0b0000000_0,
], 7);

pub const BODY_FONT_0X27: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b00_000000,
    0b10_000000,
    0b10_000000,
    0b00_000000,
    0b00_000000,
    0b00_000000,
    0b00_000000,
    0b00_000000,
    0b00_000000,
    0b00_000000,
], 2);

pub const BODY_FONT_0X28: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b010_00000,
    0b100_00000,
    0b100_00000,
    0b100_00000,
    0b100_00000,
    0b100_00000,
    0b100_00000,
    0b100_00000,
    0b010_00000,
    0b000_00000,
], 3);

pub const BODY_FONT_0X29: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b100_00000,
    0b010_00000,
    0b010_00000,
    0b010_00000,
    0b010_00000,
    0b010_00000,
    0b010_00000,
    0b010_00000,
    0b100_00000,
    0b000_00000,
], 3);

pub const BODY_FONT_0X2A: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b001000_00,
    0b101010_00,
    0b011100_00,
    0b101010_00,
    0b001000_00,
    0b000000_00,
    0b000000_00,
    0b000000_00,
    0b000000_00,
    0b000000_00,
], 6);

pub const BODY_FONT_0X2B: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b000000_00,
    0b000000_00,
    0b001000_00,
    0b001000_00,
    0b111110_00,
    0b001000_00,
    0b001000_00,
    0b000000_00,
    0b000000_00,
    0b000000_00,
], 6);

pub const BODY_FONT_0X2C: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b000_00000,
    0b000_00000,
    0b000_00000,
    0b000_00000,
    0b000_00000,
    0b000_00000,
    0b110_00000,
    0b110_00000,
    0b010_00000,
    0b100_00000,
], 3);

pub const BODY_FONT_0X2D: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b000000_00,
    0b000000_00,
    0b000000_00,
    0b000000_00,
    0b111110_00,
    0b000000_00,
    0b000000_00,
    0b000000_00,
    0b000000_00,
    0b000000_00,
], 6);

pub const BODY_FONT_0X2E: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b000_00000,
    0b000_00000,
    0b000_00000,
    0b000_00000,
    0b000_00000,
    0b000_00000,
    0b110_00000,
    0b110_00000,
    0b000_00000,
    0b000_00000,
], 3);

pub const BODY_FONT_0X2F: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b00000000,
    0b00000010,
    0b00000100,
    0b00001000,
    0b00010000,
    0b00100000,
    0b01000000,
    0b10000000,
    0b00000000,
    0b00000000,
], 8);

pub const BODY_FONT_0X30: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b000000_00,
    0b011100_00,
    0b100010_00,
    0b100110_00,
    0b101010_00,
    0b110010_00,
    0b100010_00,
    0b011100_00,
    0b000000_00,
    0b000000_00,
], 6);

pub const BODY_FONT_0X31: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b000_00000,
    0b110_00000,
    0b010_00000,
    0b010_00000,
    0b010_00000,
    0b010_00000,
    0b010_00000,
    0b010_00000,
    0b000_00000,
    0b000_00000,
], 3);

pub const BODY_FONT_0X32: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b000000_00,
    0b111100_00,
    0b000010_00,
    0b000010_00,
    0b011100_00,
    0b100000_00,
    0b100000_00,
    0b111110_00,
    0b000000_00,
    0b000000_00,
], 6);

pub const BODY_FONT_0X33: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b000000_00,
    0b111100_00,
    0b000010_00,
    0b000010_00,
    0b011100_00,
    0b000010_00,
    0b000010_00,
    0b111100_00,
    0b000000_00,
    0b000000_00,
], 6);

pub const BODY_FONT_0X34: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b000000_00,
    0b000100_00,
    0b001100_00,
    0b010100_00,
    0b100100_00,
    0b111110_00,
    0b000100_00,
    0b000100_00,
    0b000000_00,
    0b000000_00,
], 6);

pub const BODY_FONT_0X35: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b000000_00,
    0b111110_00,
    0b100000_00,
    0b100000_00,
    0b111100_00,
    0b000010_00,
    0b000010_00,
    0b111100_00,
    0b000000_00,
    0b000000_00,
], 6);

pub const BODY_FONT_0X36: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b000000_00,
    0b011100_00,
    0b100000_00,
    0b100000_00,
    0b111100_00,
    0b100010_00,
    0b100010_00,
    0b011100_00,
    0b000000_00,
    0b000000_00,
], 6);

pub const BODY_FONT_0X37: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b000000_00,
    0b111110_00,
    0b000010_00,
    0b000100_00,
    0b000100_00,
    0b001000_00,
    0b001000_00,
    0b010000_00,
    0b000000_00,
    0b000000_00,
], 6);

pub const BODY_FONT_0X38: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b000000_00,
    0b011100_00,
    0b100010_00,
    0b100010_00,
    0b011100_00,
    0b100010_00,
    0b100010_00,
    0b011100_00,
    0b000000_00,
    0b000000_00,
], 6);

pub const BODY_FONT_0X39: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b000000_00,
    0b011100_00,
    0b100010_00,
    0b100010_00,
    0b011110_00,
    0b000010_00,
    0b000010_00,
    0b011100_00,
    0b000000_00,
    0b000000_00,
], 6);

pub const BODY_FONT_0X3A: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b000_00000,
    0b000_00000,
    0b000_00000,
    0b110_00000,
    0b110_00000,
    0b000_00000,
    0b110_00000,
    0b110_00000,
    0b000_00000,
    0b000_00000,
], 3);

pub const BODY_FONT_0X3B: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b000_00000,
    0b000_00000,
    0b000_00000,
    0b110_00000,
    0b110_00000,
    0b000_00000,
    0b110_00000,
    0b110_00000,
    0b010_00000,
    0b100_00000,
], 3);

pub const BODY_FONT_0X3C: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b00000_000,
    0b00010_000,
    0b00100_000,
    0b01000_000,
    0b10000_000,
    0b01000_000,
    0b00100_000,
    0b00010_000,
    0b00000_000,
    0b00000_000,
], 5);

pub const BODY_FONT_0X3D: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b000000_00,
    0b000000_00,
    0b000000_00,
    0b111110_00,
    0b000000_00,
    0b111110_00,
    0b000000_00,
    0b000000_00,
    0b000000_00,
    0b000000_00,
], 6);

pub const BODY_FONT_0X3E: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b00000_000,
    0b10000_000,
    0b01000_000,
    0b00100_000,
    0b00010_000,
    0b00100_000,
    0b01000_000,
    0b10000_000,
    0b00000_000,
    0b00000_000,
], 5);

pub const BODY_FONT_0X3F: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b000000_00,
    0b011100_00,
    0b100010_00,
    0b000010_00,
    0b000100_00,
    0b001000_00,
    0b000000_00,
    0b001000_00,
    0b000000_00,
    0b000000_00,
], 6);

pub const BODY_FONT_0X40: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b00111100,
    0b01000010,
    0b10011001,
    0b10000101,
    0b10011101,
    0b10100101,
    0b10011110,
    0b01000000,
    0b00111100,
    0b00000000,
], 8);

pub const BODY_FONT_0X41: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b000000_00,
    0b011100_00,
    0b100010_00,
    0b100010_00,
    0b111110_00,
    0b100010_00,
    0b100010_00,
    0b100010_00,
    0b000000_00,
    0b000000_00,
], 6);

pub const BODY_FONT_0X42: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b000000_00,
    0b111100_00,
    0b100010_00,
    0b100010_00,
    0b111100_00,
    0b100010_00,
    0b100010_00,
    0b111100_00,
    0b000000_00,
    0b000000_00,
], 6);

pub const BODY_FONT_0X43: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b000000_00,
    0b011100_00,
    0b100010_00,
    0b100000_00,
    0b100000_00,
    0b100000_00,
    0b100010_00,
    0b011100_00,
    0b000000_00,
    0b000000_00,
], 6);

pub const BODY_FONT_0X44: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b000000_00,
    0b111100_00,
    0b100010_00,
    0b100010_00,
    0b100010_00,
    0b100010_00,
    0b100010_00,
    0b111100_00,
    0b000000_00,
    0b000000_00,
], 6);

pub const BODY_FONT_0X45: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b000000_00,
    0b111110_00,
    0b100000_00,
    0b100000_00,
    0b111100_00,
    0b100000_00,
    0b100000_00,
    0b111110_00,
    0b000000_00,
    0b000000_00,
], 6);

pub const BODY_FONT_0X46: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b000000_00,
    0b111110_00,
    0b100000_00,
    0b100000_00,
    0b111100_00,
    0b100000_00,
    0b100000_00,
    0b100000_00,
    0b000000_00,
    0b000000_00,
], 6);

pub const BODY_FONT_0X47: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b000000_00,
    0b011100_00,
    0b100010_00,
    0b100000_00,
    0b101110_00,
    0b100010_00,
    0b100010_00,
    0b011110_00,
    0b000000_00,
    0b000000_00,
], 6);

pub const BODY_FONT_0X48: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b000000_00,
    0b100010_00,
    0b100010_00,
    0b100010_00,
    0b111110_00,
    0b100010_00,
    0b100010_00,
    0b100010_00,
    0b000000_00,
    0b000000_00,
], 6);

pub const BODY_FONT_0X49: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b0000_0000,
    0b1110_0000,
    0b0100_0000,
    0b0100_0000,
    0b0100_0000,
    0b0100_0000,
    0b0100_0000,
    0b1110_0000,
    0b0000_0000,
    0b0000_0000,
], 4);

pub const BODY_FONT_0X4A: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b000000_00,
    0b000010_00,
    0b000010_00,
    0b000010_00,
    0b000010_00,
    0b000010_00,
    0b100010_00,
    0b011100_00,
    0b000000_00,
    0b000000_00,
], 6);

pub const BODY_FONT_0X4B: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b000000_00,
    0b100010_00,
    0b100100_00,
    0b101000_00,
    0b110000_00,
    0b101000_00,
    0b100100_00,
    0b100010_00,
    0b000000_00,
    0b000000_00,
], 6);

pub const BODY_FONT_0X4C: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b000000_00,
    0b100000_00,
    0b100000_00,
    0b100000_00,
    0b100000_00,
    0b100000_00,
    0b100000_00,
    0b111110_00,
    0b000000_00,
    0b000000_00,
], 6);

pub const BODY_FONT_0X4D: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b00000000,
    0b10000010,
    0b11000110,
    0b10101010,
    0b10010010,
    0b10000010,
    0b10000010,
    0b10000010,
    0b00000000,
    0b00000000,
], 8);

pub const BODY_FONT_0X4E: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b000000_00,
    0b100010_00,
    0b100010_00,
    0b110010_00,
    0b101010_00,
    0b100110_00,
    0b100010_00,
    0b100010_00,
    0b000000_00,
    0b000000_00,
], 6);

pub const BODY_FONT_0X4F: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b000000_00,
    0b011100_00,
    0b100010_00,
    0b100010_00,
    0b100010_00,
    0b100010_00,
    0b100010_00,
    0b011100_00,
    0b000000_00,
    0b000000_00,
], 6);

pub const BODY_FONT_0X50: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b000000_00,
    0b111100_00,
    0b100010_00,
    0b100010_00,
    0b100010_00,
    0b111100_00,
    0b100000_00,
    0b100000_00,
    0b000000_00,
    0b000000_00,
], 6);

pub const BODY_FONT_0X51: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b000000_00,
    0b011100_00,
    0b100010_00,
    0b100010_00,
    0b100010_00,
    0b100010_00,
    0b100010_00,
    0b011100_00,
    0b000010_00,
    0b000000_00,
], 6);

pub const BODY_FONT_0X52: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b000000_00,
    0b111100_00,
    0b100010_00,
    0b100010_00,
    0b100010_00,
    0b111100_00,
    0b100100_00,
    0b100010_00,
    0b000000_00,
    0b000000_00,
], 6);

pub const BODY_FONT_0X53: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b000000_00,
    0b011100_00,
    0b100010_00,
    0b100000_00,
    0b011100_00,
    0b000010_00,
    0b100010_00,
    0b011100_00,
    0b000000_00,
    0b000000_00,
], 6);

pub const BODY_FONT_0X54: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b000000_00,
    0b111110_00,
    0b001000_00,
    0b001000_00,
    0b001000_00,
    0b001000_00,
    0b001000_00,
    0b001000_00,
    0b000000_00,
    0b000000_00,
], 6);

pub const BODY_FONT_0X55: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b000000_00,
    0b100010_00,
    0b100010_00,
    0b100010_00,
    0b100010_00,
    0b100010_00,
    0b100010_00,
    0b011100_00,
    0b000000_00,
    0b000000_00,
], 6);

pub const BODY_FONT_0X56: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b000000_00,
    0b100010_00,
    0b100010_00,
    0b100010_00,
    0b100010_00,
    0b010100_00,
    0b010100_00,
    0b001000_00,
    0b000000_00,
    0b000000_00,
], 6);

pub const BODY_FONT_0X57: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b00000000,
    0b10010010,
    0b10010010,
    0b10010010,
    0b10010010,
    0b10010010,
    0b10010010,
    0b01101100,
    0b00000000,
    0b00000000,
], 8);

pub const BODY_FONT_0X58: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b000000_00,
    0b100010_00,
    0b100010_00,
    0b010100_00,
    0b001000_00,
    0b010100_00,
    0b100010_00,
    0b100010_00,
    0b000000_00,
    0b000000_00,
], 6);

pub const BODY_FONT_0X59: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b000000_00,
    0b100010_00,
    0b100010_00,
    0b100010_00,
    0b010100_00,
    0b001000_00,
    0b001000_00,
    0b001000_00,
    0b000000_00,
    0b000000_00,
], 6);

pub const BODY_FONT_0X5A: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b000000_00,
    0b111110_00,
    0b000010_00,
    0b000100_00,
    0b001000_00,
    0b010000_00,
    0b100000_00,
    0b111110_00,
    0b000000_00,
    0b000000_00,
], 6);

pub const BODY_FONT_0X5B: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b0000_0000,
    0b1110_0000,
    0b1000_0000,
    0b1000_0000,
    0b1000_0000,
    0b1000_0000,
    0b1000_0000,
    0b1110_0000,
    0b0000_0000,
    0b0000_0000,
], 4);

pub const BODY_FONT_0X5C: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b00000000,
    0b10000000,
    0b01000000,
    0b00100000,
    0b00010000,
    0b00001000,
    0b00000100,
    0b00000010,
    0b00000000,
    0b00000000,
], 8);

pub const BODY_FONT_0X5D: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b0000_0000,
    0b1110_0000,
    0b0010_0000,
    0b0010_0000,
    0b0010_0000,
    0b0010_0000,
    0b0010_0000,
    0b1110_0000,
    0b0000_0000,
    0b0000_0000,
], 4);

pub const BODY_FONT_0X5E: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b0000_0000,
    0b0100_0000,
    0b1010_0000,
    0b0000_0000,
    0b0000_0000,
    0b0000_0000,
    0b0000_0000,
    0b0000_0000,
    0b0000_0000,
    0b0000_0000,
], 4);

pub const BODY_FONT_0X5F: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b000000_00,
    0b000000_00,
    0b000000_00,
    0b000000_00,
    0b000000_00,
    0b000000_00,
    0b000000_00,
    0b111110_00,
    0b000000_00,
    0b000000_00,
], 6);

pub const BODY_FONT_0X60: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b000_00000,
    0b100_00000,
    0b010_00000,
    0b000_00000,
    0b000_00000,
    0b000_00000,
    0b000_00000,
    0b000_00000,
    0b000_00000,
    0b000_00000,
], 3);

pub const BODY_FONT_0X61: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b000000_00,
    0b000000_00,
    0b000000_00,
    0b011100_00,
    0b000010_00,
    0b011110_00,
    0b100010_00,
    0b011110_00,
    0b000000_00,
    0b000000_00,
], 6);

pub const BODY_FONT_0X62: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b000000_00,
    0b100000_00,
    0b100000_00,
    0b111100_00,
    0b100010_00,
    0b100010_00,
    0b100010_00,
    0b111100_00,
    0b000000_00,
    0b000000_00,
], 6);

pub const BODY_FONT_0X63: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b000000_00,
    0b000000_00,
    0b000000_00,
    0b011110_00,
    0b100000_00,
    0b100000_00,
    0b100000_00,
    0b011110_00,
    0b000000_00,
    0b000000_00,
], 6);

pub const BODY_FONT_0X64: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b000000_00,
    0b000010_00,
    0b000010_00,
    0b011110_00,
    0b100010_00,
    0b100010_00,
    0b100010_00,
    0b011110_00,
    0b000000_00,
    0b000000_00,
], 6);

pub const BODY_FONT_0X65: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b000000_00,
    0b000000_00,
    0b000000_00,
    0b011100_00,
    0b100010_00,
    0b111110_00,
    0b100000_00,
    0b011110_00,
    0b000000_00,
    0b000000_00,
], 6);

pub const BODY_FONT_0X66: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b00000_000,
    0b00110_000,
    0b01000_000,
    0b11110_000,
    0b01000_000,
    0b01000_000,
    0b01000_000,
    0b01000_000,
    0b00000_000,
    0b00000_000,
], 5);

pub const BODY_FONT_0X67: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b000000_00,
    0b000000_00,
    0b000000_00,
    0b011110_00,
    0b100010_00,
    0b100010_00,
    0b100010_00,
    0b011110_00,
    0b000010_00,
    0b011100_00,
], 6);

pub const BODY_FONT_0X68: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b000000_00,
    0b100000_00,
    0b100000_00,
    0b111100_00,
    0b100010_00,
    0b100010_00,
    0b100010_00,
    0b100010_00,
    0b000000_00,
    0b000000_00,
], 6);

pub const BODY_FONT_0X69: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b00_000000,
    0b10_000000,
    0b00_000000,
    0b10_000000,
    0b10_000000,
    0b10_000000,
    0b10_000000,
    0b10_000000,
    0b00_000000,
    0b00_000000,
], 2);

pub const BODY_FONT_0X6A: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b000_00000,
    0b010_00000,
    0b000_00000,
    0b010_00000,
    0b010_00000,
    0b010_00000,
    0b010_00000,
    0b010_00000,
    0b010_00000,
    0b100_00000,
], 3);

pub const BODY_FONT_0X6B: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b00000_000,
    0b10000_000,
    0b10000_000,
    0b10010_000,
    0b10100_000,
    0b11000_000,
    0b10100_000,
    0b10010_000,
    0b00000_000,
    0b00000_000,
], 5);

pub const BODY_FONT_0X6C: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b00_000000,
    0b10_000000,
    0b10_000000,
    0b10_000000,
    0b10_000000,
    0b10_000000,
    0b10_000000,
    0b10_000000,
    0b00_000000,
    0b00_000000,
], 2);

pub const BODY_FONT_0X6D: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b00000000,
    0b00000000,
    0b00000000,
    0b11111100,
    0b10010010,
    0b10010010,
    0b10010010,
    0b10010010,
    0b00000000,
    0b00000000,
], 8);

pub const BODY_FONT_0X6E: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b000000_00,
    0b000000_00,
    0b000000_00,
    0b111100_00,
    0b100010_00,
    0b100010_00,
    0b100010_00,
    0b100010_00,
    0b000000_00,
    0b000000_00,
], 6);

pub const BODY_FONT_0X6F: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b000000_00,
    0b000000_00,
    0b000000_00,
    0b011100_00,
    0b100010_00,
    0b100010_00,
    0b100010_00,
    0b011100_00,
    0b000000_00,
    0b000000_00,
], 6);

pub const BODY_FONT_0X70: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b000000_00,
    0b000000_00,
    0b000000_00,
    0b111100_00,
    0b100010_00,
    0b100010_00,
    0b100010_00,
    0b111100_00,
    0b100000_00,
    0b100000_00,
], 6);

pub const BODY_FONT_0X71: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b000000_00,
    0b000000_00,
    0b000000_00,
    0b011110_00,
    0b100010_00,
    0b100010_00,
    0b100010_00,
    0b011110_00,
    0b000010_00,
    0b000010_00,
], 6);

pub const BODY_FONT_0X72: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b00000_000,
    0b00000_000,
    0b00000_000,
    0b10110_000,
    0b11000_000,
    0b10000_000,
    0b10000_000,
    0b10000_000,
    0b00000_000,
    0b00000_000,
], 5);

pub const BODY_FONT_0X73: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b000000_00,
    0b000000_00,
    0b000000_00,
    0b011110_00,
    0b100000_00,
    0b011100_00,
    0b000010_00,
    0b111100_00,
    0b000000_00,
    0b000000_00,
], 6);

pub const BODY_FONT_0X74: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b00000_000,
    0b01000_000,
    0b01000_000,
    0b11110_000,
    0b01000_000,
    0b01000_000,
    0b01000_000,
    0b00110_000,
    0b00000_000,
    0b00000_000,
], 5);

pub const BODY_FONT_0X75: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b000000_00,
    0b000000_00,
    0b000000_00,
    0b100010_00,
    0b100010_00,
    0b100010_00,
    0b100010_00,
    0b011110_00,
    0b000000_00,
    0b000000_00,
], 6);

pub const BODY_FONT_0X76: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b000000_00,
    0b000000_00,
    0b000000_00,
    0b100010_00,
    0b100010_00,
    0b010100_00,
    0b010100_00,
    0b001000_00,
    0b000000_00,
    0b000000_00,
], 6);

pub const BODY_FONT_0X77: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b00000000,
    0b00000000,
    0b00000000,
    0b10010010,
    0b10010010,
    0b10010010,
    0b10010010,
    0b01101100,
    0b00000000,
    0b00000000,
], 8);

pub const BODY_FONT_0X78: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b000000_00,
    0b000000_00,
    0b000000_00,
    0b100010_00,
    0b010100_00,
    0b001000_00,
    0b010100_00,
    0b100010_00,
    0b000000_00,
    0b000000_00,
], 6);

pub const BODY_FONT_0X79: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b000000_00,
    0b000000_00,
    0b000000_00,
    0b100010_00,
    0b100010_00,
    0b100010_00,
    0b100010_00,
    0b011110_00,
    0b000010_00,
    0b011100_00,
], 6);

pub const BODY_FONT_0X7A: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b000000_00,
    0b000000_00,
    0b000000_00,
    0b111110_00,
    0b000100_00,
    0b001000_00,
    0b010000_00,
    0b111110_00,
    0b000000_00,
    0b000000_00,
], 6);

pub const BODY_FONT_0X7B: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b00000_000,
    0b00110_000,
    0b01000_000,
    0b01000_000,
    0b10000_000,
    0b01000_000,
    0b01000_000,
    0b00110_000,
    0b00000_000,
    0b00000_000,
], 5);

pub const BODY_FONT_0X7C: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b00_000000,
    0b10_000000,
    0b10_000000,
    0b10_000000,
    0b10_000000,
    0b10_000000,
    0b10_000000,
    0b10_000000,
    0b00_000000,
    0b00_000000,
], 2);

pub const BODY_FONT_0X7D: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b00000_000,
    0b11000_000,
    0b00100_000,
    0b00100_000,
    0b00010_000,
    0b00100_000,
    0b00100_000,
    0b11000_000,
    0b00000_000,
    0b00000_000,
], 5);

pub const BODY_FONT_0X7E: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[
    0b0000000_0,
    0b0000001_0,
    0b0000001_0,
    0b0000010_0,
    0b0000010_0,
    0b0100100_0,
    0b0010100_0,
    0b0001000_0,
    0b0000000_0,
    0b0000000_0,
], 7);
