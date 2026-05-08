import * as anchor from "@coral-xyz/anchor";
import BN from "bn.js";
import { test } from "node:test";
import assert from "node:assert/strict";

import type { SolanaDistributionProtocol } from "../target/types/solana_distribution_protocol";

test("creates a stream state account", async () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace
    .solanaDistributionProtocol as anchor.Program<SolanaDistributionProtocol>;
  const funder = provider.wallet.publicKey;
  const recipient = anchor.web3.Keypair.generate().publicKey;
  const tokenMint = anchor.web3.Keypair.generate().publicKey;
  const vaultTokenAccount = anchor.web3.Keypair.generate().publicKey;

  const [vestingState] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("vesting-state"),
      funder.toBuffer(),
      recipient.toBuffer(),
      tokenMint.toBuffer(),
    ],
    program.programId,
  );

  await program.methods
    .createStream({
      distributionKind: { stream: {} },
      totalAmount: new BN(1_000),
      startTime: new BN(1_700_000_000),
      endTime: new BN(1_700_086_400),
      cliffTime: new BN(1_700_000_000),
      authorityRevoker: funder,
      authorityMilestone: funder,
      treasuryReturnAddress: funder,
    })
    .accounts({
      authorityFunder: funder,
      recipient,
      tokenMint,
      vaultTokenAccount,
      vestingState,
      systemProgram: anchor.web3.SystemProgram.programId,
    })
    .rpc();

  const streamState = await program.account.vestingState.fetch(vestingState);

  assert.equal(streamState.totalAmount.toString(), "0");
  assert.equal(streamState.isRevoked, false);
});
