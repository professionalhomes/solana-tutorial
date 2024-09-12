import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { ComputeUnit } from "../target/types/compute_unit";

describe("compute_unit", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.ComputeUnit as Program<ComputeUnit>;
  const defaultKeyPair = new anchor.web3.PublicKey("4jf9eJdjePz6TBNvc1MNdavfAq87pequgZRxeMAVRVPA");

  it("Is initialized!", async () => {
    // Add your test here.
    let bal_before = await program.provider.connection.getBalance( defaultKeyPair );
    console.log("before", bal_before);
    const tx = await program.methods.initialize().rpc();
    let bal_after = await program.provider.connection.getBalance( defaultKeyPair );
    console.log("after", bal_after);

    console.log("diff:", BigInt(bal_before.toString()) - BigInt(bal_after.toString()));
 });
});



