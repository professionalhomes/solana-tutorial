import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AnchorFunctionTutorial } from "../target/types/anchor_function_tutorial";

describe("anchor-function-tutorial", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.AnchorFunctionTutorial as Program<AnchorFunctionTutorial>;

  it("Call boaty mcboatface", async () => {
    // Add your test here.
    const tx = await program.methods.boatyMcBoatface().rpc();
    console.log("Your transaction signature", tx);
  });
  it("Call add fun", async () => {
    const tx = await program.methods.add(new anchor.BN(4), new anchor.BN(9)).rpc();
    console.log("sum is ", tx);
  });
  it("Call sub fun", async () => {
    const tx = await program.methods.sub(new anchor.BN(4), new anchor.BN(1)).rpc();
    console .log("sub is ", tx);
  })
});
