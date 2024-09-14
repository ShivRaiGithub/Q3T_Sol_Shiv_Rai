// import * as anchor from "@coral-xyz/anchor";
// import { Program } from "@coral-xyz/anchor";
// import { NftGrounds } from "../target/types/nft_grounds";
// import { Keypair } from "@solana/web3.js";
// import { assert } from "chai";

// describe("nft_grounds", () => {
//   // Configure the client to use the local cluster.
//   const provider = anchor.AnchorProvider.env();
//   anchor.setProvider(provider);

//   const program = anchor.workspace.NftGrounds as Program<NftGrounds>;

//   // Generate keypairs for marketplace, competition, user accounts, etc.
//   const marketplaceKeypair = Keypair.generate();
//   const competitionKeypair = Keypair.generate();
//   const userKeypair = Keypair.generate();

//   it("Initializes the marketplace", async () => {
//     const tx = await program.methods
//       .initializeMarketplace()
//       .accounts({
//         marketplace: marketplaceKeypair.publicKey,
//         authority: provider.wallet.publicKey,
//         systemProgram: anchor.web3.SystemProgram.programId,
//       })
//       .signers([marketplaceKeypair])
//       .rpc();
    
//     console.log("Marketplace initialized, transaction signature", tx);
//     // You can add assertions here if needed
//   });

//   it("Initializes the competition", async () => {
//     const numCompetitions = new anchor.BN(5);
//     const entryFee = 10;

//     const tx = await program.methods
//       .initializeCompetition(numCompetitions, entryFee)
//       .accounts({
//         competition: competitionKeypair.publicKey,
//         marketplace: marketplaceKeypair.publicKey,
//         authority: provider.wallet.publicKey,
//         systemProgram: anchor.web3.SystemProgram.programId,
//       })
//       .signers([competitionKeypair])
//       .rpc();

//     console.log("Competition initialized, transaction signature", tx);
//     // Assert competition is initialized properly
//   });

//   it("Initializes user account", async () => {
//     const tx = await program.methods
//       .initializeUserAccount()
//       .accounts({
//         user: userKeypair.publicKey,
//         authority: provider.wallet.publicKey,
//         systemProgram: anchor.web3.SystemProgram.programId,
//       })
//       .signers([userKeypair])
//       .rpc();

//     console.log("User account initialized, transaction signature", tx);
//     // Additional checks can be added here
//   });

//   it("User pays the entry fee", async () => {
//     const tx = await program.methods
//       .payEntry()
//       .accounts({
//         user: userKeypair.publicKey,
//         competition: competitionKeypair.publicKey,
//         authority: provider.wallet.publicKey,
//         systemProgram: anchor.web3.SystemProgram.programId,
//       })
//       .signers([userKeypair])
//       .rpc();

//     console.log("Entry fee paid, transaction signature", tx);
//     // Additional checks/asserts for balances
//   });

//   it("User enters the competition", async () => {
//     const tx = await program.methods
//       .enterCompetition()
//       .accounts({
//         user: userKeypair.publicKey,
//         competition: competitionKeypair.publicKey,
//         authority: provider.wallet.publicKey,
//         systemProgram: anchor.web3.SystemProgram.programId,
//       })
//       .signers([userKeypair])
//       .rpc();

//     console.log("User entered competition, transaction signature", tx);
//     // Additional checks/asserts for competition entry
//   });

//   it("User votes", async () => {
//     const tx = await program.methods
//       .vote()
//       .accounts({
//         user: userKeypair.publicKey,
//         competition: competitionKeypair.publicKey,
//         authority: provider.wallet.publicKey,
//         systemProgram: anchor.web3.SystemProgram.programId,
//       })
//       .signers([userKeypair])
//       .rpc();

//     console.log("User voted, transaction signature", tx);
//     // Additional checks/asserts for voting
//   });

//   it("User claims points", async () => {
//     const tx = await program.methods
//       .claimPoints()
//       .accounts({
//         user: userKeypair.publicKey,
//         competition: competitionKeypair.publicKey,
//         authority: provider.wallet.publicKey,
//         systemProgram: anchor.web3.SystemProgram.programId,
//       })
//       .signers([userKeypair])
//       .rpc();

//     console.log("User claimed points, transaction signature", tx);
//     // Additional checks/asserts for points
//   });

