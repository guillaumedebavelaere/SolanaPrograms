import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Todolist } from "../target/types/todolist";
import { SystemProgram } from "@solana/web3.js";

describe("todolist", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Todolist as Program<Todolist>;

  it("Is user initialized!", async () => {

    const [user, bump] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from("user"), provider.wallet.publicKey.toBytes()],
      program.programId
    );

    const tx = await program.methods
    .initializeUser("Julien")
    .accounts({
      user: user,
      signer: provider.wallet.publicKey,
      systemProgram: SystemProgram.programId
    })
    .rpc();

    console.log("Your transaction signature", tx);
  });

  /*it("Is todo initialized!", async () => {

  });*/
});
