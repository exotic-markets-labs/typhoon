use {solana_address::Address, typhoon_traits::CheckProgramId};

pub struct System;

impl CheckProgramId for System {
    #[inline(always)]
    fn address_eq(program_id: &Address) -> bool {
        let p = program_id.as_array().as_ptr().cast::<u64>();

        unsafe {
            core::ptr::read_unaligned(p)
                | core::ptr::read_unaligned(p.add(1))
                | core::ptr::read_unaligned(p.add(2))
                | core::ptr::read_unaligned(p.add(3))
                == 0
        }
    }
}
