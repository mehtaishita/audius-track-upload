use anchor_lang::prelude::*;
use std::collections::HashMap;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod audius_track_upload {
    use super::*;

    pub fn set_up_library(ctx: Context<TracksLibrary>) -> Result<()> {
        let tracks = &mut ctx.accounts.all_tracks;
        tracks.library = HashMap::new();
        Ok(())
    }

    pub fn upload_track<'info>(ctx: Context<TracksLibrary>, cid: String) -> Result<TrackID> {
        // TODO: check to make sure Tracks hashmap is initialized
        require!(cid.len() >= 46, TrackUploadErrors::TrackAddressTooShort); // ipfs cid v0 are 46 chars long, not sure about v1
        let track_id = Track::new_track_id().unwrap();
        let new_track = Track { cid };
        ctx.accounts.all_tracks.library.insert(track_id, new_track);
        // TODO: set current track id maximum (to generate next id easily)
        Ok(track_id)
    }

    pub fn get_track(ctx: Context<TracksLibrary>, track_id: TrackID) -> Result<String> {
        let lib = &ctx.accounts.all_tracks.library;
        match lib.get(&track_id) {
            Some(track) => Ok(track.cid.to_owned()),
            None => Ok("Track not found!".to_string())
        }
    }
}

// Base context set
#[derive(Accounts)]
pub struct TracksLibrary<'info> {
    #[account(mut)]
    pub all_tracks: Account<'info, Tracks>,
    #[account(mut)]
    pub current_track_id_maximum: Account<'info, TrackID>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// solana program data 
#[account]
pub struct Tracks {
    library: HashMap<TrackID, Track>
}

#[account]
#[derive(PartialOrd, PartialEq, Eq, Hash)]
pub struct TrackID {
    id: u64
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
#[derive(Default)]
pub struct Track {
    pub cid: String
    // TODO: extra: add metadata type (with title or album art)
}

impl Track {
    pub fn new_track_id() -> Result<TrackID> {
        // TODO: let next = Self::get_current_track_id().check_add(1).ok();
        let next = TrackID { id: 1 };
        Ok(next)
    }
}

#[error_code]
pub enum TrackUploadErrors {
    // Error thrown is IPFS cid already exists in the system
    #[msg("Track already exists!")]
    DuplicateTrackUpload,

    // Simple validation rule 
    #[msg("Track IPFS cid invalid")]
    TrackAddressTooShort,
}
