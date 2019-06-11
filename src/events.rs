extern crate serenity;

use serenity::{
    model::{
        gateway::Ready,
        id::GuildId,
        voice::VoiceState
    },
    prelude::*,
};

pub struct Handler;

impl EventHandler for Handler {

    fn voice_state_update(&self, _ctx: Context, _: Option<GuildId>, _old: Option<VoiceState>, _new: VoiceState) {
        if _old.is_some() {
            if _new.channel_id == _old.as_ref().unwrap().channel_id {
                println!("Same channel");
                return;
            }
        }
        if _old.is_some() {
            println!("Is some");
        }

    }

    // Set a handler to be called on the `ready` event. This is called when a
    // shard is booted, and a READY payload is sent by Discord. This payload
    // contains data like the current user's guild Ids, current user data,
    // private channels, and more.
    //
    // In this case, just print what the current user's username is.
    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}
