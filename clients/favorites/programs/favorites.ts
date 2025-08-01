/**
 * This code was AUTOGENERATED using the codama library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun codama to update it.
 *
 * @see https://github.com/codama-idl/codama
 */

import {
  containsBytes,
  getU8Encoder,
  type Address,
  type ReadonlyUint8Array,
} from '@solana/kit';
import {
  type ParsedCreatePdaInstruction,
  type ParsedGetPdaInstruction,
} from '../instructions';

export const FAVORITES_PROGRAM_ADDRESS =
  'E4V6siQsowLXsu9akW4CT57ALDEMiMXerTzgYvy3yG7R' as Address<'E4V6siQsowLXsu9akW4CT57ALDEMiMXerTzgYvy3yG7R'>;

export enum FavoritesAccount {
  Favorites,
}

export enum FavoritesInstruction {
  CreatePda,
  GetPda,
}

export function identifyFavoritesInstruction(
  instruction: { data: ReadonlyUint8Array } | ReadonlyUint8Array,
): FavoritesInstruction {
  const data = 'data' in instruction ? instruction.data : instruction;
  if (containsBytes(data, getU8Encoder().encode(0), 0)) {
    return FavoritesInstruction.CreatePda;
  }
  if (containsBytes(data, getU8Encoder().encode(1), 0)) {
    return FavoritesInstruction.GetPda;
  }
  throw new Error(
    'The provided instruction could not be identified as a favorites instruction.',
  );
}

export type ParsedFavoritesInstruction<
  TProgram extends string = 'E4V6siQsowLXsu9akW4CT57ALDEMiMXerTzgYvy3yG7R',
> =
  | ({
      instructionType: FavoritesInstruction.CreatePda;
    } & ParsedCreatePdaInstruction<TProgram>)
  | ({
      instructionType: FavoritesInstruction.GetPda;
    } & ParsedGetPdaInstruction<TProgram>);
