import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { AudiusTrackUpload } from "../target/types/audius_track_upload";

describe("audius-track-upload", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.AudiusTrackUpload as Program<AudiusTrackUpload>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
