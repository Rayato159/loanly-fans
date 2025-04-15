import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { LoanlyFans } from "../target/types/loanly_fans";
import { assert } from "chai";

describe("loanly-fans", () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.LoanlyFans as Program<LoanlyFans>;
  const loaner = (program.provider as anchor.AnchorProvider).wallet;

  it("Initializes a loan contract", async () => {
    const owner = anchor.web3.Keypair.generate();
    const amount = new anchor.BN(1000);
    const dueAt = new anchor.BN(1);

    const tx = await program.methods
      .initialize(owner.publicKey, amount, dueAt)
      .rpc();

    const [contractPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("loan"),
        loaner.publicKey.toBuffer(),
      ],
      program.programId
    );

    const account = await program.account.contract.fetch(contractPda);

    assert.ok(account.owner.equals(owner.publicKey));
    assert.ok(account.amount.eq(amount));
    assert.ok(account.dueAt.eq(dueAt));
    assert.ok(account.isConfirmed === false);
    assert.ok(account.loaner.equals(loaner.publicKey));
  });
});
