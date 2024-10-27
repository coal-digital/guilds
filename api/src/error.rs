use steel::*;

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq, IntoPrimitive)]
#[repr(u32)]
pub enum GuildError {
    #[error("Too early to unstake")]
    TooEarly = 0,
    #[error("Invalid guild")]
    InvalidGuild = 1,
}

error!(GuildError);