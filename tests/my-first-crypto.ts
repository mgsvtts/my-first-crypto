import {
  AnchorProvider,
  Program,
  setProvider,
  workspace,
} from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";
import { MyFirstCrypto } from "../target/types/my_first_crypto";

describe("first-crypto", () => {
  const provider = AnchorProvider.local();
  setProvider(provider);

  const program = workspace.MyFirstCrypto as Program<MyFirstCrypto>;
  const wallet = provider.wallet;

  const [messagePda, messageBump] = PublicKey.findProgramAddressSync(
    [Buffer.from("message"), wallet.publicKey.toBuffer()],
    program.programId
  );
});
