use coal_guilds_api::{
    consts::GUILD,
    instruction::NewGuild,
    state::Guild,
};
use solana_program::system_program;
use steel::*;

/// New creates a new guild
pub fn process_new_guild(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
    // Parse args.
    let args = NewGuild::try_from_bytes(data)?;

    // Load accounts.
    let [signer_info, guild_info, system_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    signer_info.is_signer()?;
    guild_info.is_writable()?.is_empty()?.has_seeds(
        &[GUILD, signer_info.key.as_ref()],
        args.guild_bump,
        &coal_guilds_api::id(),
    )?;
    system_program.is_program(&system_program::ID)?;

    // Initialize the guild account.
    create_account::<Guild>(
        guild_info,
        &coal_guilds_api::id(),
        &[GUILD, signer_info.key.as_ref(), &[args.guild_bump]],
        system_program,
        signer_info,
    )?;
    let guild = guild_info.to_account_mut::<Guild>(&coal_guilds_api::ID)?;
    guild.bump = args.guild_bump as u64;
    guild.authority = *signer_info.key;
    guild.exclusive = 1;
    guild.min_stake = 0;
    guild.total_stake = 0;
    guild.last_stake_at = 0;

    Ok(())
}
