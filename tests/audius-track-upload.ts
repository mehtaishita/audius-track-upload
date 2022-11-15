import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { AudiusTrackUpload } from "../target/types/audius_track_upload";

describe("audius-track-upload", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.AudiusTrackUpload as Program<AudiusTrackUpload>;

  // rudimentary test
  it("It finishes the transaction!", async () => {
    const keyPair = anchor.web3.Keypair.generate();

    let cid = "QmY9cxiHqTFoWamkQVkpmmqzBrY3hCBEL2XNu3NtX74Fuu"; // example for file 'hello'
    const tx = await program.methods.upload_track(cid).signers(keyPair).rpc();
    console.log("success", tx);
  })
});
