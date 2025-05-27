pub struct MaybeUninitWriter<'a> {
    buffer: &'a mut [core::mem::MaybeUninit<u8>],
    position: usize,
}

impl<'a> MaybeUninitWriter<'a> {
    pub fn new(buffer: &'a mut [core::mem::MaybeUninit<u8>], position: usize) -> Self {
        Self { buffer, position }
    }

    pub fn initialized(&self) -> &[u8] {
        unsafe { core::slice::from_raw_parts(self.buffer.as_ptr() as *const u8, self.position) }
    }
}

impl borsh::io::Write for MaybeUninitWriter<'_> {
    fn write(&mut self, data: &[u8]) -> borsh::io::Result<usize> {
        let available = self.buffer.len().saturating_sub(self.position);
        let to_write = data.len().min(available);

        if to_write == 0 {
            return Err(borsh::io::Error::new(
                borsh::io::ErrorKind::WriteZero,
                "Buffer full",
            ));
        }

        // SAFETY: We're writing to `MaybeUninit` and ensuring the data is valid.
        unsafe {
            let dst = self
                .buffer
                .get_unchecked_mut(self.position..self.position + to_write);
            for (uninit_byte, &src_byte) in dst.iter_mut().zip(data) {
                uninit_byte.write(src_byte);
            }
        }

        self.position += to_write;
        Ok(to_write)
    }

    fn flush(&mut self) -> borsh::io::Result<()> {
        Ok(())
    }
}
