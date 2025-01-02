use std::io::Write;

pub const UNINIT_BYTE: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::<u8>::uninit();

#[inline(always)]
pub fn write_bytes(destination: &mut [std::mem::MaybeUninit<u8>], source: &[u8]) {
    for (d, s) in destination.iter_mut().zip(source.iter()) {
        d.write(*s);
    }
}

pub struct MaybeUninitWriter<'a> {
    buffer: &'a mut [std::mem::MaybeUninit<u8>],
    position: usize,
}

impl<'a> MaybeUninitWriter<'a> {
    pub fn new(buffer: &'a mut [std::mem::MaybeUninit<u8>], position: usize) -> Self {
        Self { buffer, position }
    }

    pub fn initialized(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.buffer.as_ptr() as *const u8, self.position) }
    }
}

impl Write for MaybeUninitWriter<'_> {
    fn write(&mut self, data: &[u8]) -> std::io::Result<usize> {
        let available = self.buffer.len().saturating_sub(self.position);
        let to_write = data.len().min(available);

        if to_write == 0 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::WriteZero,
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

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

// TODO unit test
