import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createSignerFromKeypair, signerIdentity, generateSigner, percentAmount } from "@metaplex-foundation/umi"
import { createNft, mplTokenMetadata } from "@metaplex-foundation/mpl-token-metadata";

import wallet from "../wba-wallet.json"
import base58 from "bs58";

const RPC_ENDPOINT = "https://api.devnet.solana.com";
const umi = createUmi(RPC_ENDPOINT);

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const myKeypairSigner = createSignerFromKeypair(umi, keypair);


umi.use(signerIdentity(myKeypairSigner));
umi.use(mplTokenMetadata())

const mint = generateSigner(umi);

(async () => {
    let tx = createNft(umi, {
        mint,
        name: "THE RUG",
        symbol: "RUG",
        uri: "https://arweave.net/2TSxjjMqhhwA917LM2YYrzW4aoI1MxtGZVm4GKmX9ZU",
        sellerFeeBasisPoints: percentAmount(25),

    })
    let result = await tx.sendAndConfirm(umi);
    const signature = base58.encode(result.signature);
    
    console.log(`Succesfully Minted! Check out your TX here:\nhttps://explorer.solana.com/tx/${signature}?cluster=devnet`)

    console.log("Mint Address: ", mint.publicKey);
})();





















const {
    Connection,
    Keypair,
    Transaction,
    sendAndConfirmTransaction,
    PublicKey,
    SystemProgram,
} = require('@solana/web3.js');
const {
    createMint,
    getOrCreateAssociatedTokenAccount,
    mintTo,
    createAssociatedTokenAccountInstruction,
    getMinimumBalanceForRentExemptMint,
} = require('@solana/spl-token');
const {
    createCreateMetadataAccountV2Instruction,
    createUpdateMetadataAccountV2Instruction,
    DataV2,
} = require('@metaplex-foundation/mpl-token-metadata');
const fs = require('fs');
const bs58 = require('bs58');

// Load the wallet
const wallet = JSON.parse(fs.readFileSync("../wba-wallet.json"));
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

// Connection to the Solana devnet
const connection = new Connection("https://api.devnet.solana.com", 'confirmed');

(async () => {
    // Create a new mint
    const mint = await createMint(
        connection,
        keypair,            // Payer of the transaction
        keypair.publicKey,  // Mint authority
        keypair.publicKey,  // Freeze authority
        0                   // Decimals for NFT
    );

    // Get or create the associated token account for the mint and wallet
    const tokenAccount = await getOrCreateAssociatedTokenAccount(
        connection,
        keypair,
        mint,
        keypair.publicKey // Owner of the associated token account
    );

    // Mint 1 token (NFT)
    await mintTo(
        connection,
        keypair,
        mint,
        tokenAccount.address,
        keypair,
        1 // Number of tokens to mint (1 for NFT)
    );

    // Metadata account
    const metadataPDA = (
        await PublicKey.findProgramAddress(
            [
                Buffer.from("metadata"),
                (await PublicKey.findProgramAddress(
                    [],
                    new PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s")
                ))[0].toBuffer(),
                mint.toBuffer(),
            ],
            new PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s")
        )
    )[0];

    const metadata = new DataV2({
        name: "THE RUG",
        symbol: "RUG",
        uri: "https://arweave.net/2TSxjjMqhhwA917LM2YYrzW4aoI1MxtGZVm4GKmX9ZU", // URL for the NFT metadata JSON
        sellerFeeBasisPoints: 250, // 2.5% seller fee
    });

    // Create metadata account transaction
    const transaction = new Transaction().add(
        createCreateMetadataAccountV2Instruction(
            {
                metadata: metadataPDA,
                mint: mint,
                mintAuthority: keypair.publicKey,
                payer: keypair.publicKey,
                updateAuthority: keypair.publicKey,
            },
            {
                createMetadataAccountArgsV2: {
                    data: metadata,
                    isMutable: true,
                },
            }
        )
    );

    // Send and confirm the transaction
    const txid = await sendAndConfirmTransaction(connection, transaction, [keypair]);
    console.log(`Successfully minted NFT!`);
    console.log(`Mint Address: ${mint.toBase58()}`);
    console.log(`Transaction: https://explorer.solana.com/tx/${txid}?cluster=devnet`);
})();
