use coal_guilds_api::{error::GuildError, state::{Guild, Member}, consts::LEAVE_DELAY};
use solana_program;
use steel::*;

/// New creates a new guild
pub fn process_leave(accounts: &[AccountInfo<'_>], _data: &[u8]) -> ProgramResult {
    // Load accounts.
    let [signer_info, guild_info, member_info] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    signer_info.is_signer()?;
    let guild = guild_info.is_writable()?.to_account_mut::<Guild>(&coal_guilds_api::ID)?;
    let member = member_info
        .is_writable()?
        .to_account_mut::<Member>(&coal_guilds_api::ID)?
        .check_mut(|m| m.authority.eq(&signer_info.key))?
        .check_mut(|m| m.guild.eq(&guild_info.key))?;

    if member.last_join_at.checked_add(LEAVE_DELAY).unwrap() > Clock::get()?.unix_timestamp {
        return Err(GuildError::TooEarly.into());
    }
    
    member.guild = solana_program::system_program::id();
    guild.total_stake = guild.total_stake.checked_sub(member.total_stake).unwrap();

    Ok(())
}