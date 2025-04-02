
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Pyth } from "../target/types/pyth";
import {
  PythSolanaReceiver,
  InstructionWithEphemeralSigners,
} from "@pythnetwork/pyth-solana-receiver";
import * as buffer from "buffer";
import { AnchorProvider, BN, Wallet } from "@coral-xyz/anchor";
import { HermesClient } from "@pythnetwork/hermes-client";
import { Transaction } from "@solana/web3.js";
const { SystemProgram, Keypair, Connection, PublicKey } = require('@solana/web3.js');
const fs = require('fs');
const path = require('path');
const readline = require('readline');
const SOL_PRICE_FEED_ID = "0xe62df6c8b4a85fe1a67db44dc12de5db330f7ac66b72dc658afedf0f4a415b43"; //BTC/USD
const HERMES_URL = "https://hermes.pyth.network/";
const DEVNET_RPC_URL = "https://api.devnet.solana.com";
const connection = new Connection(DEVNET_RPC_URL);
const provider = anchor.AnchorProvider.env()
const wallet = anchor.web3.Keypair.fromSecretKey(new Uint8Array(JSON.parse(fs.readFileSync("/Users/odomart/.config/solana/id.json", 'utf8'))));

describe("pyth", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(provider);

  const program = anchor.workspace.Pyth as Program<Pyth>;

  it("Is initialized!", async () => {
    const priceServiceConnection = new HermesClient(
      HERMES_URL,
      {}
    );

    const pythSolanaReceiver = new PythSolanaReceiver({
      connection: connection,
      wallet: new Wallet(wallet),
    });
    const priceUpdateData = await priceServiceConnection.getLatestPriceUpdates([SOL_PRICE_FEED_ID], { encoding: "base64" });

    // Build transaction
    const transactionBuilder = pythSolanaReceiver.newTransactionBuilder({
      closeUpdateAccounts: true,
    });
    await transactionBuilder.addPostPriceUpdates(priceUpdateData.binary.data);

    await transactionBuilder.addPriceConsumerInstructions(
      async (getPriceUpdateAccount: (priceFeedId: string) => typeof PublicKey): Promise<InstructionWithEphemeralSigners[]> => {
        return [{
          instruction: await program.methods
            .sample() // Replace with your actual method and parameters
            .accounts({
              payer: wallet.publicKey,
              priceUpdate: getPriceUpdateAccount(SOL_PRICE_FEED_ID),
              // Add other required accounts here
            })
            .instruction(),
          signers: [],
        }];
      }
    );

    const txs = await pythSolanaReceiver.provider.sendAll(
      await transactionBuilder.buildVersionedTransactions({
        computeUnitPriceMicroLamports: 500000,
      }),
      { skipPreflight: true }
    );
    for (const signature of txs) {
      try {
        const tx = await connection.getTransaction(signature, { maxSupportedTransactionVersion: 0 }, { commitment: 'confirmed' });

        if (tx && tx.meta && tx.meta.logMessages) {
          console.log("Transaction logs:", tx.meta.logMessages);
        } else {
          console.log(" Solana Explorer:");
          console.log(`https://explorer.solana.com/tx/${signature}?cluster=devnet`);
        }
      } catch (error) {
        console.error("Error fetching transaction logs for signature:", signature, error);
      }
    }


  });
});