use coal_guilds_api::{
    consts::{INVITE, MEMBER},
    instruction::Join,
    state::{Guild, Invite, Member},
};
use solana_program::system_program;
use steel::*;

/// New creates a new guild
pub fn process_join(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
    // Parse args.
    let args = Join::try_from_bytes(data)?;

    // Load accounts.
    let [signer_info, guild_info, guild_authority, invite_info, member_info, system_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    signer_info.is_signer()?;
    let guild = guild_info
        .to_account_mut::<Guild>(&coal_guilds_api::ID)?
        .check_mut(|g| g.authority.eq(&guild_authority.key))?;
    let member = member_info.is_writable()?.has_seeds(
        &[MEMBER, signer_info.key.as_ref()],
        args.member_bump,
        &coal_guilds_api::id(),
    )?.to_account_mut::<Member>(&coal_guilds_api::ID)?;
    invite_info.is_writable()?.has_seeds(
        &[INVITE, guild_info.key.as_ref(), member_info.key.as_ref()],
        args.invite_bump,
        &coal_guilds_api::id(),
    )?.to_account::<Invite>(&coal_guilds_api::ID)?;
    system_program.is_program(&system_program::ID)?;

    // Check if the member is already in a guild.
    if member.guild.ne(&system_program::ID) {
        return Err(ProgramError::InvalidAccountData);
    }

    member.guild = *guild_info.key;
    member.is_active = 1;
    member.last_join_at = Clock::get()?.unix_timestamp;
    guild.total_stake = guild.total_stake.checked_add(member.total_stake).unwrap();
    guild.last_stake_at = Clock::get()?.unix_timestamp;

    // Realloc data to zero.
    invite_info.realloc(0, true)?;

    // Send remaining lamports to signer.
    **guild_authority.lamports.borrow_mut() += invite_info.lamports();
    **invite_info.lamports.borrow_mut() = 0;

    Ok(())
}