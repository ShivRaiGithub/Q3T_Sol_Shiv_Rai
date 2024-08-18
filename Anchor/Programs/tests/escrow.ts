// import * as anchor from "@coral-xyz/anchor";
// import {Program, BN} from "@coral-xyz/anchor";
// import {Provider} from "@coral-xyz/anchor";
// import { Escrow } from "../target/types/escrow";
// import { program } from "@coral-xyz/anchor/dist/cjs/native/system";
// import {Keypair,PublicKey, SystemProgram} from "@solana/web3.js";
// import {TOKEN_2022_PROGRAM_ID, getAssociatedTokenAddressSync, tokenGroupInitializeGroup} from "@solana/spl-token";
// import { publicKey, token } from "@coral-xyz/anchor/dist/cjs/utils";

// describe("escrow", () => {
//     anchor.setProvider(anchor.AnchorProvider.env());

//     const provider = anchor.getProvider();
//     const connection = provider.connection;

//     const seed = new BN(0);
//     const receive = new BN(100);
//     const deposit = new BN(200);

//     const escrowProgram = anchor.workspace.Escrow as Program<Escrow>;
//     // const escrow = escrowProgram.account.escrow;

//     const [maker,taker,mintA,mintB] = Array.from({length : 4},()=>Keypair.generate());
//     const tokenProgram = TOKEN_2022_PROGRAM_ID;
//     const[maker_ata_a, makerAtaB, takerAtaA, takerAtaB] = [maker,taker]
//     .map((a) => [mintA,mintB]
//         .map((m) => getAssociatedTokenAddressSync(a.publicKey,m.publicKey,false,tokenProgram )))
//         .flat();

//     const escrow = PublicKey.findProgramAddressSync([
//         Buffer.from("escrow"),
//         maker.publicKey.toBuffer(),
//         seed.toArrayLike(Buffer,"le",8),
//     ],escrowProgram.programId);
    
//     const accounts = {
//         maker,
//         mint_a,
//         mint_b,
//         maker_ata_a,
//         escrow,
//         vault,
//         associated_token_program,
//         token_program,
//         system_program
//     };

//     it("Creates a new escrow account", async () => {
//         const tx = await escrowProgram.methods
//         .make(seed,receive,deposit)
//         .accounts({...accounts})
//         .signers([])
//         .rpc();

//         console.log(tx);

//     });


// });



