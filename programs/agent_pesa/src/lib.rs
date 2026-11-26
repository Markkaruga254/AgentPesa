use anchor_lang::prelude::*;

// TEMPORARY PLACEHOLDER. Generate a program keypair and replace this address
// here and in Anchor.toml before building for deployment.
declare_id!("11111111111111111111111111111111");

const AGENT_SEED: &[u8] = b"agent";
const VAULT_SEED: &[u8] = b"vault";

#[program]
pub mod agent_pesa {
    use super::*;

    /// Creates the owner's AgentPesa configuration account.
    pub fn initialize_agent(ctx: Context<InitializeAgent>) -> Result<()> {
        let agent = &mut ctx.accounts.agent;
        agent.owner = ctx.accounts.owner.key();
        agent.bump = ctx.bumps.agent;
        agent.vault_bump = ctx.bumps.vault;
        agent.total_deposited = 0;
        agent.total_withdrawn = 0;
        agent.created_at = Clock::get()?.unix_timestamp;

        emit!(AgentInitialized {
            owner: agent.owner,
            agent: agent.key(),
            vault: ctx.accounts.vault.key(),
        });

        Ok(())
    }

    /// Deposits native SOL into the program-derived vault.
    pub fn fund_vault(ctx: Context<FundVault>, amount: u64) -> Result<()> {
        require!(amount > 0, AgentPesaError::ZeroAmount);

        let cpi_accounts = anchor_lang::system_program::Transfer {
            from: ctx.accounts.owner.to_account_info(),
            to: ctx.accounts.vault.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            cpi_accounts,
        );
        anchor_lang::system_program::transfer(cpi_ctx, amount)?;

        let agent = &mut ctx.accounts.agent;
        agent.total_deposited = agent
            .total_deposited
            .checked_add(amount)
            .ok_or(AgentPesaError::ArithmeticOverflow)?;

        emit!(VaultFunded {
            owner: agent.owner,
            vault: ctx.accounts.vault.key(),
            amount,
        });

        Ok(())
    }

    /// Returns native SOL from the PDA vault to its recorded owner.
    pub fn withdraw_vault(ctx: Context<WithdrawVault>, amount: u64) -> Result<()> {
        require!(amount > 0, AgentPesaError::ZeroAmount);

        let vault_info = ctx.accounts.vault.to_account_info();
        let owner_info = ctx.accounts.owner.to_account_info();
        let available = vault_info.lamports();

        require!(available >= amount, AgentPesaError::InsufficientVaultBalance);

        let agent_key = ctx.accounts.agent.key();
        let bump = [ctx.accounts.agent.vault_bump];
        let signer_seeds: &[&[u8]] = &[VAULT_SEED, agent_key.as_ref(), &bump];
        let signer = &[signer_seeds];

        let cpi_accounts = anchor_lang::system_program::Transfer {
            from: vault_info,
            to: owner_info,
        };
        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.system_program.to_account_info(),
            cpi_accounts,
            signer,
        );
        anchor_lang::system_program::transfer(cpi_ctx, amount)?;

        let agent = &mut ctx.accounts.agent;
        agent.total_withdrawn = agent
            .total_withdrawn
            .checked_add(amount)
            .ok_or(AgentPesaError::ArithmeticOverflow)?;

        emit!(VaultWithdrawn {
            owner: agent.owner,
            vault: ctx.accounts.vault.key(),
            amount,
        });

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeAgent<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        payer = owner,
        space = 8 + AgentAccount::INIT_SPACE,
        seeds = [AGENT_SEED, owner.key().as_ref()],
        bump
    )]
    pub agent: Account<'info, AgentAccount>,

    /// CHECK: This PDA is the system-owned, data-free SOL vault. It is
    /// constrained by seeds and is only debited through a signed system CPI.
    #[account(
        mut,
        seeds = [VAULT_SEED, agent.key().as_ref()],
        bump
    )]
    pub vault: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct FundVault<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds = [AGENT_SEED, owner.key().as_ref()],
        bump = agent.bump,
        has_one = owner
    )]
    pub agent: Account<'info, AgentAccount>,

    /// CHECK: PDA address is constrained; the System Program owns this
    /// data-free account after its first successful deposit.
    #[account(
        mut,
        seeds = [VAULT_SEED, agent.key().as_ref()],
        bump = agent.vault_bump
    )]
    pub vault: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct WithdrawVault<'info> {
    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds = [AGENT_SEED, owner.key().as_ref()],
        bump = agent.bump,
        has_one = owner
    )]
    pub agent: Account<'info, AgentAccount>,

    /// CHECK: PDA address is constrained and its system ownership is
    /// required by the System Program transfer CPI.
    #[account(
        mut,
        seeds = [VAULT_SEED, agent.key().as_ref()],
        bump = agent.vault_bump
    )]
    pub vault: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct AgentAccount {
    pub owner: Pubkey,
    pub bump: u8,
    pub vault_bump: u8,
    pub total_deposited: u64,
    pub total_withdrawn: u64,
    pub created_at: i64,
}

#[event]
pub struct AgentInitialized {
    pub owner: Pubkey,
    pub agent: Pubkey,
    pub vault: Pubkey,
}

#[event]
pub struct VaultFunded {
    pub owner: Pubkey,
    pub vault: Pubkey,
    pub amount: u64,
}

#[event]
pub struct VaultWithdrawn {
    pub owner: Pubkey,
    pub vault: Pubkey,
    pub amount: u64,
}

#[error_code]
pub enum AgentPesaError {
    #[msg("Amount must be greater than zero.")]
    ZeroAmount,
    #[msg("Vault does not have enough SOL.")]
    InsufficientVaultBalance,
    #[msg("Arithmetic overflow.")]
    ArithmeticOverflow,
}
