import { expect, describe, beforeAll, test } from "@jest/globals";
import * as anchor from "@coral-xyz/anchor";
import { type Program, BN } from "@coral-xyz/anchor";
import { EscrowPlus } from "../target/types/escrow_plus";
import {
  Connection,
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
  SystemProgram,
  Transaction,
  TransactionInstruction,
} from "@solana/web3.js";
import {
  MINT_SIZE,
  TOKEN_2022_PROGRAM_ID,
  TOKEN_PROGRAM_ID,
  burnChecked,
  createAssociatedTokenAccountIdempotentInstruction,
  createInitializeMint2Instruction,
  createMintToInstruction,
  getAssociatedTokenAddressSync,
  getMinimumBalanceForRentExemptMint,
} from "@solana/spl-token";
import { randomBytes } from "crypto";

import { confirmTransaction, makeKeypairs } from "@solana-developers/helpers";
import { tokenProgramErrors } from "./token_program_error";

const TOKEN_PROGRAM: typeof TOKEN_2022_PROGRAM_ID | typeof TOKEN_PROGRAM_ID = TOKEN_PROGRAM_ID;

export const getRandomBigNumber = (size: number = 8) => {
  return new BN(randomBytes(size));
};

function areBnEqual(a: unknown, b: unknown): boolean | undefined {
  const isABn = a instanceof BN;
  const isBBn = b instanceof BN;

  if (isABn && isBBn) {
    return a.eq(b);
  } else if (isABn === isBBn) {
    return undefined;
  } else {
    return false;
  }
}
expect.addEqualityTesters([areBnEqual]);

const createTokenAndMintTo = async (
  connection: Connection,
  payer: PublicKey,
  tokenMint: PublicKey,
  decimals: number,
  mintAuthority: PublicKey,
  mintTo: Array<{ recepient: PublicKey; amount: number }>
): Promise<Array<TransactionInstruction>> => {
  let minimumLamports = await getMinimumBalanceForRentExemptMint(connection);

  let createTokeIxs = [
    SystemProgram.createAccount({
      fromPubkey: payer,
      newAccountPubkey: tokenMint,
      lamports: minimumLamports,
      space: MINT_SIZE,
      programId: TOKEN_PROGRAM,
    }),
    createInitializeMint2Instruction(
      tokenMint,
      decimals,
      mintAuthority,
      null,
      TOKEN_PROGRAM
    ),
  ];

  let mintToIxs = mintTo.flatMap(({ recepient, amount }) => {
    const ataAddress = getAssociatedTokenAddressSync(
      tokenMint,
      recepient,
      false,
      TOKEN_PROGRAM
    );

    return [
      createAssociatedTokenAccountIdempotentInstruction(
        payer,
        ataAddress,
        recepient,
        tokenMint,
        TOKEN_PROGRAM
      ),
      createMintToInstruction(
        tokenMint,
        ataAddress,
        mintAuthority,
        amount,
        [],
        TOKEN_PROGRAM
      ),
    ];
  });

  return [...createTokeIxs, ...mintToIxs];
};

const getTokenBalanceOn = (
  connection: Connection,
) => async (
  tokenAccountAddress: PublicKey,
): Promise<BN> => {
  const tokenBalance = await connection.getTokenAccountBalance(tokenAccountAddress);
  return new BN(tokenBalance.value.amount);
};

// Jest debug console it too verbose.
// const jestConsole = console;

