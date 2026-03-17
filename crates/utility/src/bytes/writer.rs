use {
    core::{mem::MaybeUninit, ptr},
    typhoon_errors::ErrorCode,
};

pub struct MaybeUninitWriter<'a> {
    buffer: &'a mut [MaybeUninit<u8>],
    position: usize,
}

impl<'a> MaybeUninitWriter<'a> {
    #[inline(always)]
    pub fn new(buffer: &'a mut [MaybeUninit<u8>], position: usize) -> Self {
        Self { buffer, position }
    }

    #[inline(always)]
    pub fn initialized(&self) -> &[u8] {
        unsafe { core::slice::from_raw_parts(self.buffer.as_ptr() as *const u8, self.position) }
    }

    #[inline(always)]
    pub fn write_bytes(&mut self, data: &[u8]) -> Result<usize, ErrorCode> {
        if data.is_empty() {
            return Ok(0);
        }

        if self.position >= self.buffer.len() {
            return Err(buffer_full());
        }

        let to_write = data.len().min(self.buffer.len() - self.position);

        if to_write == 0 {
            return Err(buffer_full());
        }

        // SAFETY: We're writing to `MaybeUninit` and ensuring the data is valid.
        unsafe {
            let dst_ptr = self.buffer.as_mut_ptr().add(self.position);
            ptr::copy_nonoverlapping(data.as_ptr(), dst_ptr as *mut u8, to_write);
        }

        self.position += to_write;

        Ok(to_write)
    }
}

#[cold]
#[inline(never)]
fn buffer_full() -> ErrorCode {
    ErrorCode::BufferFull
}
