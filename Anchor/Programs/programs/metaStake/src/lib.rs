use anchor_lang::prelude::*;

use mpl_core::{
    ID as MPL_CORE_ID,
    fetch_plugin,
    accounts::{BaseAssetV1, BaseCollectionV1},
    types::{PluginType, Attributes, Attribute, Plugin, PluginAuthority, FreezeDelegate},
    instructions::{UpdatePluginV1CpiBuilder,AddPluginV1CpiBuilder, RemovePluginV1CpiBuilder},
};

declare_id!("CWHxW3xWt1iNZ2K2TSiZkZC9wthHaRc35AuX8hYrtQFD");

#[program]
pub mod meta_stake {
    use super::*;
    use mpl_core::types::{Attribute, Plugin};

    pub fn stake(ctx: Context<Stake>) -> Result<()> {
        match fetch_plugin::<BaseAssetV1,Attributes>(&ctx.accounts.asset.to_account_info(), PluginType::Attributes) {
            Ok( (_, fetched_attribute_list, _)) =>{
                let mut attribute_list: Vec<Attribute> = vec![];
                let mut is_initialized = false;
                
                for attribute in fetched_attribute_list.attribute_list {
                    if attribute.key == "staked" {
                        require!(attribute.value=="0", StakingError::AlreadyStaked);
                        attribute_list.push(Attribute{
                            key: "staked".to_string(),
                            value: Clock::get()?.unix_timestamp.to_string(),
                        });
                        is_initialized = true;

                    }else{
                        attribute_list.push(attribute);
                    }
                }
                if !is_initialized{
                    attribute_list.push(Attribute{
                        key: "staked".to_string(),
                        value: Clock::get()?.unix_timestamp.to_string(),
                    });
                    attribute_list.push(Attribute{
                        key: "staked_time".to_string(),
                        value: "0".to_string(),
                    });

                }
                
                UpdatePluginV1CpiBuilder::new(&ctx.accounts.mpl_core_program.to_account_info())
                    .asset(&ctx.accounts.asset.to_account_info())
                    .collection(Some(&ctx.accounts.collection.to_account_info()))
                    .payer(&ctx.accounts.payer.to_account_info())
                    .authority(Some(&ctx.accounts.update_authority.to_account_info()))
                    .system_program(&ctx.accounts.system_program.to_account_info())
                    .plugin(Plugin::Attributes(Attributes{
                        attribute_list,
                    }))
                    .invoke()?;
                },
                Err(_)=>{
                    let mut attribute_list: Vec<Attribute> = vec![];

                    attribute_list.push(Attribute{
                        key: "staked".to_string(),
                        value: Clock::get()?.unix_timestamp.to_string(),
                    });
                    attribute_list.push(Attribute{
                        key: "staked_time".to_string(),
                        value: "0".to_string(),
                    });

                    AddPluginV1CpiBuilder::new(&ctx.accounts.mpl_core_program.to_account_info())
                    .asset(&ctx.accounts.asset.to_account_info())
                    .collection(Some(&ctx.accounts.collection.to_account_info()))
                    .payer(&ctx.accounts.payer.to_account_info())
                    .authority(Some(&ctx.accounts.update_authority.to_account_info()))
                    .system_program(&ctx.accounts.system_program.to_account_info())
                    .plugin(Plugin::Attributes(Attributes{
                        attribute_list,
                    }))
                    .invoke()?;
                }
            } 

            AddPluginV1CpiBuilder::new(&ctx.accounts.mpl_core_program.to_account_info())
            .asset(&ctx.accounts.asset.to_account_info())
            .collection(Some(&ctx.accounts.collection.to_account_info()))
            .payer(&ctx.accounts.payer.to_account_info())
            .authority(Some(&ctx.accounts.owner.to_account_info()))
            .system_program(&ctx.accounts.system_program.to_account_info())
            .plugin(Plugin::FreezeDelegate(FreezeDelegate{
                frozen: true
            }))
            .init_authority(PluginAuthority::UpdateAuthority)
            .invoke()?;
            
            Ok(())
    }
    pub fn unstake(ctx: Context<Stake>) -> Result<()> {
        match fetch_plugin::<BaseAssetV1, Attributes>( &ctx.accounts.asset.to_account_info(), PluginType::Attributes) {
            Ok((_, fetch_attribute_list, _))=>{
                let mut attribute_list: Vec<Attribute> = vec![];
                let mut is_initialized = true;
                let mut staked_time:i64 = 0;

                for attribute in fetch_attribute_list.attribute_list {
                    if attribute.key == "staked" {
                        require!(attribute.value!="0", StakingError::NotStaked);
                        attribute_list.push(Attribute{
                            key: "staked".to_string(),
                            value: "0".to_string(),
                        });
                        staked_time = staked_time
                        .checked_add(Clock::get()?.unix_timestamp)
                        .ok_or(StakingError::Overflow)?
                    .checked_sub(attribute.value.parse::<i64>().map_err(|_| StakingError::InvalidTimestamp)?)
                    .ok_or(StakingError::Underflow)?;
                    is_initialized = true;

                    }else if attribute.key == "staked_time" {
                        staked_time = staked_time
                        .checked_add(
                            attribute.value.parse::<i64>().map_err(|_| StakingError::InvalidTimestamp)?
                        ).ok_or(StakingError::Overflow)?;

                        attribute_list.push(Attribute{
                            key: "staked_time".to_string(),
                            value: staked_time.to_string(),
                        });
                         
                    }else{
                        attribute_list.push(attribute);
                    }
                }
                require!(is_initialized, StakingError::NotStaked);

                UpdatePluginV1CpiBuilder::new(&ctx.accounts.mpl_core_program.to_account_info())
                    .asset(&ctx.accounts.asset.to_account_info())
                    .collection(Some(&ctx.accounts.collection.to_account_info()))
                    .payer(&ctx.accounts.payer.to_account_info())
                    .authority(Some(&ctx.accounts.update_authority.to_account_info()))
                    .system_program(&ctx.accounts.system_program.to_account_info())
                    .plugin(Plugin::Attributes(Attributes{
                        attribute_list,
                    }))
                    .invoke()?;
                
            },
        Err(_)=>{
            return Err(StakingError::AttributesNotInitialized.into());
        }}

        UpdatePluginV1CpiBuilder::new(&ctx.accounts.mpl_core_program.to_account_info())
            .asset(&ctx.accounts.asset.to_account_info())
            .collection(Some(&ctx.accounts.collection.to_account_info()))
            .payer(&ctx.accounts.payer.to_account_info())
            .authority(Some(&ctx.accounts.update_authority.to_account_info()))
            .system_program(&ctx.accounts.system_program.to_account_info())
            .plugin(Plugin::FreezeDelegate(FreezeDelegate{
                frozen: false
            }))
            .invoke()?;

        RemovePluginV1CpiBuilder::new(&ctx.accounts.mpl_core_program.to_account_info())
            .asset(&ctx.accounts.asset.to_account_info())
            .collection(Some(&ctx.accounts.collection.to_account_info()))
            .payer(&ctx.accounts.payer.to_account_info())
            .authority(Some(&ctx.accounts.owner.to_account_info()))
            .system_program(&ctx.accounts.system_program.to_account_info())
            .plugin_type(PluginType::FreezeDelegate)
            .invoke()?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Stake<'info> {
    pub owner: Signer<'info>,
    pub update_authority: Signer<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        mut,
        has_one = owner,
    )]
    pub asset: Account<'info, BaseAssetV1>,
    #[account(
        mut,
        has_one = update_authority,
    )]
    pub collection: Account<'info, BaseCollectionV1>,
    #[account(address= MPL_CORE_ID)]
    pub mpl_core_program: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,

}


#[error_code]
pub enum StakingError {
    #[msg("Already staked")]
    AlreadyStaked,
    #[msg("Attributes not initialized")]
    AttributesNotInitialized,
    #[msg("Asset not staked")]
    NotStaked,
    #[msg("Overflow")]
    Overflow,
    #[msg("Underflow")]
    Underflow,
    #[msg("Invalid timestamp")]
    InvalidTimestamp, 
}