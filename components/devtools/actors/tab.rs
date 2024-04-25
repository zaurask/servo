/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use std::net::TcpStream;

use serde::Serialize;
use serde_json::{Map, Value};

use super::watcher::WatcherTraits;
use crate::actor::{Actor, ActorMessageStatus, ActorRegistry};
use crate::actors::browsing_context::{BrowsingContextActor, BrowsingContextActorMsg};
use crate::actors::root::RootActor;
use crate::actors::watcher::WatcherActor;
use crate::protocol::JsonPacketStream;
use crate::StreamId;

#[derive(Serialize)]
pub struct TabDescriptorTraits {
    watcher: bool,
    supportsReloadDescriptor: bool,
}

#[derive(Serialize)]
pub struct TabDescriptorActorMsg {
    actor: String,
    title: String,
    url: String,
    outerWindowID: u32,
    browsingContextId: u32,
    browserId: u32,
    selected: bool,
    isZombieTab: bool,
    traits: TabDescriptorTraits,
}

#[derive(Serialize)]
struct GetTargetReply {
    from: String,
    frame: BrowsingContextActorMsg,
}

#[derive(Serialize)]
struct GetFaviconReply {
    from: String,
    favicon: String,
}

#[derive(Serialize)]
struct GetWacherReply {
    from: String,
    actor: String,
    traits: WatcherTraits,
}

pub struct TabDescriptorActor {
    name: String,
    browsing_context_actor: String,
    watcher_actor_name: String,
}

impl Actor for TabDescriptorActor {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn handle_message(
        &self,
        registry: &ActorRegistry,
        msg_type: &str,
        _msg: &Map<String, Value>,
        stream: &mut TcpStream,
        _id: StreamId,
    ) -> Result<ActorMessageStatus, ()> {
        Ok(match msg_type {
            "getTarget" => {
                let frame = registry
                    .find::<BrowsingContextActor>(&self.browsing_context_actor)
                    .encodable();
                let _ = stream.write_json_packet(&GetTargetReply {
                    from: self.name(),
                    frame,
                });
                ActorMessageStatus::Processed
            },
            "getFavicon" => {
                // favicon is not available yet, so we just return an
                // empty one.
                let _ = stream.write_json_packet(&GetFaviconReply {
                    from: self.name(),
                    favicon: String::new(),
                });
                ActorMessageStatus::Processed
            },
            "getWatcher" => {
                let _ = stream.write_json_packet(&GetWacherReply {
                    from: self.name(),
                    actor: self.watcher_actor_name.clone(),
                    traits: WatcherTraits::new(),
                });
                ActorMessageStatus::Processed
            },
            _ => ActorMessageStatus::Ignored,
        })
    }
}

impl TabDescriptorActor {
    pub(crate) fn new(
        actors: &mut ActorRegistry,
        browsing_context_actor: String,
    ) -> TabDescriptorActor {
        let name = actors.new_name("tabDescription");
        // TODO: move watcher actor creation to getWatcher
        // message handling
        let watcher = WatcherActor::new(actors, browsing_context_actor.clone());
        let watcher_actor_name = watcher.name();
        actors.register(Box::new(watcher));

        let root = actors.find_mut::<RootActor>("root");
        root.tabs.push(name.clone());
        TabDescriptorActor {
            name,
            browsing_context_actor,
            watcher_actor_name,
        }
    }

    pub fn encodable(&self, registry: &ActorRegistry, selected: bool) -> TabDescriptorActorMsg {
        let ctx_actor = registry.find::<BrowsingContextActor>(&self.browsing_context_actor);

        let title = ctx_actor.title.borrow().clone();
        let url = ctx_actor.url.borrow().clone();

        TabDescriptorActorMsg {
            title,
            url,
            actor: self.name(),
            browsingContextId: ctx_actor.browsing_context_id.index.0.get(),
            outerWindowID: ctx_actor.active_pipeline.get().index.0.get(),
            browserId: ctx_actor.active_pipeline.get().index.0.get(),
            selected,
            isZombieTab: false,
            traits: TabDescriptorTraits {
                watcher: true,
                supportsReloadDescriptor: false,
            },
        }
    }
}
