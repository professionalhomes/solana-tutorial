import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Tryrust } from "../target/types/tryrust";

describe("tryrust", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Tryrust as Program<Tryrust>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
  it("Are you adult?", async () => {
    const tx = await program.methods.ageChecker(new anchor.BN(23)).rpc();
    console.log("your transaction signture",tx);
  })
  it("The age range", async () => {
    const tx = await program.methods.ageChecker1(new anchor.BN(4)).rpc();
    console.log("your transaction signature", tx);
  })
  it("The array and Vector", async () => {
    const tx = await program.methods.arrayVector().rpc();
    console.log("your transaction signature", tx);
  })
  it("name mapping", async () => {
    const tx = await program.methods.keyValueMapping("Zenko", "Yamamoto").rpc();
    console.log("your transaction signature", tx);
  })
  it("usized example", async () => {
    const tx = await program.methods.usizeExample().rpc();
    console.log("your transaction signature", tx);
  })
});
