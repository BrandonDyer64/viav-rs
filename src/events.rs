extern crate serenity;

use::std::collections::HashMap;

use serenity::{
    model::{
        channel::ChannelType,
        gateway::Ready,
        id::{ ChannelId, GuildId, UserId },
        voice::VoiceState,
    },
    prelude::*,
};

use filter_opt;

pub struct Handler;

fn do_join(ctx: &Context, member_count: u8, channel_id: ChannelId, guild_id: GuildId) {
    if member_count > 1 { return; }
    let channel_name = filter_opt!(channel_id.name(&ctx), return);
    let channel = filter_opt_ok!(channel_id.to_channel(&ctx), return);
    let new_channel = filter_opt_ok!(guild_id.create_channel(&ctx.http, &channel_name, ChannelType::Voice, None), return);
}

fn do_leave(ctx: &Context, member_count: u8, channel_id: ChannelId, guild_id: GuildId) -> Option<()> {
    if member_count <= 0 {
        channel_id
            .to_channel(&ctx)?
            .delete(&ctx)?;
        Some(())
    } else {
        None
    }
}

fn do_voice(
    ctx: &Context,
    voice_state: Option<VoiceState>,
    fun: &Fn(&Context, u8, ChannelId, GuildId),
    guild_id: &GuildId
) -> Option<()> {
    let voice_state = voice_state?;
    let channel_id = voice_state.channel_id?;
    let voice_states = voice_state_to_voice_states(&voice_state, &ctx)?;
    let member_count = count_voice_channel_members(&ctx, &voice_states, &channel_id);
    fun(&ctx, member_count, channel_id, *guild_id);
    Some(())
}

fn count_voice_channel_members(ctx: &Context, voice_states: &HashMap<UserId, VoiceState>, channel_id: &ChannelId) -> u8 {
    let mut count = 0;
    for (_, voice_state) in voice_states {
        count += match &voice_state.channel_id {
            Some(_) => 1,
            None => 0,
        };
    }
    count
}

fn voice_state_to_voice_states(voice_state: &VoiceState, ctx: &Context) -> Option<HashMap<UserId, VoiceState>> {
    let channel_id = filter_opt!(&voice_state.channel_id, return None);
    let channel = filter_opt_ok!(channel_id.to_channel(ctx), return None);
    let guild_lock = filter_opt!(channel.guild(), return None);
    let guild_lock = filter_opt!(guild_lock.read().guild(&ctx.cache), return None);
    let voice_states = guild_lock.read().voice_states.clone();
    return Some(voice_states);
}

impl EventHandler for Handler {

    fn voice_state_update(&self, ctx: Context, guild_id: Option<GuildId>, old: Option<VoiceState>, new: VoiceState) {
        let guild_id = filter_opt!(guild_id, return);
        if let Some(old) = &old {
            if new.channel_id == old.channel_id {
                return;
            }
        }
        do_voice(&ctx, old, &do_leave, &guild_id);
        do_voice(&ctx, Some(new), &do_join, &guild_id);
    }

    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}
