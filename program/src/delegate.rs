use coal_guilds_api::{
    consts::MEMBER,
    instruction::Delegate,
    state::{Guild, Member},
};
use solana_program::system_program;
use steel::*;

/// New creates a new guild
pub fn process_delegate(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
    // Parse args.
    let args = Delegate::try_from_bytes(data)?;

    // Load accounts.
    let [signer_info, guild_info, member_info, system_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    signer_info.is_signer()?;
    let guild = guild_info
        .to_account_mut::<Guild>(&coal_guilds_api::ID)?;
    let member = member_info.is_writable()?.has_seeds(
        &[MEMBER, signer_info.key.as_ref()],
        args.member_bump,
        &coal_guilds_api::id(),
    )?.to_account_mut::<Member>(&coal_guilds_api::ID)?;
    system_program.is_program(&system_program::ID)?;

    // Check if the member is already in a guild.
    if member.guild.ne(&system_program::ID) {
        return Err(ProgramError::InvalidAccountData);
    }

    member.guild = *guild_info.key;
    member.is_active = 0;
    member.last_join_at = Clock::get()?.unix_timestamp;
    guild.total_stake = guild.total_stake.checked_add(member.total_stake).unwrap();
    guild.last_stake_at = Clock::get()?.unix_timestamp;

    Ok(())
}