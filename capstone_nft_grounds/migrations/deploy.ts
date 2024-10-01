import * as anchor from '@project-serum/anchor';
import { PublicKey } from '@solana/web3.js';
import * as fs from 'fs';
import { Keypair } from '@solana/web3.js';

const PROGRAM_KEYPAIR_PATH = './target/deploy/nft_grounds-keypair.json';

async function main() {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  // Load program keypair
  const programKeypair = Keypair.fromSecretKey(
    Uint8Array.from(JSON.parse(fs.readFileSync(PROGRAM_KEYPAIR_PATH, 'utf-8')))
  );

  // Get the deployed program ID
  const programId = new PublicKey(programKeypair.publicKey);

  console.log(`Deploying program with ID: ${programId.toBase58()}`);

  try {
    // Load the IDL file for your program
    const idl = JSON.parse(
      fs.readFileSync('./target/idl/nft_grounds.json', 'utf-8') // Adjust based on your project
    );

    // Load the Anchor program using the provider
    const program = new anchor.Program(idl, programId, provider);

    console.log('Program deployed successfully.');
  } catch (error) {
    console.error('Error deploying program:', error);
  }
}

console.log('Starting deployment...');
main().then(() => {
  console.log('Deployment complete.');
}).catch((err) => {
  console.error(err);
});
