use anchor_lang::prelude::*;

declare_id!("A2DNJjEaxWNeuCHPFJqGQxm7Uqsztnub5uYgsdZD3QXM");

#[program]
pub mod leaderboard {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, name: String) -> Result<()> {
        let board = &mut ctx.accounts.leaderboard;
        board.authority = ctx.accounts.authority.key();
        board.name = name;
        board.entry_count = 0;
        msg!("Leaderboard created: {}", board.name);
        Ok(())
    }

    pub fn submit_score(ctx: Context<SubmitScore>, player_name: String, score: u64) -> Result<()> {
        let board = &mut ctx.accounts.leaderboard;
        let entry = &mut ctx.accounts.entry;

        require!(player_name.len() <= 32, LeaderboardError::NameTooLong);

        entry.player = ctx.accounts.player.key();
        entry.player_name = player_name.clone();
        entry.score = score;
        entry.leaderboard = board.key();
        entry.timestamp = Clock::get()?.unix_timestamp;

        board.entry_count += 1;

        msg!("Score submitted: {} -> {}", player_name, score);
        Ok(())
    }

    pub fn update_score(ctx: Context<UpdateScore>, new_score: u64) -> Result<()> {
        let entry = &mut ctx.accounts.entry;

        require!(
            new_score > entry.score,
            LeaderboardError::ScoreNotHigher
        );

        let old_score = entry.score;
        entry.score = new_score;
        entry.timestamp = Clock::get()?.unix_timestamp;

        msg!(
            "Score updated for {}: {} -> {}",
            entry.player_name,
            old_score,
            new_score
        );
        Ok(())
    }

    pub fn reset_leaderboard(ctx: Context<ResetLeaderboard>) -> Result<()> {
        let board = &mut ctx.accounts.leaderboard;
        board.entry_count = 0;
        msg!("Leaderboard reset by authority");
        Ok(())
    }
}

// ─── Accounts ───────────────────────────────────────────────────────────────

#[account]
pub struct LeaderboardState {
    pub authority: Pubkey,   // admin who owns this leaderboard
    pub name: String,        // leaderboard name (max 32 chars)
    pub entry_count: u64,    // total entries submitted
}

#[account]
pub struct PlayerEntry {
    pub leaderboard: Pubkey, // which leaderboard this belongs to
    pub player: Pubkey,      // player's wallet address
    pub player_name: String, // player display name (max 32 chars)
    pub score: u64,          // current best score
    pub timestamp: i64,      // last updated
}

// ─── Contexts ────────────────────────────────────────────────────────────────

#[derive(Accounts)]
#[instruction(name: String)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 4 + 32 + 8,
    )]
    pub leaderboard: Account<'info, LeaderboardState>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(player_name: String)]
pub struct SubmitScore<'info> {
    #[account(mut)]
    pub leaderboard: Account<'info, LeaderboardState>,

    #[account(
        init,
        payer = player,
        space = 8 + 32 + 32 + 4 + 32 + 8 + 8,
        seeds = [b"entry", leaderboard.key().as_ref(), player.key().as_ref()],
        bump,
    )]
    pub entry: Account<'info, PlayerEntry>,

    #[account(mut)]
    pub player: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateScore<'info> {
    #[account(
        mut,
        seeds = [b"entry", entry.leaderboard.as_ref(), player.key().as_ref()],
        bump,
        has_one = player,
    )]
    pub entry: Account<'info, PlayerEntry>,

    pub player: Signer<'info>,
}

#[derive(Accounts)]
pub struct ResetLeaderboard<'info> {
    #[account(mut, has_one = authority)]
    pub leaderboard: Account<'info, LeaderboardState>,

    pub authority: Signer<'info>,
}

// ─── Errors ──────────────────────────────────────────────────────────────────

#[error_code]
pub enum LeaderboardError {
    #[msg("Player name must be 32 characters or less")]
    NameTooLong,
    #[msg("New score must be higher than current score")]
    ScoreNotHigher,
}
