use anchor_lang::prelude::*;
use std::collections::HashMap;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod audius_track_upload {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> { 
        Ok(())
    }

    pub fn upload_track<'info>(ctx: Context<UploadTracks>, cid: String) -> Result<()> {
        // TODO: if (no all tracks no init) let mut all_tracks = HashMap::new();
        let track_id = Track::new_track_id().unwrap();
        let new_track = Track { cid };
        ctx.accounts.all_tracks.library.insert(track_id,new_track);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
pub struct UploadTracks<'info> {
    #[account(mut)]
    pub all_tracks: Account<'info, TracksLibrary>
}

#[account]
pub struct TracksLibrary {
    library: HashMap<String, Track>
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
#[derive(Default)]
pub struct Track {
    pub cid: String
    // extra: add metadata type (with title or album art)
}

impl Track {
    pub fn new_track_id() -> Result<String> {
        Ok(String::from("I am unique"))
    }
}
