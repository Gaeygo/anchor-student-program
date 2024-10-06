import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AnchorStudentProgram } from "../target/types/anchor_student_program";
import { expect } from "chai";

describe("anchor-student-program", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider);

  const program = anchor.workspace.AnchorStudentProgram as Program<AnchorStudentProgram>;
  const student = {
    name: "Great",
    description: "I love chelsea"
  }
  const updatedDescription = "I love chelsea even more"

  const [studentPDA] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from(student.name), provider.wallet.publicKey.toBuffer()], program.programId)

  it("Student PDA initialized!", async () => {
    // Add your test here.
    // const account = await program.account.movieAccountState.fetch(moviePda);

    const tx = await program.methods.addStudentInfo(student.name, student.description).rpc();
    // console.log("Your transaction signature", tx);
    const account = await program.account.studentAccountState.fetch(studentPDA)

    expect(account.message === student.description)
    expect(account.studentName === student.name)
    expect(account.student.toBuffer() === studentPDA.toBuffer())

  });

  it("should update student details", async () => {
    const tx = await program.methods.updateStudentInfo(student.name, updatedDescription).rpc()
    const account = await program.account.studentAccountState.fetch(studentPDA)

    expect(updatedDescription === account.message)
    // console.log("Your transaction signature", tx);

  })

  it("should delete student's account", async () => {
    // const tx = await program.methods.deleteStudentInfo(student.name).rpc()
    try {
      await program.methods.deleteStudentInfo(student.name).rpc();
      try {
        await program.account.studentAccountState.fetch(studentPDA);
        throw new Error("Account should have been deleted");
      } catch (error) {
        expect(error.message).to.include("Account does not exist");
      }
    } catch (error) {
      console.error("Error deleting movie review:", error);
      throw error;
    }
    // console.log("Your transaction signature", tx);

  })
});
