/**
 * This code was AUTOGENERATED using the codama library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun codama to update it.
 *
 * @see https://github.com/codama-idl/codama
 */

import {
  combineCodec,
  getStructDecoder,
  getStructEncoder,
  getU8Decoder,
  getU8Encoder,
  transformEncoder,
  type Address,
  type Codec,
  type Decoder,
  type Encoder,
  type IAccountMeta,
  type IAccountSignerMeta,
  type IInstruction,
  type IInstructionWithAccounts,
  type IInstructionWithData,
  type ReadonlyAccount,
  type TransactionSigner,
  type WritableAccount,
  type WritableSignerAccount,
} from '@solana/kit';
import { COUNTER_PROGRAM_ADDRESS } from '../programs';
import { getAccountMetaFactory, type ResolvedAccount } from '../shared';

export const CREATE_DISCRIMINATOR = 0;

export function getCreateDiscriminatorBytes() {
  return getU8Encoder().encode(CREATE_DISCRIMINATOR);
}

export type CreateInstruction<
  TProgram extends string = typeof COUNTER_PROGRAM_ADDRESS,
  TAccountMaker extends string | IAccountMeta<string> = string,
  TAccountCounter extends string | IAccountMeta<string> = string,
  TAccountSystemProgram extends
    | string
    | IAccountMeta<string> = '11111111111111111111111111111111',
  TRemainingAccounts extends readonly IAccountMeta<string>[] = [],
> = IInstruction<TProgram> &
  IInstructionWithData<Uint8Array> &
  IInstructionWithAccounts<
    [
      TAccountMaker extends string
        ? WritableSignerAccount<TAccountMaker> &
            IAccountSignerMeta<TAccountMaker>
        : TAccountMaker,
      TAccountCounter extends string
        ? WritableAccount<TAccountCounter>
        : TAccountCounter,
      TAccountSystemProgram extends string
        ? ReadonlyAccount<TAccountSystemProgram>
        : TAccountSystemProgram,
      ...TRemainingAccounts,
    ]
  >;

export type CreateInstructionData = { discriminator: number };

export type CreateInstructionDataArgs = {};

export function getCreateInstructionDataEncoder(): Encoder<CreateInstructionDataArgs> {
  return transformEncoder(
    getStructEncoder([['discriminator', getU8Encoder()]]),
    (value) => ({ ...value, discriminator: CREATE_DISCRIMINATOR }),
  );
}

export function getCreateInstructionDataDecoder(): Decoder<CreateInstructionData> {
  return getStructDecoder([['discriminator', getU8Decoder()]]);
}

export function getCreateInstructionDataCodec(): Codec<
  CreateInstructionDataArgs,
  CreateInstructionData
> {
  return combineCodec(
    getCreateInstructionDataEncoder(),
    getCreateInstructionDataDecoder(),
  );
}

export type CreateInput<
  TAccountMaker extends string = string,
  TAccountCounter extends string = string,
  TAccountSystemProgram extends string = string,
> = {
  /** The payer of the counter */
  maker: TransactionSigner<TAccountMaker>;
  /** The counter account */
  counter: Address<TAccountCounter>;
  /** The system program */
  systemProgram?: Address<TAccountSystemProgram>;
};

export function getCreateInstruction<
  TAccountMaker extends string,
  TAccountCounter extends string,
  TAccountSystemProgram extends string,
  TProgramAddress extends Address = typeof COUNTER_PROGRAM_ADDRESS,
>(
  input: CreateInput<TAccountMaker, TAccountCounter, TAccountSystemProgram>,
  config?: { programAddress?: TProgramAddress },
): CreateInstruction<
  TProgramAddress,
  TAccountMaker,
  TAccountCounter,
  TAccountSystemProgram
> {
  // Program address.
  const programAddress = config?.programAddress ?? COUNTER_PROGRAM_ADDRESS;

  // Original accounts.
  const originalAccounts = {
    maker: { value: input.maker ?? null, isWritable: true },
    counter: { value: input.counter ?? null, isWritable: true },
    systemProgram: { value: input.systemProgram ?? null, isWritable: false },
  };
  const accounts = originalAccounts as Record<
    keyof typeof originalAccounts,
    ResolvedAccount
  >;

  // Resolve default values.
  if (!accounts.systemProgram.value) {
    accounts.systemProgram.value =
      '11111111111111111111111111111111' as Address<'11111111111111111111111111111111'>;
  }

  const getAccountMeta = getAccountMetaFactory(programAddress, 'programId');
  const instruction = {
    accounts: [
      getAccountMeta(accounts.maker),
      getAccountMeta(accounts.counter),
      getAccountMeta(accounts.systemProgram),
    ],
    programAddress,
    data: getCreateInstructionDataEncoder().encode({}),
  } as CreateInstruction<
    TProgramAddress,
    TAccountMaker,
    TAccountCounter,
    TAccountSystemProgram
  >;

  return instruction;
}

export type ParsedCreateInstruction<
  TProgram extends string = typeof COUNTER_PROGRAM_ADDRESS,
  TAccountMetas extends readonly IAccountMeta[] = readonly IAccountMeta[],
> = {
  programAddress: Address<TProgram>;
  accounts: {
    /** The payer of the counter */
    maker: TAccountMetas[0];
    /** The counter account */
    counter: TAccountMetas[1];
    /** The system program */
    systemProgram: TAccountMetas[2];
  };
  data: CreateInstructionData;
};

export function parseCreateInstruction<
  TProgram extends string,
  TAccountMetas extends readonly IAccountMeta[],
>(
  instruction: IInstruction<TProgram> &
    IInstructionWithAccounts<TAccountMetas> &
    IInstructionWithData<Uint8Array>,
): ParsedCreateInstruction<TProgram, TAccountMetas> {
  if (instruction.accounts.length < 3) {
    // TODO: Coded error.
    throw new Error('Not enough accounts');
  }
  let accountIndex = 0;
  const getNextAccount = () => {
    const accountMeta = instruction.accounts![accountIndex]!;
    accountIndex += 1;
    return accountMeta;
  };
  return {
    programAddress: instruction.programAddress,
    accounts: {
      maker: getNextAccount(),
      counter: getNextAccount(),
      systemProgram: getNextAccount(),
    },
    data: getCreateInstructionDataDecoder().decode(instruction.data),
  };
}
