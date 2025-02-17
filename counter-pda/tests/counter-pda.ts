import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Counter } from "../target/types/counter";
import { SystemProgram } from "@solana/web3.js";

describe("counter-pda", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.CounterPda as Program<Counter>;

  it("Create Counter!", async () => {
    const [counter, _counterBump] =
      await anchor.web3.PublicKey.findProgramAddress(
        [Buffer.from("counter"), provider.wallet.publicKey.toBytes()],
        program.programId
      );

    const tx = await program.methods.createCounter()
    .accounts({
      authority: provider.wallet.publicKey,
      counter: counter,
      systemProgram: SystemProgram.programId,
    })
    .rpc();
    console.log("Your transaction signature", tx);
  });
});
