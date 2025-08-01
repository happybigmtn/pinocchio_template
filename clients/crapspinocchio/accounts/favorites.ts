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
  getU8Decoder,
  getU8Encoder,
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

export type Favorites = {
  number: ReadonlyUint8Array;
  color: ReadonlyUint8Array;
  hobby1: ReadonlyUint8Array;
  hobby2: ReadonlyUint8Array;
  hobby3: ReadonlyUint8Array;
  hobby4: ReadonlyUint8Array;
  hobby5: ReadonlyUint8Array;
  bump: number;
};

export type FavoritesArgs = Favorites;

export function getFavoritesEncoder(): Encoder<FavoritesArgs> {
  return getStructEncoder([
    ['number', fixEncoderSize(getBytesEncoder(), 8)],
    ['color', fixEncoderSize(getBytesEncoder(), 50)],
    ['hobby1', fixEncoderSize(getBytesEncoder(), 50)],
    ['hobby2', fixEncoderSize(getBytesEncoder(), 50)],
    ['hobby3', fixEncoderSize(getBytesEncoder(), 50)],
    ['hobby4', fixEncoderSize(getBytesEncoder(), 50)],
    ['hobby5', fixEncoderSize(getBytesEncoder(), 50)],
    ['bump', getU8Encoder()],
  ]);
}

export function getFavoritesDecoder(): Decoder<Favorites> {
  return getStructDecoder([
    ['number', fixDecoderSize(getBytesDecoder(), 8)],
    ['color', fixDecoderSize(getBytesDecoder(), 50)],
    ['hobby1', fixDecoderSize(getBytesDecoder(), 50)],
    ['hobby2', fixDecoderSize(getBytesDecoder(), 50)],
    ['hobby3', fixDecoderSize(getBytesDecoder(), 50)],
    ['hobby4', fixDecoderSize(getBytesDecoder(), 50)],
    ['hobby5', fixDecoderSize(getBytesDecoder(), 50)],
    ['bump', getU8Decoder()],
  ]);
}

export function getFavoritesCodec(): Codec<FavoritesArgs, Favorites> {
  return combineCodec(getFavoritesEncoder(), getFavoritesDecoder());
}

export function decodeFavorites<TAddress extends string = string>(
  encodedAccount: EncodedAccount<TAddress>,
): Account<Favorites, TAddress>;
export function decodeFavorites<TAddress extends string = string>(
  encodedAccount: MaybeEncodedAccount<TAddress>,
): MaybeAccount<Favorites, TAddress>;
export function decodeFavorites<TAddress extends string = string>(
  encodedAccount: EncodedAccount<TAddress> | MaybeEncodedAccount<TAddress>,
): Account<Favorites, TAddress> | MaybeAccount<Favorites, TAddress> {
  return decodeAccount(
    encodedAccount as MaybeEncodedAccount<TAddress>,
    getFavoritesDecoder(),
  );
}

export async function fetchFavorites<TAddress extends string = string>(
  rpc: Parameters<typeof fetchEncodedAccount>[0],
  address: Address<TAddress>,
  config?: FetchAccountConfig,
): Promise<Account<Favorites, TAddress>> {
  const maybeAccount = await fetchMaybeFavorites(rpc, address, config);
  assertAccountExists(maybeAccount);
  return maybeAccount;
}

export async function fetchMaybeFavorites<TAddress extends string = string>(
  rpc: Parameters<typeof fetchEncodedAccount>[0],
  address: Address<TAddress>,
  config?: FetchAccountConfig,
): Promise<MaybeAccount<Favorites, TAddress>> {
  const maybeAccount = await fetchEncodedAccount(rpc, address, config);
  return decodeFavorites(maybeAccount);
}

export async function fetchAllFavorites(
  rpc: Parameters<typeof fetchEncodedAccounts>[0],
  addresses: Array<Address>,
  config?: FetchAccountsConfig,
): Promise<Account<Favorites>[]> {
  const maybeAccounts = await fetchAllMaybeFavorites(rpc, addresses, config);
  assertAccountsExist(maybeAccounts);
  return maybeAccounts;
}

export async function fetchAllMaybeFavorites(
  rpc: Parameters<typeof fetchEncodedAccounts>[0],
  addresses: Array<Address>,
  config?: FetchAccountsConfig,
): Promise<MaybeAccount<Favorites>[]> {
  const maybeAccounts = await fetchEncodedAccounts(rpc, addresses, config);
  return maybeAccounts.map((maybeAccount) => decodeFavorites(maybeAccount));
}

export function getFavoritesSize(): number {
  return 309;
}
