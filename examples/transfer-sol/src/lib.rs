use bytemuck::{Pod, Zeroable};
use typhoon::prelude::*;

program_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

handlers! {
    transfer_sol_with_cpi,
    transfer_sol_with_program
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Pod, Zeroable)]
#[repr(transparent)]
pub struct PodU64(pub [u8; 8]);

impl PodU64 {
    pub const fn from_primitive(n: u64) -> Self {
        Self(n.to_le_bytes())
    }
}
impl From<u64> for PodU64 {
    fn from(n: u64) -> Self {
        Self::from_primitive(n)
    }
}
impl From<PodU64> for u64 {
    fn from(pod: PodU64) -> Self {
        Self::from_le_bytes(pod.0)
    }
}

#[context]
pub struct TransferContext {
    pub payer: Mut<Signer>,
    pub recipient: Mut<SystemAccount>,
}

#[context]
pub struct SystemContext {
    pub system: Program<System>,
}

pub fn transfer_sol_with_cpi(
    amount: Args<PodU64>,
    ctx: TransferContext,
    _: SystemContext,
) -> Result<(), ProgramError> {
    ctx.payer.transfer(&ctx.recipient, (*amount).into())?;

    Ok(())
}

pub fn transfer_sol_with_program(
    amount: Args<PodU64>,
    ctx: TransferContext,
) -> Result<(), ProgramError> {
    ctx.payer.send(&ctx.recipient, (*amount).into())?;

    Ok(())
}
