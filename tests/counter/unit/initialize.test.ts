import {
  Connection,
  Keypair,
  Transaction,
  TransactionInstruction,
  sendAndConfirmTransaction,
  LAMPORTS_PER_SOL,
  PublicKey,
  AccountMeta,
  SystemProgram,
} from "@solana/web3.js";
import { Buffer } from "buffer";

const PROGRAM_ID = new PublicKey(
  "BpU1mcCAtpJN6bRzetzNvfP1Z4do5hJGSVPq9MtfeT6J",
);

const INSTRUCTIONS = {
  Initialize: 0,
  Increment: 1,
  Decrement: 2,
};

jest.setTimeout(100000);

test("initialize counter", async () => {
  const connection = new Connection("http://127.0.0.1:8899");
  const payer = Keypair.fromSecretKey(
    new Uint8Array([
      192, 45, 79, 47, 38, 198, 135, 27, 191, 116, 8, 103, 96, 204, 251, 131,
      110, 7, 179, 0, 236, 71, 217, 202, 191, 140, 13, 148, 165, 62, 107, 20,
      118, 252, 252, 98, 134, 2, 49, 17, 166, 221, 114, 65, 149, 220, 228, 81,
      254, 57, 227, 230, 70, 178, 135, 176, 103, 235, 188, 54, 173, 91, 232, 57,
    ]),
  );

  let airdropSignature = await connection.requestAirdrop(
    payer.publicKey,
    5 * LAMPORTS_PER_SOL,
  );

  console.log("airdrop tx:", airdropSignature);

  console.log("payer:", payer.publicKey);
  console.log("balance:", await connection.getBalance(payer.publicKey));

  const [counterAccountPda, canonicalBump] = PublicKey.findProgramAddressSync(
    [payer.publicKey.toBuffer(), Buffer.from("counter_account")],
    PROGRAM_ID,
  );

  const accounts: AccountMeta[] = [
    {
      pubkey: payer.publicKey,
      isSigner: true,
      isWritable: true,
    },
    {
      pubkey: counterAccountPda,
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: SystemProgram.programId,
      isSigner: false,
      isWritable: true,
    },
  ];

  const CODE_SIZE = 1; // byte (octet)
  const BUMP_SIZE = 1; // byte (octet)

  const DATA_SIZE = CODE_SIZE + BUMP_SIZE;

  // instruction data
  const data = Buffer.alloc(DATA_SIZE);
  data.writeUInt8(INSTRUCTIONS.Initialize, 0); // InitializeCounter(bump)
  data.writeUInt8(canonicalBump, CODE_SIZE); // bump

  const instruction = new TransactionInstruction({
    keys: accounts,
    programId: PROGRAM_ID,
    data,
  });

  const tx = new Transaction().add(instruction);

  try {
    const signature = await sendAndConfirmTransaction(connection, tx, [payer]);
  } catch (err) {
    console.error(err);
  }

  // expect(a)
});
