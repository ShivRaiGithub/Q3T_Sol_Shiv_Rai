import wallet from "../wba-wallet.json"
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createGenericFile, createSignerFromKeypair, signerIdentity } from "@metaplex-foundation/umi"
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys"

// Create a devnet connection
const umi = createUmi('https://api.devnet.solana.com');

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);

umi.use(irysUploader());
umi.use(signerIdentity(signer));

(async () => {
    try {
        // Follow this JSON structure
        // https://docs.metaplex.com/programs/token-metadata/changelog/v1.0#json-structure

        const imageURI = "https://arweave.net/tikdBK-SiJsFTPQV1lrA-IWnjc9vdxUPUURcOwtAEGc";
        const metadata = {
            name: "THE RUG",
            symbol: "RUG",
            description: "Rug, rug, yes rug",
            image: imageURI,
            attributes: [
                {trait_type: 'Pattern', value: 'WhiteFLower'}
            ],
            properties: {
                files: [
                    {
                        type: "image/png",
                        uri: imageURI
                    },
                ]
            },
            creators: []
        };
        const myUri = await umi.uploader.uploadJson(metadata);
        console.log("Your image URI: ", myUri);
    }
    catch(error) {
        console.log("Oops.. Something went wrong", error);
    }
})();
