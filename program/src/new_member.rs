use coal_guilds_api::{
    consts::MEMBER,
    instruction::NewMember,
    state::Member,
};
use solana_program::system_program;
use steel::*;

/// New creates a new guild
pub fn process_new_member(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
    // Parse args.
    let args = NewMember::try_from_bytes(data)?;

    // Load accounts.
    let [signer_info, member_info, system_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    signer_info.is_signer()?;
    member_info.is_writable()?.is_empty()?.has_seeds(
        &[MEMBER, signer_info.key.as_ref()],
        args.member_bump,
        &coal_guilds_api::id(),
    )?;
    system_program.is_program(&system_program::ID)?;

    // Initialize the member account.
    create_account::<Member>(
        member_info,
        &coal_guilds_api::id(),
        &[MEMBER, signer_info.key.as_ref(), &[args.member_bump]],
        system_program,
        signer_info,
    )?;
    let member = member_info.to_account_mut::<Member>(&coal_guilds_api::ID)?;
    member.bump = args.member_bump as u64;
    member.authority = *signer_info.key;
    member.guild = *system_program.key;
    member.is_active = 1;
    member.last_stake_at = 0;
    member.total_stake = 0;

    Ok(())
}