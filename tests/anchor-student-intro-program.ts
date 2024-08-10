import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AnchorStudentIntroProgram } from "../target/types/anchor_student_intro_program";
import { expect } from "chai";

describe("anchor-student-intro-program", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()

  anchor.setProvider(provider)

  const program = anchor.workspace.AnchorStudentIntroProgram as Program<AnchorStudentIntroProgram>
  const userWallet = anchor.workspace.AnchorStudentIntroProgram.provider.wallet

  const student = {
    name: "Julian",
    message: "Hello, I am Julian - a solana big fan",
  }

  const [stuPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from(student.name), provider.wallet.publicKey.toBuffer()],
    program.programId
  )

  it("Student added", async () => {
    // Add your test here.
    const tx = await program.methods.addStudentIntro(student.name, student.message).accounts({
      studentIntro: stuPda,
      student: userWallet.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    }).rpc()
    const account = await program.account.studentAccountState.fetch(stuPda)
    expect(student.name === account.name)
    expect(student.message === account.message)
    expect(account.student === provider.wallet.publicKey)
  });
});
