
import { ACTIONS_CORS_HEADERS, ActionGetResponse, ActionPostRequest, ActionPostResponse, createPostResponse } from "@solana/actions";
import { PublicKey } from "@solana/web3.js";

import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import { createSignerFromKeypair, generateSigner, percentAmount, signTransaction, signerIdentity } from "@metaplex-foundation/umi";
import { mplTokenMetadata, TokenStandard, createAndMint } from "@metaplex-foundation/mpl-token-metadata";
import { fromWeb3JsPublicKey, toWeb3JsTransaction } from "@metaplex-foundation/umi-web3js-adapters";


export async function POST(request: Request) {
  try{
  const requestBody: ActionPostRequest = await request.json();
  const userPubKey = new PublicKey(requestBody.account);

  const url = new URL(request.url);
  const choice = url.searchParams.get("choice");

  if (choice != null && process.env.wallet != null) {
    const RPC_ENDPOINT = process.env.RPC;
    if(RPC_ENDPOINT == null){return;}
    const umi = createUmi(RPC_ENDPOINT);

    let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(JSON.parse(process.env.wallet)));
    const myKeypairSigner = createSignerFromKeypair(umi, keypair);
    umi.use(signerIdentity(myKeypairSigner));
    umi.use(mplTokenMetadata());

    const mint = generateSigner(umi);

    const metadata = {
      char: { name: "Charmander", symbol: "CHR", uri: "https://arweave.net/FKRhaMOv0WUtZLBkzvzeUzeOYBhMcLjU-18dtsib3fE" },
      squir: { name: "Squirtle", symbol: "SQR", uri: "https://arweave.net/YUiC6b9MbDPLx1Nr2nqT7l3wX0BuO_gvN85lS_COD9s" },
      bulba: { name: "Bulbasaur", symbol: "BLB", uri: "https://arweave.net/F0J9tWyBCJ7DLaIbk8TRgqgutfUI_S_bopgIfp48ukw" }
    };

    const { name, symbol, uri } = metadata[choice as 'char' | 'squir' | 'bulba'];

    const mintInstructions = createAndMint(umi, {
      mint,
      authority: umi.identity,
      name,
      symbol,
      uri,
      sellerFeeBasisPoints: percentAmount(0),
      decimals: 0,
      amount: 1,
      tokenOwner: fromWeb3JsPublicKey(userPubKey),
      tokenStandard: TokenStandard.NonFungible,
    });

    const bhash = await umi.rpc.getLatestBlockhash();

    const transaction = umi.transactions.create({
      version: 0,
      blockhash: (bhash).blockhash,
      instructions: mintInstructions.getInstructions(),
      payer: fromWeb3JsPublicKey(userPubKey),
    });


    try {
      const signedTx = await signTransaction(transaction, [
        mint,
        myKeypairSigner,
      ]);


      const tx = toWeb3JsTransaction(signedTx);
      

      const responseBody: ActionPostResponse = await createPostResponse({
        fields: {
          transaction: tx,
          message: "Check your NFTs",
        },
      });

  
      const response = Response.json(responseBody, { headers: ACTIONS_CORS_HEADERS });
      return response;
    } catch (error) {
      console.error("Error signing transaction:", error);
      throw new Error("Failed to sign transaction");
    }
  }
} catch (error) {
  console.error("Error in POST function:", error);
  return new Response("An unexpected error occurred", { status: 500 });
}
}


// export async function OPTIONS(request : Request){
//   return GET(request);
// }

export async function OPTIONS(request : Request){
  return new Response(null, {headers: ACTIONS_CORS_HEADERS})
}

export async function GET(request: Request) {
  const responseBody : ActionGetResponse = {
    icon: "https://arweave.net/7egjokz-oQ5cVqUge_oIkg1EJHJZ-TW1OoHJH0ClCZs",
    title: "Choose starter",
    description: "Choose your starter pokemon",
    label: "Choose your starter",
    type: "action",
    
    links: {
      actions: [
        {
          href: request.url + "?choice=bulba",
          label: "Choose Bulbasaur"
        },
        {
          href: request.url + "?choice=char",
          label: "Choose Charmander"
        },
        {
          href: request.url + "?choice=squir",
          label: "Choose Squirtle"
        }
      ]
    },

    error: {
      message: "Choose carefully"
    }
  }; 
  const response =  Response.json(responseBody, {headers : ACTIONS_CORS_HEADERS});
  return response;
}