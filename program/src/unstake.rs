use coal_guilds_api::{
    prelude::*,
    consts::MEMBER,
};
use solana_program::msg;
use steel::*;

/// Stake adds tokens to a guild member's stake to earn a multiplier.
pub fn process_unstake(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
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

    if member.last_stake_at.checked_add(UNSTAKE_DELAY).unwrap() > Clock::get()?.unix_timestamp {
        msg!("Too early to unstake");
        return Err(GuildError::TooEarly.into());
    }

    if member.total_stake.lt(&amount) {
        msg!("Insufficient balance");
        return Err(GuildError::InsufficientBalance.into());
    }
        // Update balances.
    member.total_stake = member.total_stake.checked_sub(amount).unwrap();
    config.total_stake = config.total_stake.checked_sub(amount).unwrap();

    if member.guild.ne(&system_program::ID) {
        if member.guild.ne(guild_info.key) {
            return Err(GuildError::InvalidGuild.into());
        }

        // Update guild total stake.
        let guild = guild_info
            .is_writable()?
            .to_account_mut::<Guild>(&coal_guilds_api::ID)?;
        guild.total_stake = guild.total_stake.checked_sub(amount).unwrap();
    }

    // Transfer tokens.
    transfer_signed(
        member_info,
        stake_tokens_info,
        member_tokens_info,
        token_program,
        amount,
        &[&[MEMBER, signer_info.key.as_ref(), &[member.bump as u8]]],
    )?;

    Ok(())
}