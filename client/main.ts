// client.ts
import {
  Connection,
  PublicKey,
  Keypair,
  Transaction,
  TransactionInstruction,
  LAMPORTS_PER_SOL,
} from '@solana/web3.js';
import * as borsh from 'borsh';

// Your program ID
const PROGRAM_ID = new PublicKey('5Nm79rArgFbmkbTP3aCVSJS65azFwbtjpkDqt49Pa9Ac');

class CreateElectionInstruction {
  name: string;
  candidates: string[];

  constructor({ name, candidates }: { name: string, candidates: string[] }) {
    this.name = name;
    this.candidates = candidates;
  }
}

const schema = new Map([
  [
    CreateElectionInstruction,
    {
      kind: 'struct',
      fields: [
        ['name', 'string'],
        ['candidates', ['string']],
      ],
    },
  ],
]);

async function main() {
  const connection = new Connection('http://localhost:8899', 'confirmed');
  const payer = Keypair.generate();
  const airdropSignature = await connection.requestAirdrop(payer.publicKey, LAMPORTS_PER_SOL);
  await connection.confirmTransaction(airdropSignature);

  const electionAccount = Keypair.generate();

  const createElectionData = new CreateElectionInstruction({
    name: 'Test Election',
    candidates: ['Alice', 'Bob'],
  });

  const data = borsh.serialize(schema, createElectionData);

  const instruction = new TransactionInstruction({
    keys: [
      { pubkey: electionAccount.publicKey, isSigner: true, isWritable: true },
      { pubkey: payer.publicKey, isSigner: true, isWritable: false },
    ],
    programId: PROGRAM_ID,
    data: Buffer.concat([Buffer.from([0]), Buffer.from(data)]),
  });

  const transaction = new Transaction().add(instruction);
  transaction.feePayer = payer.publicKey;
  const { blockhash } = await connection.getRecentBlockhash();
  transaction.recentBlockhash = blockhash;
  transaction.sign(payer, electionAccount);

  const signature = await connection.sendTransaction(transaction, [payer, electionAccount]);
  await connection.confirmTransaction(signature);

  console.log(`Transaction confirmed with signature: ${signature}`);
}

main().catch(console.error);
