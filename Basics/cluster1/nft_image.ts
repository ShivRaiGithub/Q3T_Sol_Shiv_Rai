import wallet from "../wba-wallet.json"
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createGenericFile, createSignerFromKeypair, signerIdentity } from "@metaplex-foundation/umi"
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys"
import { readFile } from "fs/promises"

// Create a devnet connection
const umi = createUmi('https://api.devnet.solana.com');

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);

umi.use(irysUploader());
umi.use(signerIdentity(signer));

const uploadImage = async (imgSrc:string,typeImg:string) => {
    try {
        //1. Load image
        const image = await readFile(imgSrc);
        //2. Convert image to generic file.
        const genericFile = createGenericFile(image,typeImg );
        //3. Upload image
        const [myUri] = await umi.uploader.upload([genericFile]);

        console.log("Your image URI: ", myUri);
    }
    catch(error) {
        console.log("Oops.. Something went wrong", error);
    }
};
(async () => {
    console.log("Image uri of pokeballs");
    uploadImage("tempImg/pokeballs.jpg", "image/jpg");
    console.log("Image uri of char");
    uploadImage("tempImg/char.jpeg", "image/jpeg");
    console.log("Image uri of bulba");
    uploadImage("tempImg/bulba.jpeg", "image/jpeg");
    console.log("Image uri of squir");
    uploadImage("tempImg/squir.jpeg","image/jpeg");

})();