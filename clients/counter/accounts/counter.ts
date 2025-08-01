/**
 * This code was AUTOGENERATED using the codama library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun codama to update it.
 *
 * @see https://github.com/codama-idl/codama
 */

import {
  assertAccountExists,
  assertAccountsExist,
  combineCodec,
  decodeAccount,
  fetchEncodedAccount,
  fetchEncodedAccounts,
  fixDecoderSize,
  fixEncoderSize,
  getBytesDecoder,
  getBytesEncoder,
  getStructDecoder,
  getStructEncoder,
  type Account,
  type Address,
  type Codec,
  type Decoder,
  type EncodedAccount,
  type Encoder,
  type FetchAccountConfig,
  type FetchAccountsConfig,
  type MaybeAccount,
  type MaybeEncodedAccount,
  type ReadonlyUint8Array,
} from '@solana/kit';

export type Counter = { count: ReadonlyUint8Array };

export type CounterArgs = Counter;

export function getCounterEncoder(): Encoder<CounterArgs> {
  return getStructEncoder([['count', fixEncoderSize(getBytesEncoder(), 8)]]);
}

export function getCounterDecoder(): Decoder<Counter> {
  return getStructDecoder([['count', fixDecoderSize(getBytesDecoder(), 8)]]);
}

export function getCounterCodec(): Codec<CounterArgs, Counter> {
  return combineCodec(getCounterEncoder(), getCounterDecoder());
}

export function decodeCounter<TAddress extends string = string>(
  encodedAccount: EncodedAccount<TAddress>,
): Account<Counter, TAddress>;
export function decodeCounter<TAddress extends string = string>(
  encodedAccount: MaybeEncodedAccount<TAddress>,
): MaybeAccount<Counter, TAddress>;
export function decodeCounter<TAddress extends string = string>(
  encodedAccount: EncodedAccount<TAddress> | MaybeEncodedAccount<TAddress>,
): Account<Counter, TAddress> | MaybeAccount<Counter, TAddress> {
  return decodeAccount(
    encodedAccount as MaybeEncodedAccount<TAddress>,
    getCounterDecoder(),
  );
}

export async function fetchCounter<TAddress extends string = string>(
  rpc: Parameters<typeof fetchEncodedAccount>[0],
  address: Address<TAddress>,
  config?: FetchAccountConfig,
): Promise<Account<Counter, TAddress>> {
  const maybeAccount = await fetchMaybeCounter(rpc, address, config);
  assertAccountExists(maybeAccount);
  return maybeAccount;
}

export async function fetchMaybeCounter<TAddress extends string = string>(
  rpc: Parameters<typeof fetchEncodedAccount>[0],
  address: Address<TAddress>,
  config?: FetchAccountConfig,
): Promise<MaybeAccount<Counter, TAddress>> {
  const maybeAccount = await fetchEncodedAccount(rpc, address, config);
  return decodeCounter(maybeAccount);
}

export async function fetchAllCounter(
  rpc: Parameters<typeof fetchEncodedAccounts>[0],
  addresses: Array<Address>,
  config?: FetchAccountsConfig,
): Promise<Account<Counter>[]> {
  const maybeAccounts = await fetchAllMaybeCounter(rpc, addresses, config);
  assertAccountsExist(maybeAccounts);
  return maybeAccounts;
}

export async function fetchAllMaybeCounter(
  rpc: Parameters<typeof fetchEncodedAccounts>[0],
  addresses: Array<Address>,
  config?: FetchAccountsConfig,
): Promise<MaybeAccount<Counter>[]> {
  const maybeAccounts = await fetchEncodedAccounts(rpc, addresses, config);
  return maybeAccounts.map((maybeAccount) => decodeCounter(maybeAccount));
}

export function getCounterSize(): number {
  return 8;
}
