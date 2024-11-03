use coal_guilds_api::prelude::*;
use steel::*;

/// Initialize sets up the boost program.
pub fn process_initialize(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
    // Parse args.
    let args = Initialize::try_from_bytes(data)?;

    // Load accounts.
    let [signer_info, config_info, system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    signer_info.is_signer()?.has_address(&INITIALIZER_ADDRESS)?;
    config_info.is_writable()?.is_empty()?.has_seeds(
        &[CONFIG],
        args.config_bump,
        &coal_guilds_api::ID,
    )?;
    system_program.is_program(&system_program::ID)?;

    // Initialize config account.
    create_account::<Config>(
        config_info,
        &coal_guilds_api::id(),
        &[CONFIG, &[args.config_bump]],
        system_program,
        signer_info,
    )?;
    let config = config_info.to_account_mut::<Config>(&coal_guilds_api::ID)?;
    config.bump = args.config_bump as u64;
    config.total_stake = 0;
    config.total_multiplier = 24; // 24x

    Ok(())
}