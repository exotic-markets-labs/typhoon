use {
    crate::{FromAccountInfo, ProgramId, ReadableAccount},
    core::marker::PhantomData,
    pinocchio::{
        account_info::{AccountInfo, Ref},
        pubkey::Pubkey,
    },
    typhoon_errors::{Error, ErrorCode},
};

///
/// Checks:
/// * `account_info.key == expected_program`
/// * `account_info.executable == true`
pub struct Program<'a, T> {
    info: &'a AccountInfo,
    _phantom: PhantomData<T>,
}

impl<'a, T> FromAccountInfo<'a> for Program<'a, T>
where
    T: ProgramId,
{
    fn try_from_info(info: &'a AccountInfo) -> Result<Self, Error> {
        if info.key() != &T::ID {
            return Err(ErrorCode::AccountOwnedByWrongProgram.into());
        }

        if !info.executable() {
            return Err(ErrorCode::AccountOwnedByWrongProgram.into());
        }

        Ok(Program {
            info,
            _phantom: PhantomData,
        })
    }
}

impl<'a, T> From<Program<'a, T>> for &'a AccountInfo {
    fn from(value: Program<'a, T>) -> Self {
        value.info
    }
}

impl<T> AsRef<AccountInfo> for Program<'_, T> {
    fn as_ref(&self) -> &AccountInfo {
        self.info
    }
}

impl<T> ReadableAccount for Program<'_, T> {
    type Data<'a>
        = Ref<'a, [u8]>
    where
        Self: 'a;

    fn key(&self) -> &Pubkey {
        self.info.key()
    }

    fn is_owned_by(&self, owner: &Pubkey) -> bool {
        self.info.is_owned_by(owner)
    }

    fn lamports(&self) -> Result<Ref<u64>, Error> {
        self.info.try_borrow_lamports().map_err(Into::into)
    }

    fn data<'a>(&'a self) -> Result<Self::Data<'a>, Error> {
        self.info.try_borrow_data().map_err(Into::into)
    }
}