//   it("User lists an NFT", async () => {
//     const price = new anchor.BN(100); // Set price for the NFT

//     const tx = await program.methods
//       .list(price)
//       .accounts({
//         user: userKeypair.publicKey,
//         marketplace: marketplaceKeypair.publicKey,
//         authority: provider.wallet.publicKey,
//         systemProgram: anchor.web3.SystemProgram.programId,
//       })
//       .signers([userKeypair])
//       .rpc();

//     console.log("NFT listed, transaction signature", tx);
//     // Additional checks/asserts for listing
//   });

//   it("User buys an NFT", async () => {
//     const tx = await program.methods
//       .buy()
//       .accounts({
//         buyer: userKeypair.publicKey,
//         seller: marketplaceKeypair.publicKey,
//         marketplace: marketplaceKeypair.publicKey,
//         systemProgram: anchor.web3.SystemProgram.programId,
//       })
//       .signers([userKeypair])
//       .rpc();

//     console.log("NFT purchased, transaction signature", tx);
//     // Additional checks/asserts for the purchase
//   });
// });


import * as anchor from "@coral-xyz/anchor";
import { Program, BN } from "@coral-xyz/anchor";
import { NftGrounds } from "../target/types/nft_grounds";
import { expect } from "chai";

import {Provider} from "@coral-xyz/anchor";
import { program } from "@coral-xyz/anchor/dist/cjs/native/system";
import {Keypair,PublicKey, SystemProgram} from "@solana/web3.js";
import { publicKey, token } from "@coral-xyz/anchor/dist/cjs/utils"

describe("nft_grounds", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Escrow as Program<NftGrounds>;
  const admin = provider.wallet;
  let marketplacePDA: anchor.web3.PublicKey;
  let competitionPDA: anchor.web3.PublicKey;
  let rankingPDA: anchor.web3.PublicKey;
  let userAccount: anchor.web3.PublicKey;
  let stakeAccount: anchor.web3.PublicKey;

  before(async () => {
    [marketplacePDA] = await anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("marketplace")],
      program.programId
    );

    [competitionPDA] = await anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("competition"), Buffer.from([1]), admin.publicKey.toBuffer()],
      program.programId
    );

    [rankingPDA] = await anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("ranking"), admin.publicKey.toBuffer()],
      program.programId
    );

    [userAccount] = await anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("user"), admin.publicKey.toBuffer()],
      program.programId
    );

    [stakeAccount] = await anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("stake"), admin.publicKey.toBuffer()],
      program.programId
    );
  });

  it("Initializes the marketplace", async () => {
    await program.methods
      .initializeMarketplace()
      .accounts({
        admin: admin.publicKey,
        marketplace: marketplacePDA,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    const marketplaceAccount = await program.account.marketplace.fetch(marketplacePDA);
    expect(marketplaceAccount.admin.toString()).to.equal(admin.publicKey.toString());
  });

  it("Initializes a competition", async () => {
    await program.methods
      .initializeCompetition(new anchor.BN(1), 100)
      .accounts({
        admin: admin.publicKey,
        competition: competitionPDA,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    const competitionAccount = await program.account.competition.fetch(competitionPDA);
    expect(competitionAccount.admin.toString()).to.equal(admin.publicKey.toString());
    expect(competitionAccount.number.toNumber()).to.equal(1);
    expect(competitionAccount.fee).to.equal(100);
  });

  it("Initializes the ranking", async () => {
    await program.methods
      .initializeRanking()
      .accounts({
        admin: admin.publicKey,
        ranking: rankingPDA,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    const rankingAccount = await program.account.ranking.fetch(rankingPDA);
    expect(rankingAccount.admin.toString()).to.equal(admin.publicKey.toString());
  });

  it("Initializes a user account", async () => {
    await program.methods
      .initializeUserAccount()
      .accounts({
        user: admin.publicKey,
        userAccount: userAccount,
        stakeAccount: stakeAccount,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    const userAccountData = await program.account.userAccount.fetch(userAccount);
    expect(userAccountData.points.toNumber()).to.equal(0);
    expect(userAccountData.nftInCompetition).to.be.false;
    expect(userAccountData.paidEntryFees).to.be.false;
  });

  // Add more tests for other functions like startEntry, startCompetition, vote, claim, etc.
});