use {
    crate::{FromAccountInfo, ProgramIds, ReadableAccount},
    core::marker::PhantomData,
    pinocchio::{
        account_info::{AccountInfo, Ref},
        program_error::ProgramError,
    },
    typhoon_errors::{Error, ErrorCode},
};

pub struct Interface<'a, T> {
    info: &'a AccountInfo,
    _phantom: PhantomData<T>,
}

impl<'a, T> FromAccountInfo<'a> for Interface<'a, T>
where
    T: ProgramIds,
{
    #[inline(always)]
    fn try_from_info(info: &'a AccountInfo) -> Result<Self, Error> {
        if !T::IDS.contains(info.key()) {
            return Err(ProgramError::IncorrectProgramId.into());
        }

        if !info.executable() {
            return Err(ErrorCode::AccountOwnedByWrongProgram.into());
        }

        Ok(Interface {
            info,
            _phantom: PhantomData,
        })
    }
}

impl<'a, T> From<Interface<'a, T>> for &'a AccountInfo {
    #[inline(always)]
    fn from(value: Interface<'a, T>) -> Self {
        value.info
    }
}

impl<T> AsRef<AccountInfo> for Interface<'_, T> {
    #[inline(always)]
    fn as_ref(&self) -> &AccountInfo {
        self.info
    }
}

impl<T> ReadableAccount for Interface<'_, T> {
    type Data<'a>
        = Ref<'a, [u8]>
    where
        Self: 'a;

    #[inline(always)]
    fn data<'a>(&'a self) -> Result<Self::Data<'a>, Error> {
        self.info.try_borrow_data().map_err(Into::into)
    }
}
