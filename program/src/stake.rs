use coal_guilds_api::prelude::*;
use steel::*;

/// Stake adds tokens to a guild member's stake to earn a multiplier.
pub fn process_stake(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
    // Parse args.
    let args = Stake::try_from_bytes(data)?;
    let amount = u64::from_le_bytes(args.amount);

    // Load accounts.
    let [signer_info, config_info, guild_info, member_info, member_tokens_info, stake_tokens_info, token_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    signer_info.is_signer()?;
    member_tokens_info
        .is_writable()?
        .to_token_account()?
        .check(|t| t.owner.eq(signer_info.key))?
        .check(|t| t.mint.eq(&LP_MINT_ADDRESS))?;
    stake_tokens_info
        .is_writable()?
        .to_associated_token_account(member_info.key, &LP_MINT_ADDRESS)?;    
    token_program.is_program(&spl_token::ID)?;

    let config = config_info
        .is_writable()?
        .to_account_mut::<Config>(&coal_guilds_api::ID)?;
    let member = member_info
        .is_writable()?
        .to_account_mut::<Member>(&coal_guilds_api::ID)?
        .check_mut(|m| m.authority.eq(signer_info.key))?
        .check_mut(|m| m.guild.eq(guild_info.key))?;
    

    // Update balances.
    member.total_stake = member.total_stake.checked_add(amount).unwrap();
    config.total_stake = config.total_stake.checked_add(amount).unwrap();
    
    // Update timestamps.
    let clock = Clock::get()?;
    member.last_stake_at = clock.unix_timestamp;

    if member.guild.ne(&system_program::ID) {
        let guild = guild_info
            .is_writable()?
            .to_account_mut::<Guild>(&coal_guilds_api::ID)?;
        guild.total_stake = guild.total_stake.checked_add(amount).unwrap();
        guild.last_stake_at = clock.unix_timestamp;
    }

    // Transfer tokens.
    transfer(
        signer_info,
        member_tokens_info,
        stake_tokens_info,
        token_program,
        amount,
    )?;

    Ok(())
}