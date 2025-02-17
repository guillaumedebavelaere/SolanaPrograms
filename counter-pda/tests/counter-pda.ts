import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { CounterPda } from "../target/types/counter_pda";
import { SystemProgram } from "@solana/web3.js";

describe("counter-pda", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.CounterPda as Program<CounterPda>;

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

  it("Increment Counter!", async () => {
    const [counter, _counterBump] =
      await anchor.web3.PublicKey.findProgramAddress(
        [Buffer.from("counter"), provider.wallet.publicKey.toBytes()],
        program.programId
      );

    const tx = await program.methods.updateCounter()
    .accounts({
      authority: provider.wallet.publicKey,
      counter: counter,
    })
    .rpc();
    console.log("Your transaction signature", tx);
  });

  it("Close Counter!", async () => {
    const [counter, _counterBump] =
      await anchor.web3.PublicKey.findProgramAddress(
        [Buffer.from("counter"), provider.wallet.publicKey.toBytes()],
        program.programId
      );

    const tx = await program.methods.closeCounter()
    .accounts({
      authority: provider.wallet.publicKey,
      counter: counter,
    })
    .rpc();
    console.log("Your transaction signature", tx);
  });
});
