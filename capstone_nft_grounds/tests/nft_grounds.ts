// import * as anchor from "@coral-xyz/anchor";
// import { Program } from "@coral-xyz/anchor";
// import { NftGrounds, NFT_GROUNDS_IDL } from "../target/types/nft_grounds";
// import { assert } from "chai";

// describe("nft_grounds", () => {
//   // Configure the client to use the local cluster.
//   const provider = anchor.AnchorProvider.env();
//   anchor.setProvider(provider);

//   const program = anchor.workspace.NftGrounds as Program<typeof NFT_GROUNDS_IDL>;

//   // it("Is initialized!", async () => {
//   //   // Add your test here.
//   //   const marketplaceKeypair = anchor.web3.Keypair.generate();

//   //   const tx = await program.methods
//   //     .initializeMarketplace()
//   //     .accounts({
//   //       admin: provider.wallet.publicKey,
//   //       marketplace: marketplaceKeypair.publicKey,
//   //       systemProgram: anchor.web3.SystemProgram.programId,
//   //     })
//   //     .signers([marketplaceKeypair])
//   //     .rpc();

//   //   console.log("Your transaction signature", tx);
//   //   assert.ok(tx, "Transaction should be successful");

//   //   // Fetch the created account
//   //   const marketplaceAccount = await program.account.marketplace.fetch(marketplaceKeypair.publicKey);
//   //   assert.ok(marketplaceAccount, "Marketplace account should exist");
//   //   assert.equal(marketplaceAccount.admin.toBase58(), provider.wallet.publicKey.toBase58(), "Admin should be set correctly");
//   // });
// });

