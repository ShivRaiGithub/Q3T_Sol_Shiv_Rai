// use anchor_lang::prelude::*;

// use mpl_core::{
//     ID as MPL_CORE_ID,
//     fetch_plugin,
//     accounts::{BaseAssetV1, BaseCollectionV1},
//     types::{PluginType, Attributes, Attribute, Plugin, PluginAuthority},
//     instructions::{UpdatePluginV1CpiBuilder,AddPluginV1CpiBuilder},
// };

// declare_id!("CWHxW3xWt1iNZ2K2TSiZkZC9wthHaRc35AuX8hYrtQFD");

// #[program]
// pub mod meta_stake {
//     use std::f32::consts::E;

//     use mpl_core::types::{Attribute, Plugin};

//     use super::*;

//     pub fn stake(ctx: Context<Stake>) -> Result<()> {
//         match fetch_plugin::<BaseAssetV1,Attributes>(&ctx.accounts.asset.to_account_info(), PluginType::Attributes) {
//             Ok((_, fetched_attribute_list, _)) =>{
//                 let mut attribute_list: Vec<Attribute> = vec![];
//                 let mut is_initialized = false;
                
//                 for attribute in fetched_attribute_list.attribute_list() {
//                     if attribute.key == "staked" {
//                         require!(attribute.value=="0", StakingError::AlreadyStaked);
//                         attribute_list.push(Attribute{
//                             key: "staked".to_string,
//                             value: Clock::get()?.unix_timestamp.to_string(),
//                         });
//                         is_initialized = true;

//                     }else{
//                         attribute_list.push(attribute);
//                     }
//                 }
//                 if !is_initialized{
//                     attribute_list.push(Attribute{
//                         key: "staked".to_string(),
//                         value: Clock::get()?.unix_timestamp.to_string(),
//                     });
//                     attribute_list.push(Attribute{
//                         key: "staked_time".to_string(),
//                         value: "0".to_string(),
//                     });

//                     AddPluginV1CpiBuilder::new(&ctx.accounts.mpl_core_program.to_account_info())
//                 }

//                 UpdatePluginV1CpiBuilder::new(&ctx.accounts.mpl_core_program.to_account_info())
//                     .asset(&ctx.accounts.asset.to_account_info())
//                     .collection(&ctx.accounts.collection.to_account_info())
//                     .payer(&ctx.accounts.payer.to_account_info())
//                     .authority(&ctx.accounts.update_authority.to_account_info())
//                     .system_program(&ctx.accounts.system_program.to_account_info())
//                     .plugin(Plugin::Attributes(Attributes{
//                         attribute_list,
//                     }))
//                     .invoke()?;
//                 }
//             }
            
//             Ok(())
//     }
//     pub fn unstake(ctx: Context<Stake>) -> Result<()> {
        
//         Ok(())
//     }
// }

// #[derive(Accounts)]
// pub struct Stake<'info> {
//     pub owner: Signer<'info>,
//     pub update_authority: Signer<'info>,
//     #[account(mut)]
//     pub payer: Signer<'info>,
//     #[account(
//         mut,
//         has_one = owner,
//     )]
//     pub asset: Account<'info, BaseAssetV1>,
//     #[account(
//         mut,
//         has_one = update_authority,
//     )]
//     pub collection: Account<'info, BaseCollectionV1>,
//     #[account(address= MPL_CORE_ID)]
//     pub mpl_core_program: UncheckedAccount<'info>,
//     pub system_program: Program<'info, System>,

// }


