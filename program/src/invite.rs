use coal_guilds_api::{
    consts::{GUILD, INVITE},
    instruction::NewInvite,
    state::{Guild, Invite, Member},
};
use solana_program::system_program;
use steel::*;

/// New creates a new guild
pub fn process_new_invite(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
    // Parse args.
    let args = NewInvite::try_from_bytes(data)?;

    // Load accounts.
    let [signer_info, guild_info, invite_info, member_info, system_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    signer_info.is_signer()?;
    invite_info.is_writable()?.is_empty()?.has_seeds(
        &[INVITE, guild_info.key.as_ref(), member_info.key.as_ref()],
        args.invite_bump,
        &coal_guilds_api::id(),
    )?;
    guild_info.has_seeds(
        &[GUILD, signer_info.key.as_ref()],
        args.guild_bump,
        &coal_guilds_api::id(),
    )?.to_account::<Guild>(&coal_guilds_api::ID)?;
    member_info
        .to_account::<Member>(&coal_guilds_api::ID)?
        .check(|m| m.guild.eq(&solana_program::system_program::id()))?;
    system_program.is_program(&system_program::ID)?;

    // Initialize the invite account.
    create_account::<Invite>(
        invite_info,
        &coal_guilds_api::id(),
        &[INVITE, guild_info.key.as_ref(), member_info.key.as_ref(), &[args.invite_bump]],
        system_program,
        signer_info,
    )?;

    let invite = invite_info.to_account_mut::<Invite>(&coal_guilds_api::ID)?;
    invite.bump = args.invite_bump as u64;
    invite.guild = *guild_info.key;
    invite.member = *member_info.key;
    let clock = Clock::get()?;
    invite.created_at = clock.unix_timestamp;

    Ok(())
}