import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Vault } from "../target/types/vault";
import { SystemProgram, Keypair } from "@solana/web3.js";
import { BN } from "bn.js";

describe("vault", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.Vault as Program<Vault>;

  const adminKeypair = Keypair.generate();

  before(async () => {
    const airdropSig = await provider.connection.requestAirdrop(
      adminKeypair.publicKey,
      1 * anchor.web3.LAMPORTS_PER_SOL
    );
    await provider.connection.confirmTransaction(airdropSig);
  });

  it.skip("Vault creation", async () => {
    const [vault, _bump] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from("vault")],
      program.programId
    )

    const tx = await program.methods
    .createVault()
    .accounts({
      signer: provider.wallet.publicKey,
      vault: vault,
      systemProgram: SystemProgram.programId
    })
    .rpc();
    console.log("Your transaction signature", tx);
  });

  it("Vault deposit", async () => {
    const [vault, _bump] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from("vault")],
      program.programId
    )

    const tx = await program.methods
    .deposit(new BN(3500000000))
    .accounts({
      signer: provider.wallet.publicKey,
      vault: vault,
      systemProgram: SystemProgram.programId
    })
    .rpc();

    //let vault_after = program.account.vault.fetch(vault);
    //vault_after.amount > 

    console.log("Your transaction signature", tx);
  });

  it("Vault withdraw", async () => {
    const [vault, _bump] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from("vault")],
      program.programId
    )

    const tx = await program.methods
    .withdraw(new BN(100000000))
    .accounts({
      signer: provider.wallet.publicKey,
      vault: vault,
      admin: adminKeypair.publicKey,
      systemProgram: SystemProgram.programId
    })
    .rpc();

    console.log("Your transaction signature", tx);
  });
});
