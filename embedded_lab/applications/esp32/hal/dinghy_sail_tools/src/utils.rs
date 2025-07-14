// pub struct BufferWriter<'a> {
//     pub buffer: &'a mut [u8],
//     pub offset: usize,
// }

// impl<'a> BufferWriter<'a> {
//     pub fn new(buffer: &'a mut [u8]) -> Self {
//         BufferWriter { buffer, offset: 0 }
//     }

//     pub fn as_str(&self) -> &str {
//         core::str::from_utf8(&self.buffer[..self.offset]).unwrap()
//     }
// }
