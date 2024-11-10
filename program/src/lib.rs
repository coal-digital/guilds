mod initialize;
mod delegate;
mod new_guild;
mod new_member;
mod invite;
mod join;
mod leave;
mod stake;
mod unstake;

use initialize::*;
use delegate::*;
use new_guild::*;
use new_member::*;
use invite::*;
use join::*;
use leave::*;
use stake::*;
use unstake::*;

use coal_guilds_api::instruction::*;
use steel::*;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let (ix, data) = parse_instruction(&coal_guilds_api::ID, program_id, data)?;

    match ix {
        // Member
        GuildInstruction::Join => process_join(accounts, data)?,
        GuildInstruction::Leave => process_leave(accounts, data)?,
        GuildInstruction::Stake => process_stake(accounts, data)?,
        GuildInstruction::Delegate => process_delegate(accounts, data)?,
        GuildInstruction::Unstake => process_unstake(accounts, data)?,

        // Guild
        GuildInstruction::NewGuild => process_new_guild(accounts, data)?,
        GuildInstruction::NewMember => process_new_member(accounts, data)?,
        GuildInstruction::NewInvite => process_new_invite(accounts, data)?,
        // Admin
        GuildInstruction::Initialize => process_initialize(accounts, data)?,
    }

    Ok(())
}

entrypoint!(process_instruction);