import { AccountMeta, TransactionInstruction } from "@solana/web3.js";

export enum CounterInstructions {
  InitializeCounter,
  IncrementCounter,
  DecrementCounter,
}

// export interface Builder {
//   accounts: () =>
//   instruction: (
//     code: CounterInstructions,
//     data: Buffer,
//   ) => TransactionInstruction;
// }

// export function getCounterBuilder(): Builder {
//   return {};
// }
