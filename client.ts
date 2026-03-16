import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Leaderboard } from "./target/types/leaderboard";
import { Keypair } from "@solana/web3.js";

async function main() {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Leaderboard as Program<Leaderboard>;
  const authority = provider.wallet;

  console.log("Authority:", authority.publicKey.toString());

  // Create leaderboard account
  const leaderboardKeypair = Keypair.generate();
  console.log("\n1. Initializing leaderboard...");
  await program.methods
    .initialize("Global Leaderboard")
    .accounts({
      leaderboard: leaderboardKeypair.publicKey,
      authority: authority.publicKey,
    })
    .signers([leaderboardKeypair])
    .rpc();
  console.log("Leaderboard created:", leaderboardKeypair.publicKey.toString());

  // Derive entry PDA
  const [entryPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("entry"),
      leaderboardKeypair.publicKey.toBuffer(),
      authority.publicKey.toBuffer(),
    ],
    program.programId
  );

  console.log("\n2. Submitting score...");
  await program.methods
    .submitScore("Player1", new anchor.BN(9500))
    .accounts({
      leaderboard: leaderboardKeypair.publicKey,
      player: authority.publicKey,
    })
    .rpc();
  console.log("Score submitted!");

  // Read entry
  const entry = await program.account.playerEntry.fetch(entryPda);
  console.log("\n3. Leaderboard entry:");
  console.log("  Player:", entry.playerName);
  console.log("  Score:", entry.score.toString());
  console.log("  Timestamp:", new Date(entry.timestamp.toNumber() * 1000).toISOString());

  // Update score
  console.log("\n4. Updating score to 12000...");
  await program.methods
    .updateScore(new anchor.BN(12000))
    .accounts({
      player: authority.publicKey,
      entry: entryPda,
    })
    .rpc();

  const updated = await program.account.playerEntry.fetch(entryPda);
  console.log("Updated score:", updated.score.toString());
  console.log("\nAll tests passed!");
}

main().catch(console.error);
