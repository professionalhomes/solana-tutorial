import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { BasicStorage } from "../target/types/basic_storage";

describe("basic_storage", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.BasicStorage as Program<BasicStorage>;

  it("Is initialized!", async () => {
    await program.methods.initialize().accounts({}).rpc();
  });
});
