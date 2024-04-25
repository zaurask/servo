/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use std::net::TcpStream;

use serde::Serialize;
use serde_json::{Map, Value};

use crate::actor::{Actor, ActorMessageStatus, ActorRegistry};
use crate::actors::browsing_context::BrowsingContextActor;
use crate::protocol::JsonPacketStream;
use crate::StreamId;

use super::browsing_context::BrowsingContextActorMsg;

#[derive(Serialize)]
pub struct TargetAvailableMsg {
    #[serde(rename = "type")]
    type_: String,
    target: BrowsingContextActorMsg,
    from: String,
}

#[derive(Serialize)]
pub struct ResourceAvailableMsg {
    #[serde(rename = "type")]
    type_: String,
    resources: Vec<serde_json::Value>,
}

#[derive(Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct WatcherResources {
    console_message: bool,
    css_change: bool,
    css_message: bool,
    css_registered_properties: bool,
    document_event: bool,
    cache: bool,
    cookies: bool,
    error_message: bool,
    extension_storage: bool,
    indexed_db: bool,
    local_storage: bool,
    session_storage: bool,
    platform_message: bool,
    network_event: bool,
    network_event_stacktrace: bool,
    reflow: bool,
    stylesheet: bool,
    source: bool,
    threadstate: bool,
    server_sent_event: bool,
    websocket: bool,
    jstracert_trace: bool,
    jstracer_state: bool,
    last_private_context_exit: bool,
}

impl WatcherResources {
    fn new() -> Self {
        Self {
            console_message: true,
            css_change: false,
            css_message: false,
            css_registered_properties: false,
            document_event: false,
            cache: false,
            cookies: false,
            error_message: false,
            extension_storage: false,
            indexed_db: false,
            local_storage: false,
            session_storage: false,
            platform_message: false,
            network_event: false,
            network_event_stacktrace: false,
            reflow: false,
            stylesheet: false,
            source: false,
            threadstate: false,
            server_sent_event: false,
            websocket: false,
            jstracert_trace: false,
            jstracer_state: false,
            last_private_context_exit: false,
        }
    }
}

#[derive(Serialize)]
pub struct WatcherTraits {
    frame: bool,
    process: bool,
    worker: bool,
    service_worker: bool,
    resources: WatcherResources,
}

impl WatcherTraits {
    pub fn new() -> Self {
        Self {
            frame: true,
            process: false,
            worker: false,
            service_worker: false,
            resources: WatcherResources::new(),
        }
    }
}

#[derive(Serialize)]
pub struct WatcherActorMsg {
    actor: String,
    traits: WatcherTraits,
}

pub struct WatcherActor {
    name: String,
    browsing_context_actor: String,
}

impl Actor for WatcherActor {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn handle_message(
        &self,
        registry: &ActorRegistry,
        msg_type: &str,
        msg: &Map<String, Value>,
        stream: &mut TcpStream,
        _id: StreamId,
    ) -> Result<ActorMessageStatus, ()> {
        Ok(match msg_type {
            "watchTargets" => {
                if let Some(target_type) = msg.get("targetType") {
                    println!("watchTargets: {:?}", target_type);
                    let frame = registry
                    .find::<BrowsingContextActor>(&self.browsing_context_actor)
                    .encodable();
                let _ = stream.write_json_packet(&TargetAvailableMsg {
                    type_: "target-available-form".to_string(),
                    target: frame,
                    from: self.name(),
                });
                    ActorMessageStatus::Processed
                } else {
                    ActorMessageStatus::Ignored
                }
            },
            "watchResources" => {
                /* let _ = stream.write_json_packet(&ResourceAvailableMsg {
                    type: "resource-available-form".to_string(),
                    resources:
                }); */
                ActorMessageStatus::Processed
            }
            _ => ActorMessageStatus::Ignored,
        })
    }
}

impl WatcherActor {
    pub(crate) fn new(actors: &mut ActorRegistry, browsing_context_actor: String) -> WatcherActor {
        let name = actors.new_name("watcher");
        WatcherActor { name, browsing_context_actor }
    }

    pub fn encodable(&self) -> WatcherActorMsg {
        WatcherActorMsg {
            actor: self.name(),
            traits: WatcherTraits {
                frame: true,
                process: false,
                worker: false,
                service_worker: false,
                resources: WatcherResources {
                    console_message: true,
                    css_change: false,
                    css_message: false,
                    css_registered_properties: false,
                    document_event: false,
                    cache: false,
                    cookies: false,
                    error_message: false,
                    extension_storage: false,
                    indexed_db: false,
                    local_storage: false,
                    session_storage: false,
                    platform_message: false,
                    network_event: false,
                    network_event_stacktrace: false,
                    reflow: false,
                    stylesheet: false,
                    source: false,
                    threadstate: false,
                    server_sent_event: false,
                    websocket: false,
                    jstracert_trace: false,
                    jstracer_state: false,
                    last_private_context_exit: false,
                },
            },
        }
    }
}
