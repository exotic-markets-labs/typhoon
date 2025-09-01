/// Cold function used for branch prediction hints in stable Rust.
///
/// This function is marked as `#[cold]` to hint to the compiler that any
/// branch containing a call to this function is unlikely to be taken.
#[inline(always)]
#[cold]
pub fn cold() {}

/// Branch prediction hint for unlikely conditions.
///
/// Uses the stable `#[cold]` function approach to provide branch prediction hints.
/// This works by calling a cold function in the unlikely branch, which signals
/// to LLVM that this branch should be optimized for the uncommon case.
#[inline(always)]
pub fn unlikely(b: bool) -> bool {
    if b {
        cold();
    }
    b
}
