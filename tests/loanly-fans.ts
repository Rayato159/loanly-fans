import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { assert } from "chai";
import { LoanlyFans } from "../target/types/loanly_fans";
import { LAMPORTS_PER_SOL, Keypair } from "@solana/web3.js";

describe("loanly-fans", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.LoanlyFans as Program<LoanlyFans>;
  const systemProgram = anchor.web3.SystemProgram.programId;

  const loaner = new Keypair();
  const owner = new Keypair();

  const [contractPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("loan"),
      loaner.publicKey.toBuffer(),
    ],
    program.programId
  );

  it("Airdrop", async () => {
    const connection = provider.connection;

    const ownerAirdropSignature = await connection.requestAirdrop(
      owner.publicKey,
      2 * LAMPORTS_PER_SOL
    );

    const loanerAirdropSignature = await connection.requestAirdrop(
      loaner.publicKey,
      2 * LAMPORTS_PER_SOL
    );

    const latestBlockHash = await connection.getLatestBlockhash();

    await connection.confirmTransaction({
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: ownerAirdropSignature,
    });

    await connection.confirmTransaction({
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: loanerAirdropSignature,
    });

    const ownerAmount = await provider.connection.getBalance(owner.publicKey);
    const loanerAmount = await provider.connection.getBalance(loaner.publicKey);

    assert.ok(ownerAmount > 1 * LAMPORTS_PER_SOL);
    assert.ok(loanerAmount > 1 * LAMPORTS_PER_SOL);
  })

  it("Initializes a contract", async () => {
    const amount = new anchor.BN(1 * LAMPORTS_PER_SOL);
    const dueAt = new anchor.BN(Math.floor(Date.now() / 1000) + (60 * 60 * 24 * 30));

    await program.methods
      .initialize(owner.publicKey, amount, dueAt)
      .accountsPartial({
        loaner: loaner.publicKey,
        contract: contractPda,
        systemProgram,
      })
      .signers([loaner])
      .rpc();

    const account = await program.account.contract.fetch(contractPda);

    assert.ok(account.owner.equals(owner.publicKey));
    assert.ok(account.amount.eq(amount));
    assert.ok(account.dueAt.eq(dueAt));
    assert.ok(account.isConfirmed === false);
    assert.ok(account.loaner.equals(loaner.publicKey));
  });

  it("Confirm loan", async () => {
    console.log("Loaner balance: ", await provider.connection.getBalance(loaner.publicKey));
    console.log("Owner balance: ", await provider.connection.getBalance(owner.publicKey));

    await program.methods
      .loanConfirm()
      .accountsPartial({
        owner: owner.publicKey,
        contract: contractPda,
        loaner: loaner.publicKey,
        systemProgram,
      })
      .signers([owner])
      .rpc();

    const account = await program.account.contract.fetch(contractPda);

    assert.ok(account.isConfirmed === true);
    assert.ok(account.owner.equals(owner.publicKey));

    console.log("Loaner balance: ", await provider.connection.getBalance(loaner.publicKey));
    console.log("Owner balance: ", await provider.connection.getBalance(owner.publicKey));
  });

  it("Loan paid", async () => {
    console.log("Loaner balance: ", await provider.connection.getBalance(loaner.publicKey));
    console.log("Owner balance: ", await provider.connection.getBalance(owner.publicKey));

    await program.methods
      .loanPaid()
      .accountsPartial({
        loaner: loaner.publicKey,
        contract: contractPda,
        owner: owner.publicKey,
        systemProgram,
      })
      .signers([loaner])
      .rpc();

    const account = await program.account.contract.fetch(contractPda);

    assert.ok(account.isPaid === true);
    assert.ok(account.loaner.equals(loaner.publicKey));

    console.log("Loaner balance: ", await provider.connection.getBalance(loaner.publicKey));
    console.log("Owner balance: ", await provider.connection.getBalance(owner.publicKey));
  });
});
