import { AnchorProvider, Program, setProvider, workspace } from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";
import { MyFirstCrypto } from "../target/types/my_first_crypto";
 
describe("first-crypto", () => {
  const provider = AnchorProvider.local();
  setProvider(provider);

  const program = workspace.MyFirstCrypto as Program<MyFirstCrypto>;
  const wallet = provider.wallet;
 
  const [messagePda, messageBump] = PublicKey.findProgramAddressSync(
    [Buffer.from("message"), wallet.publicKey.toBuffer()],
    program.programId,
  );

  it("Create Message Account", async () => {
    const message = "Hello, World!";
    const transactionSignature = await program.methods
      .create(message)
      .accounts({
        messageAccount: messagePda,
      })
      .rpc({ commitment: "confirmed" });
 
    const messageAccount = await program.account.messageAccount.fetch(
      messagePda,
      "confirmed",
    );
 
    console.log(JSON.stringify(messageAccount, null, 2));
  });
  it("Update Message Account", async () => {
    const message = "Hello, Solana!";
    await program.methods
      .update(message)
      .accounts({
        messageAccount: messagePda,
      })
      .rpc({ commitment: "confirmed" });
 
    const messageAccount = await program.account.messageAccount.fetch(
      messagePda,
      "confirmed",
    );
 
    console.log(JSON.stringify(messageAccount, null, 2));
  });
  it("Delete Message Account", async () => {
    const transactionSignature = await program.methods
      .delete()
      .accounts({
        messageAccount: messagePda,
      })
      .rpc({ commitment: "confirmed" });
 
    const messageAccount = await program.account.messageAccount.fetchNullable(
      messagePda,
      "confirmed",
    );
 
    console.log("Expect Null:", JSON.stringify(messageAccount, null, 2));
  });
});