use super::super::types::*;
use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x67")]
pub struct ChangeMarketStatus {
    pub market_status: MarketStatus,
}

pub struct ChangeMarketStatusInstructionAccounts {
    pub phoenix_program: solana_sdk::pubkey::Pubkey,
    pub log_authority: solana_sdk::pubkey::Pubkey,
    pub market: solana_sdk::pubkey::Pubkey,
    pub market_authority: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ChangeMarketStatus {
    type ArrangedAccounts = ChangeMarketStatusInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [phoenix_program, log_authority, market, market_authority, _remaining @ ..] = accounts
        else {
            return None;
        };

        Some(ChangeMarketStatusInstructionAccounts {
            phoenix_program: phoenix_program.pubkey,
            log_authority: log_authority.pubkey,
            market: market.pubkey,
            market_authority: market_authority.pubkey,
        })
    }
}
