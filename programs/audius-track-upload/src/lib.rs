use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod audius_track_upload {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> { 
        Ok(())
    }

    pub fn upload_track<'info>(ctx: Context<TrackLibrary>, cid: String) -> Result<()> {
        // TODO: if (no all tracks no init) let mut all_tracks = HashMap::new();
        let track_id = Track::new_track_id();
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
    pub fn new_track_id() -> String { // should be result
        // TODO: unique id each time
        String::from("unique")
    }
}

