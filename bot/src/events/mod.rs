pub mod music;

use lavalink_rs::model::events::Events;


pub fn lava_events() -> Events {
    return Events {
        raw: Some(music::raw_event),
        ready: Some(music::ready_event),
        track_start: Some(music::track_start),
        ..Default::default()
    };
}
