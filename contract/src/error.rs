use cosmwasm_std::{OverflowError, StdError};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Wrong token")]
    IncorrectToken {},

    #[error("When not using CW20 as an input token to be burned a native denom for the native token must be provided, this is so it can be checked when a burn is done")]
    MissingNativeDenom {},

    #[error("Wrong number of tokens")]
    IncorrectTokenQuantity {},

    #[error("{0}")]
    OverflowError(#[from] OverflowError),

    #[error("Semver parsing error: {0}")]
    SemVer(String),
}

impl From<semver::Error> for ContractError {
    fn from(err: semver::Error) -> Self {
        Self::SemVer(err.to_string())
    }
}
