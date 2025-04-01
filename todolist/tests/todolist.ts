import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Todolist } from "../target/types/todolist";
import { SystemProgram, Keypair } from "@solana/web3.js";
import { BN } from 'bn.js';
import { expect } from "chai";

describe("todolist", () => {

  const userKeypair = new Keypair();

  // Configure the client to use the local cluster.
  const defaultProvider = anchor.AnchorProvider.env();
  const provider = new anchor.AnchorProvider(
    defaultProvider.connection,
    new anchor.Wallet(userKeypair),
    defaultProvider.opts
  );
  anchor.setProvider(provider);

  const program = anchor.workspace.Todolist as Program<Todolist>;

  before(async () => {
    const airdropSig = await provider.connection.requestAirdrop(
      userKeypair.publicKey,
      2 * anchor.web3.LAMPORTS_PER_SOL
    );
    await provider.connection.confirmTransaction(airdropSig);
  });

  it("Is user initialized!", async () => {

    const [user, bump] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from("user"), userKeypair.publicKey.toBytes()],
      program.programId
    );

    const tx = await program.methods
    .initializeUser("Julien")
    .accounts({
      user: user,
      signer: userKeypair.publicKey,
      systemProgram: SystemProgram.programId
    })
    .rpc();

    console.log("Your transaction signature", tx);

    const userAccount = await program.account.user.fetch(user);
    expect(userAccount.userPubkey.toBase58()).to.equal(userKeypair.publicKey.toBase58());
    expect(userAccount.nickname).to.equal("Julien");
    expect(userAccount.todoCount).to.equal(0);
  });

  it("Is first todo initialized!", async () => {
    const [user, bump_user] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from("user"), userKeypair.publicKey.toBytes()],
      program.programId
    );

    const todoIndex = new BN(1);
    const [todoPda, todoBump] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from("todo"),
        userKeypair.publicKey.toBytes(),
        todoIndex.toArrayLike(Buffer, "le", 4),
      ],
      program.programId
    );

    const tx = await program.methods
    .initializeTodo(todoIndex.toNumber(), "go to sport")
    .accounts({
      user: user,
      todo: todoPda,
      signer: userKeypair.publicKey,
      systemProgram: SystemProgram.programId
    })
    .rpc();

    console.log("Your transaction signature", tx);

    const userAccount = await program.account.user.fetch(user);
    const todoAccount = await program.account.todo.fetch(todoPda);
    expect(userAccount.todoCount).to.equal(1);
    expect(todoAccount.todoId).to.equal(1);
    expect(todoAccount.description).to.equal("go to sport");
    expect(todoAccount.status).to.eql({ todo: {} });
  });

  it("Updates a todo to status Done", async () => {
    const todoId = new BN(1);

    const [todoPda] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from("todo"),
        userKeypair.publicKey.toBytes(),
        todoId.toArrayLike(Buffer, "le", 4),
      ],
      program.programId
    );

    const tx = await program.methods
      .updateTodo(todoId.toNumber())
      .accounts({
        signer: userKeypair.publicKey,
        todo: todoPda,
      })
      .rpc();

    console.log("UpdateTodo transaction signature:", tx);

    const todoAccount = await program.account.todo.fetch(todoPda);

    expect(todoAccount.status).to.eql({ done: {} });
  });

  it("Close a todo", async () => {
    const todoId = new BN(1);
  
    const [todoPda] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from("todo"),
        userKeypair.publicKey.toBytes(),
        todoId.toArrayLike(Buffer, "le", 4),
      ],
      program.programId
    );
  
    const tx = await program.methods
      .closeTodo(todoId.toNumber())
      .accounts({
        signer: userKeypair.publicKey,
        todo: todoPda,
      })
      .rpc();
  
    console.log("closeTodo transaction signature:", tx);
  
    try {
      await program.account.todo.fetch(todoPda);
      throw new Error("Expected an error, but fetch succeeded");
    } catch (err) {
      console.log("As expected, the todo account no longer exists:", err.message);
    }
  });
});
