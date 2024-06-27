import { Connection, PublicKey } from "@solana/web3.js";
import * as borsh from "borsh";

export interface IProgram {
  schema: borsh.Schema;
  state: CounterAccountData;
}

export const CounterSchema = {
  struct: { count: "i64", bump: "u8" },
};

// TODO: explore WASM for annotating state
export class CounterAccountData {
  public readonly count;
  public readonly bump;

  constructor(fields: { count: number; bump: number }) {
    if (fields) {
      this.count = fields.count;
      this.bump = fields.bump;
    }
  }
}

export async function getCounterAccountData(
  connection: Connection,
  publicKey: PublicKey,
): Promise<CounterAccountData | undefined> {
  try {
    // get data
    const accountInfo = await connection.getAccountInfo(publicKey);

    if (accountInfo) {
      const fetchedData = accountInfo.data;
      const counterData = borsh.deserialize(
        CounterSchema,
        fetchedData,
      ) as CounterAccountData;
      return counterData;
    }
  } catch (err) {
    console.error(err);
  }
}
