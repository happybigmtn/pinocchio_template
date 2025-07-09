use anyhow::{anyhow, Result};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::{Signature},
    system_program,
};
use std::str::FromStr;

use crate::game::{BetType, GamePhase, GameState};

pub struct CrapsRpcClient {
    client: RpcClient,
    player: Pubkey,
    program_id: Pubkey,
    is_devnet: bool,
}

// Program constants
const GLOBAL_GAME_STATE_SEED: &[u8] = b"global_game_state";
const TREASURY_SEED: &[u8] = b"treasury";
const BONUS_STATE_SEED: &[u8] = b"bonus_state";
const RNG_STATE_SEED: &[u8] = b"rng_state";
const SCALABLE_PLAYER_SEED: &[u8] = b"scalable_player";
const BET_BATCH_SEED: &[u8] = b"bet_batch";
const EPOCH_OUTCOME_SEED: &[u8] = b"epoch_outcome";

// Instruction discriminants
const IX_PLACE_BET: u8 = 8;
const IX_CLAIM_EPOCH_PAYOUTS: u8 = 14;
const IX_WITHDRAW: u8 = 5;
const IX_DEPOSIT: u8 = 4;

impl CrapsRpcClient {
    pub fn new(url: &str, player: Pubkey, is_devnet: bool) -> Result<Self> {
        let client = RpcClient::new_with_commitment(url, CommitmentConfig::confirmed());
        
        // Get program ID based on network
        let program_id = if is_devnet {
            // You'll need to replace this with your actual devnet program ID
            Pubkey::from_str("YourDevnetProgramIDHere")?
        } else {
            // And this with your mainnet program ID
            Pubkey::from_str("YourMainnetProgramIDHere")?
        };
        
        Ok(Self {
            client,
            player,
            program_id,
            is_devnet,
        })
    }
    
    pub async fn fetch_game_state(&self) -> Result<GameState> {
        let (game_state_pda, _) = Pubkey::find_program_address(
            &[GLOBAL_GAME_STATE_SEED],
            &self.program_id,
        );
        
        let account = self.client.get_account(&game_state_pda)?;
        
        // Parse the account data
        // This is a simplified version - you'll need to match your actual account structure
        let data = account.data;
        if data.len() < 32 {
            return Err(anyhow!("Invalid game state data"));
        }
        
        let epoch = u64::from_le_bytes(data[0..8].try_into()?);
        let _current_dice = data[8];
        let die1 = data[9];
        let die2 = data[10];
        let point = data[11];
        let phase = data[12];
        
        Ok(GameState {
            epoch,
            phase: if phase == 0 { GamePhase::ComeOut } else { GamePhase::Point },
            point: if point == 0 { None } else { Some(point) },
            die1: if die1 == 0 { None } else { Some(die1) },
            die2: if die2 == 0 { None } else { Some(die2) },
            next_roll_slot: 0, // Would need to parse this from actual data
            active_bets: 0,    // Would need to parse this from actual data
            treasury_balance: 0, // Would need to fetch treasury account
        })
    }
    
    pub async fn place_bet(&self, bet_type: BetType, amount: u64) -> Result<Signature> {
        let (game_state_pda, _) = Pubkey::find_program_address(
            &[GLOBAL_GAME_STATE_SEED],
            &self.program_id,
        );
        
        let (player_state_pda, _) = Pubkey::find_program_address(
            &[SCALABLE_PLAYER_SEED, self.player.as_ref()],
            &self.program_id,
        );
        
        // Get current epoch to determine bet batch
        let game_state = self.fetch_game_state().await?;
        let batch_index = 0u32; // Simplified - would need to track actual batch index
        
        let (bet_batch_pda, _) = Pubkey::find_program_address(
            &[
                BET_BATCH_SEED,
                self.player.as_ref(),
                &game_state.epoch.to_le_bytes(),
                &batch_index.to_le_bytes(),
            ],
            &self.program_id,
        );
        
        // Create instruction data
        let mut instruction_data = vec![IX_PLACE_BET];
        instruction_data.extend_from_slice(&game_state.epoch.to_le_bytes());
        instruction_data.push(bet_type.to_program_bet_type());
        instruction_data.extend_from_slice(&[0u8; 7]); // padding
        instruction_data.extend_from_slice(&amount.to_le_bytes());
        instruction_data.extend_from_slice(&[0u8; 8]); // padding
        
        let _instruction = Instruction {
            program_id: self.program_id,
            accounts: vec![
                AccountMeta::new(bet_batch_pda, false),
                AccountMeta::new(player_state_pda, false),
                AccountMeta::new_readonly(game_state_pda, false),
                AccountMeta::new_readonly(self.player, true),
                AccountMeta::new_readonly(system_program::id(), false),
            ],
            data: instruction_data,
        };
        
        // This is a placeholder - in real implementation you'd sign and send the transaction
        Ok(Signature::default())
    }
    
    pub async fn check_and_claim_wins(&self, epoch: u64) -> Result<u64> {
        let (epoch_outcome_pda, _) = Pubkey::find_program_address(
            &[EPOCH_OUTCOME_SEED, &epoch.to_le_bytes()],
            &self.program_id,
        );
        
        // Check if epoch outcome exists
        match self.client.get_account(&epoch_outcome_pda) {
            Ok(_) => {
                // Claim winnings
                self.claim_epoch_payouts(epoch).await
            }
            Err(_) => Ok(0), // No outcome yet
        }
    }
    
    async fn claim_epoch_payouts(&self, epoch: u64) -> Result<u64> {
        // Build claim instruction
        let mut instruction_data = vec![IX_CLAIM_EPOCH_PAYOUTS];
        instruction_data.extend_from_slice(&epoch.to_le_bytes());
        instruction_data.extend_from_slice(&[0u8; 8]); // reserved
        
        // This is simplified - would need all the proper accounts
        let _instruction = Instruction {
            program_id: self.program_id,
            accounts: vec![], // Would need to add all required accounts
            data: instruction_data,
        };
        
        // Placeholder - would execute transaction and parse result
        Ok(0)
    }
    
    pub async fn withdraw_winnings(&self) -> Result<u64> {
        // Get player balance and withdraw
        // This is a placeholder implementation
        Ok(0)
    }
    
    pub async fn get_player_balance(&self) -> Result<u64> {
        let (player_state_pda, _) = Pubkey::find_program_address(
            &[SCALABLE_PLAYER_SEED, self.player.as_ref()],
            &self.program_id,
        );
        
        let account = self.client.get_account(&player_state_pda)?;
        
        // Parse balance from account data
        // This assumes balance is at offset 32 as u64
        if account.data.len() >= 40 {
            Ok(u64::from_le_bytes(account.data[32..40].try_into()?))
        } else {
            Ok(0)
        }
    }
}