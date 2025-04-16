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

  const [loanerHistoryPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("history"),
      loaner.publicKey.toBuffer(),
    ],
    program.programId
  )

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
      .initializeContract(owner.publicKey, amount, dueAt)
      .accountsPartial({
        loaner: loaner.publicKey,
        contract: contractPda,
        loanerHistory: loanerHistoryPda,
        systemProgram,
      })
      .signers([loaner])
      .rpc();

    const contract = await program.account.contract.fetch(contractPda);
    const loaner_history = await program.account.loanerHistory.fetch(loanerHistoryPda);

    assert.ok(contract.owner.equals(owner.publicKey));
    assert.ok(contract.loaner.equals(loaner.publicKey));
    assert.ok(contract.amount.eq(amount));
    assert.ok(contract.dueAt.eq(dueAt));
    assert.ok(contract.isConfirmed === false);
    assert.ok(contract.isLatePaid === false);

    assert.ok(loaner_history.loaner.equals(loaner.publicKey));
    assert.ok(loaner_history.totalLoans.eq(new anchor.BN(0)));
    assert.ok(loaner_history.latePaidLoans.eq(new anchor.BN(0)));
  });

  it("Confirm loan", async () => {
    console.log(`
      Before confirm loan:
      Loaner balance: ${await provider.connection.getBalance(loaner.publicKey)}, 
      Owner balance: ${await provider.connection.getBalance(owner.publicKey)}`
    );

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

    const contract = await program.account.contract.fetch(contractPda);
    const loaner_history = await program.account.loanerHistory.fetch(loanerHistoryPda);

    assert.ok(contract.isConfirmed === true);
    assert.ok(contract.owner.equals(owner.publicKey));
    assert.ok(loaner_history.loaner.equals(loaner.publicKey));
    assert.ok(loaner_history.totalLoans.eq(new anchor.BN(1)));
    assert.ok(loaner_history.latePaidLoans.eq(new anchor.BN(0)));

    console.log(`
      After confirm loan:
      Loaner balance: ${await provider.connection.getBalance(loaner.publicKey)}, 
      Owner balance: ${await provider.connection.getBalance(owner.publicKey)}`
    );
  });

  it("Loan paid in time", async () => {
    console.log(`
      Before loan paid:
      Loaner balance: ${await provider.connection.getBalance(loaner.publicKey)}, 
      Owner balance: ${await provider.connection.getBalance(owner.publicKey)}`
    );

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

    const contract = await program.account.contract.fetch(contractPda);
    const loaner_history = await program.account.loanerHistory.fetch(loanerHistoryPda);

    assert.ok(contract.isLatePaid === false);
    assert.ok(contract.loaner.equals(loaner.publicKey));
    assert.ok(loaner_history.loaner.equals(loaner.publicKey));
    assert.ok(loaner_history.totalLoans.eq(new anchor.BN(1)));
    assert.ok(loaner_history.latePaidLoans.eq(new anchor.BN(0)));


    console.log(`
      After loan paid:
      Loaner balance: ${await provider.connection.getBalance(loaner.publicKey)}, 
      Owner balance: ${await provider.connection.getBalance(owner.publicKey)}`
    );
  });
});