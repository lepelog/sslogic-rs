use base64::{decode, encode};
use base64::DecodeError;

pub struct PackedBitsReader {
    bytes: Vec<u8>,
    current_byte_index: usize,
    current_bit_index: u8,
}

impl PackedBitsReader {
    pub fn from_base64(encoded: &str) -> Result<Self, DecodeError> {
        let decoded = decode(encoded)?;
        let first_byte = decoded[0];
        return Ok(PackedBitsReader {
            bytes: decoded,
            current_byte_index: 0,
            current_bit_index: 0,
        });
    }

    pub fn read(&mut self, length: u8) -> Result<usize, &'static str> {
        let mut bits_read = 0;
        let mut value: usize = 0;
        let mut bits_left_to_read = length;

        while bits_read != length {
            let mut bits_to_read = bits_left_to_read.min(8);
            if bits_to_read + self.current_bit_index > 8 {
                bits_to_read = 8 - self.current_bit_index;
            }

            let mask = ((1 << bits_to_read) - 1) << self.current_bit_index;
            let current_byte = *self.bytes.get(self.current_byte_index).ok_or_else(|| "no more bytes left!")?;
            value = (((current_byte & mask) >> self.current_bit_index) as usize) << bits_read | value;

            self.current_bit_index += bits_to_read;
            self.current_byte_index += (self.current_bit_index >> 3) as usize;
            self.current_bit_index %= 8;
            bits_left_to_read -= bits_to_read;
            bits_read += bits_to_read;
        }
        Ok(value)
    }
}

pub struct PackedBitsWriter {
    bytes: Vec<u8>,
    current_byte: u8,
    bits_left_in_byte: u8,
}

impl PackedBitsWriter {

    pub fn new() -> Self {
        PackedBitsWriter {
            bytes: Vec::new(),
            current_byte: 0,
            bits_left_in_byte: 8,
        }
    }

    pub fn write(&mut self, mut value: usize, mut length: u8) {
        while length != 0 {
            let bits_to_read = length.min(self.bits_left_in_byte);
            let mask = (1 << bits_to_read) - 1;
            self.current_byte |= ((value & mask) as u8) << (8 - self.bits_left_in_byte);

            self.bits_left_in_byte -= bits_to_read;
            length -= bits_to_read;
            value >>= bits_to_read;

            if self.bits_left_in_byte != 0 {
                continue;
            }

            self.flush();
        }
    }

    pub fn flush(&mut self) {
        self.bytes.push(self.current_byte);
        self.current_byte = 0;
        self.bits_left_in_byte = 8;
    }

    pub fn to_base64(&self) -> String{
        encode(&self.bytes)
    }
}