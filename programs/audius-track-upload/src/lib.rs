use anchor_lang::prelude::*;
use std::collections::HashMap;
use std::rand::distributions::{Alphanumeric, DistString};
use std::rand::{thread_rng};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod audius_track_upload {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> { 
        Ok(())
    }

    pub fn upload_track<'info>(ctx: Context<TrackLibrary>, cid: String) -> Result<()> {
        // TODO: if (no all tracks no init) let mut all_tracks = HashMap::new();
        let track_id = Track::new_track_id().unwrap();
        let new_track = Track { cid };
        ctx.accounts.all_tracks.insert(track_id,new_track);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
pub struct TrackLibrary<'info> {
    #[account(mut)]
    pub all_tracks: HashMap<String,Track>
    
}

#[account]
#[derive(Default)]
pub struct Track {
    pub cid: String
    // extra: add metadata type (with title or album art)
}

impl Track {
    pub fn new_track_id() -> Result<String> {
        // borrowed from rust-random docs/examples
        let id = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);
        Ok(id)
    }
}
