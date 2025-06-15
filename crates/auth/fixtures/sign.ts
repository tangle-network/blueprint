import { DEV_PHRASE } from "@polkadot/keyring";
import { u8aToHex, u8aToU8a } from "@polkadot/util";
import {
    cryptoWaitReady,
    keyExtractSuri,
    keyFromPath,
    mnemonicToMiniSecret,
    secp256k1PairFromSeed,
    sr25519PairFromSeed,
    sr25519Sign,
} from "@polkadot/util-crypto";
import { secp256k1Sign } from "@polkadot/wasm-crypto";
// (when using the API and waiting on `isReady` this is done automatically)
await cryptoWaitReady();
const suri = `${DEV_PHRASE}//Alice`;
const { derivePath, password, path, phrase } = keyExtractSuri(suri);
// create a keyring with some non-default values specified
const seed = mnemonicToMiniSecret(phrase, password);
const keypairEcdsa = secp256k1PairFromSeed(seed);
const keypairSr25519 = sr25519PairFromSeed(seed);

const aliceEcdsa = keyFromPath(keypairEcdsa, path, "ecdsa");
const aliceSr25519 = keyFromPath(keypairSr25519, path, "sr25519");
const ALICE_PUBLIC_ECDSA_KEY_HEX = u8aToHex(
    aliceEcdsa.publicKey,
    undefined,
    false
);

const challengeString = u8aToHex(new Uint8Array(32), undefined, false);
const challengeBytes = u8aToU8a(`0x${challengeString}`);
const signature = secp256k1Sign(challengeBytes, aliceEcdsa.secretKey);
const signatureHex = u8aToHex(signature, undefined, false);

type IKeyType = "Sr25519" | "Ecdsa";

interface VerifyChallengeRequest {
    pub_key: string;
    key_type: IKeyType;
    challenge: string;
    signature: string;
    expires_at: number;
}

const verifyRequestBodyEcdsa = {
    pub_key: ALICE_PUBLIC_ECDSA_KEY_HEX,
    key_type: "Ecdsa",
    challenge: challengeString,
    signature: signatureHex,
    expires_at: 0,
} satisfies VerifyChallengeRequest;

console.log("Verify Challenge (ECDSA) Request Body:");
console.log(JSON.stringify(verifyRequestBodyEcdsa, null, 2));

const verifyRequestBodySr25519 = {
    pub_key: u8aToHex(aliceSr25519.publicKey, undefined, false),
    key_type: "Sr25519",
    challenge: challengeString,
    signature: u8aToHex(
        sr25519Sign(challengeBytes, aliceSr25519),
        undefined,
        false
    ),
    expires_at: 0,
} satisfies VerifyChallengeRequest;
console.log("Verify Challenge (Sr25519) Request Body:");
console.log(JSON.stringify(verifyRequestBodySr25519, null, 2));
