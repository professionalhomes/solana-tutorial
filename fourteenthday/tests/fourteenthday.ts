import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Fourteenthday } from "../target/types/fourteenthday";

describe("fourteenthday", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Fourteenthday as Program<Fourteenthday>;

  it("Is signed by a single signer", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().accounts({
      signer1: program.provider.publicKey
    }).rpc();
    console.log("The signer1:", program.provider.publicKey.toBase58());

  });

});