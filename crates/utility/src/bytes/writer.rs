use typhoon_errors::ErrorCode;

pub struct MaybeUninitWriter<'a> {
    buffer: &'a mut [core::mem::MaybeUninit<u8>],
    position: usize,
}

impl<'a> MaybeUninitWriter<'a> {
    #[inline(always)]
    pub fn new(buffer: &'a mut [core::mem::MaybeUninit<u8>], position: usize) -> Self {
        Self { buffer, position }
    }

    #[inline(always)]
    pub fn initialized(&self) -> &[u8] {
        unsafe { core::slice::from_raw_parts(self.buffer.as_ptr() as *const u8, self.position) }
    }

    #[inline(always)]
    pub fn write_bytes(&mut self, data: &[u8]) -> Result<usize, ErrorCode> {
        let available = self.buffer.len().saturating_sub(self.position);
        let to_write = data.len().min(available);

        if to_write == 0 {
            return Err(ErrorCode::BufferFull);
        }

        // SAFETY: We're writing to `MaybeUninit` and ensuring the data is valid.
        unsafe {
            let dst_ptr = self.buffer.as_mut_ptr().add(self.position);
            core::ptr::copy_nonoverlapping(data.as_ptr(), dst_ptr as _, to_write);
        }

        self.position += to_write;

        Ok(to_write)
    }
}

#[cfg(feature = "borsh")]
impl borsh::io::Write for MaybeUninitWriter<'_> {
    fn write(&mut self, data: &[u8]) -> borsh::io::Result<usize> {
        self.write_bytes(data)
            .map_err(|_| borsh::io::Error::new(borsh::io::ErrorKind::WriteZero, "Buffer full"))
    }

    fn flush(&mut self) -> borsh::io::Result<()> {
        Ok(())
    }
}