describe("escrow plus", () => {
  // Use the cluster and the keypair from Anchor.toml
  anchor.setProvider(anchor.AnchorProvider.env());

  const provider = anchor.getProvider();

  // See https://github.com/coral-xyz/anchor/issues/3122
  // const user = (provider.wallet as anchor.Wallet).payer;
  // const payer = user;

  const connection = provider.connection;

  const program = anchor.workspace.EscrowPlus as Program<EscrowPlus>;

  let alice: Keypair,
      bob: Keypair, 
      usdcMint: Keypair, 
      wifMint: Keypair;

  let aliceUsdcAccount: PublicKey, 
      aliceWifAccount: PublicKey, 
      bobUsdcAccount: PublicKey, 
      bobWifAccount: PublicKey;

  // Pick a random ID for the new offer.
  let offerId: BN;

  const decimals = 6;

  beforeAll(async () => {
    global.console = require('console');
  });

  // Creates Alice and Bob accounts, 2 token mints, and associated token
  // accounts for both tokens for both users.
  beforeEach(async () => {
    // global.console = require('console');

    offerId = getRandomBigNumber();
    [alice, bob, usdcMint, wifMint] = makeKeypairs(4);
    [aliceUsdcAccount, aliceWifAccount, bobUsdcAccount, bobWifAccount] = [
      alice,
      bob,
    ].flatMap((owner) =>
      [usdcMint, wifMint].map((tokenMint) =>
        getAssociatedTokenAddressSync(
          tokenMint.publicKey,
          owner.publicKey,
          false,
          TOKEN_PROGRAM
        )
      )
    );

    const giveAliceAndBobSolIxs: Array<TransactionInstruction> = [
      alice,
      bob,
    ].map((owner) =>
      SystemProgram.transfer({
        fromPubkey: provider.publicKey,
        toPubkey: owner.publicKey,
        lamports: 10 * LAMPORTS_PER_SOL,
      })
    );

    const usdcSetupIxs = await createTokenAndMintTo(
      connection,
      provider.publicKey,
      usdcMint.publicKey,
      decimals,
      alice.publicKey,
      [
        { recepient: alice.publicKey, amount: 100_000_000 },
        { recepient: bob.publicKey, amount: 100_000_000 },
      ]
    );

    const wifSetupIxs = await createTokenAndMintTo(
      connection,
      provider.publicKey,
      wifMint.publicKey,
      decimals,
      bob.publicKey,
      [
        { recepient: alice.publicKey, amount: 100_000_000 },
        { recepient: bob.publicKey, amount: 100_000_000 },
      ]
    );

    // Add all these instructions to our transaction
    let tx = new Transaction();
    tx.instructions = [
      ...giveAliceAndBobSolIxs,
      ...usdcSetupIxs,
      ...wifSetupIxs,
    ];

    const _setupTxSig = await provider.sendAndConfirm(tx, [
      alice,
      bob,
      usdcMint,
      wifMint,
    ]);
  });

  const getOfferAddress = (
    maker: PublicKey,
    offerId: BN
  ): PublicKey => {
    const [offerAddress, _offerBump] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("offer_vault"),
        maker.toBuffer(),
        offerId.toArrayLike(Buffer, "le", 8),
      ],
      program.programId
    );

    return offerAddress;
  }

  const makeOfferTx = async (
    maker: Keypair,
    offerId: BN,
    offeredTokenMint: PublicKey,
    offeredAmount: BN,
    wantedTokenMint: PublicKey,
    wantedAmount: BN
  ) => {
    const transactionSignature = await program.methods
      .makeOffer(offerId, offeredAmount, wantedAmount)
      .accounts({
        maker: maker.publicKey,
        tokenMintA: offeredTokenMint,
        tokenMintB: wantedTokenMint,
        tokenProgram: TOKEN_PROGRAM
      })
      .signers([maker])
      .rpc();

    await confirmTransaction(connection, transactionSignature);
  };

  const takeOfferTx = async (
    offerAddress: PublicKey,
    taker: Keypair,
  ): Promise<void> => {
    const transactionSignature = await program.methods
      .takeOffer()
      .accounts({
        taker: taker.publicKey,
        offerVault: offerAddress,
        tokenProgram: TOKEN_PROGRAM,
      })
      .signers([taker])
      .rpc();

    await confirmTransaction(connection, transactionSignature);
  };

  test("Offer created", async () => {
    const offeredUsdc = new BN(10_000_000);
    const wantedWif = new BN(50_000_000);

    const getTokenBalance = getTokenBalanceOn(connection);

    await makeOfferTx(
      alice,
      offerId,
      usdcMint.publicKey,
      offeredUsdc,
      wifMint.publicKey,
      wantedWif
    );

    const offerAddress = getOfferAddress(alice.publicKey, offerId);

    expect(await getTokenBalance(aliceUsdcAccount)).toEqual(new BN(100_000_000));
    expect(await getTokenBalance(aliceWifAccount)).toEqual(new BN(100_000_000));

    // Check our Offer account contains the correct data
    const offerAccount = await program.account.offerVault.fetch(offerAddress);
    expect(offerAccount.maker).toEqual(alice.publicKey);
    expect(offerAccount.id).toEqual(offerId);
    expect(offerAccount.tokenMintA).toEqual(usdcMint.publicKey);
    expect(offerAccount.tokenMintB).toEqual(wifMint.publicKey);
    expect(offerAccount.tokenAGivingAmount).toEqual(offeredUsdc);
    expect(offerAccount.tokenBWantedAmount).toEqual(wantedWif);
  });

  test("Offer taken", async () => {
    const offeredUsdc = new BN(10_000_000);
    const wantedWif = new BN(50_000_000);

    const getTokenBalance = getTokenBalanceOn(connection);

    await makeOfferTx(
      alice,
      offerId,
      usdcMint.publicKey,
      offeredUsdc,
      wifMint.publicKey,
      wantedWif
    );

    const offerAddress = getOfferAddress(alice.publicKey, offerId);

    expect(await getTokenBalance(aliceUsdcAccount)).toEqual(new BN(100_000_000));
    expect(await getTokenBalance(aliceWifAccount)).toEqual(new BN(100_000_000));
    expect(await getTokenBalance(bobUsdcAccount)).toEqual(new BN(100_000_000));
    expect(await getTokenBalance(bobWifAccount)).toEqual(new BN(100_000_000));

    await takeOfferTx(offerAddress, bob);

    expect(await getTokenBalance(aliceUsdcAccount)).toEqual(new BN(90_000_000));
    expect(await getTokenBalance(aliceWifAccount)).toEqual(new BN(150_000_000));

    expect(await getTokenBalance(bobUsdcAccount)).toEqual(new BN(110_000_000));
    expect(await getTokenBalance(bobWifAccount)).toEqual(new BN(50_000_000));
  });

  test("Maker offers too many tokens", async () => {
    const offeredUsdc = new BN(150_000_000);
    const wantedWif = new BN(50_000_000);

    const getTokenBalance = getTokenBalanceOn(connection);

    await expect(makeOfferTx(
      alice,
      offerId,
      usdcMint.publicKey,
      offeredUsdc,
      wifMint.publicKey,
      wantedWif
    )).rejects.toThrow(
      anchor.LangErrorMessage[anchor.LangErrorCode.ConstraintRaw]
    );

  });

  test("Maker doesn't have enough tokens when offer is taken", async () => {
    const offeredUsdc = new BN(80_000_000);
    const wantedWif = new BN(50_000_000);

    const getTokenBalance = getTokenBalanceOn(connection);

    await makeOfferTx(
      alice,
      offerId,
      usdcMint.publicKey,
      offeredUsdc,
      wifMint.publicKey,
      wantedWif
    );

    await burnChecked(
      connection,
      alice,
      aliceUsdcAccount,
      usdcMint.publicKey,
      alice,
      50_000_000,
      decimals
    );

    const offerAddress = getOfferAddress(alice.publicKey, offerId);

    await expect(takeOfferTx(offerAddress, bob)).rejects.toThrow(
      tokenProgramErrors[0x1].toLocaleLowerCase()
    );
  });

  test("Taker doesn't have enough tokens", async () => {
    const offeredUsdc = new BN(50_000_000);
    const wantedWif = new BN(150_000_000);

    const getTokenBalance = getTokenBalanceOn(connection);

    await makeOfferTx(
      alice,
      offerId,
      usdcMint.publicKey,
      offeredUsdc,
      wifMint.publicKey,
      wantedWif
    );

    const offerAddress = getOfferAddress(alice.publicKey, offerId);

    await expect(takeOfferTx(offerAddress, bob)).rejects.toThrow(
      tokenProgramErrors[0x1].toLocaleLowerCase()
    );
  });
});