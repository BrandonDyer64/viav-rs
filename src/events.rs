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

fn on_join(_ctx: Context, )

impl EventHandler for Handler {

    fn voice_state_update(&self, ctx: Context, _: Option<GuildId>, _old: Option<VoiceState>, new: VoiceState) {
        println!("Voice happened");
        if let Some(old) = &_old {
            if new.channel_id == old.channel_id {
                println!("Same channel");
                return;
            } else {
            }
        }
        if _old.is_some() {
            println!("Is some");
        }

    }
    
    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}
