use solana_program::{
    program_error::ProgramError,
    msg
};

use borsh::{BorshDeserialize};

use crate::error::HeroError::InvalidInstruction;

use crate::processor::{
    AddRecordArgs, UpdateRecordArgs, BuyRecordArgs
};

pub enum HeroInstruction {

    /// Add Heros into Repository Account
    ///
    /// Accounts expected:
    ///
    /// 0. `[signer]` The account of the person adding the hero
    /// 1. `[writable]` Our repository account should be created prior to this instruction. It will hold all infos about our heros.

    AddRecord(AddRecordArgs),

    /// Set Hero price
    ///
    ///
    /// Accounts expected:
    ///
    /// 0. `[signer]` The account of the setter & signer
    /// 1. `[writable]` Our repository account which saves all onchain data
    /// 2. `[]` The NFT mint token account of which price will be changed
    /// 3. `[]` The associated_token_account of nft mint token account
    
    UpdateRecord(UpdateRecordArgs),

    /// Buy Hero
    ///
    /// Accounts expected:
    ///
    /// 0. `[signer]` The admin account who has update authority of NFTs
    /// 1. `[signer, writable]` The account of the person buys hero
    /// 2. `[writable]` Previous owner of nft
    /// 3. `[writable]` Repository account
    /// 4. `[]` The Dead NFT Mint
    /// 5. `[]` The Dead NFT Token Account
    /// 6. `[]` The Dead NFT Metadata Account
    /// 7. `[]` New NFT mint
    /// 8. `[]` The NFT token account from which send token
    /// 9. `[]` The NFT token account to which receive token
    /// 10. `[]` Token Program Account
    /// 11. `[]` Token Metadata Program Account
    /// 12. `[]` System Program Account
    
    BuyRecord(BuyRecordArgs),

    /// for test
    OnChainMinting
}

impl HeroInstruction{
    
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (tag, rest) = input.split_first().ok_or(InvalidInstruction)?;
        
        msg!("instruction unpack: tag is {}", tag);

        Ok(match tag {
            0 => {
                Self::AddRecord(Self::unpack_add_record_args(rest)?)
            },
            1 => {
                Self::UpdateRecord(Self::unpack_update_record_args(rest)?)
            },
            2 => {
                Self::BuyRecord(Self::unpack_buy_record_args(rest)?)
            },
            3 => Self::OnChainMinting,
            _ => return Err(InvalidInstruction.into()),
        })
    }

    fn unpack_add_record_args(input: &[u8]) -> Result<AddRecordArgs, ProgramError> {
        let args = AddRecordArgs::try_from_slice(input)?;
        Ok(args)
    }

    fn unpack_update_record_args(input: &[u8]) -> Result<UpdateRecordArgs, ProgramError> {
        let args = UpdateRecordArgs::try_from_slice(input)?;
        Ok(args)
    }

    fn unpack_buy_record_args(input: &[u8]) -> Result<BuyRecordArgs, ProgramError> {
        let args = BuyRecordArgs::try_from_slice(input)?;
        Ok(args)
    }
}
