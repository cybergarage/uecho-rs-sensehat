// Copyright (C) 2022 Satoshi Konno All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use sensehat::{Colour, SenseHat};
use std::sync::Arc;
use std::sync::Mutex;

use echonet::protocol::{Property, ESV};
use echonet::util::Bytes;
use echonet::{Device, Node, Object, RequestHandler};

/// 3.3.33 Mono functional lighting (0x0291)
pub struct MonoLight<'a> {
    pub dev: Device,
    sensehat: Arc<Mutex<SenseHat<'a>>>,
}

impl MonoLight<'_> {
    pub fn new(
        node: Arc<Mutex<Node>>,
        sensehat: Arc<Mutex<SenseHat<'static>>>,
    ) -> Arc<Mutex<MonoLight<'static>>> {
        let m = Arc::new(Mutex::new(MonoLight {
            dev: Device::new_with_node(0x029101, node),
            sensehat: sensehat,
        }));
        m.lock().unwrap().dev.set_request_handler(m.clone());
        m.lock().unwrap().sensehat.lock().unwrap().clear().unwrap();
        m
    }
}

impl Drop for MonoLight<'_> {
    fn drop(&mut self) {
        self.sensehat.lock().unwrap().clear().unwrap();
    }
}

impl RequestHandler for MonoLight<'_> {
    fn property_request_received(&mut self, deoj: &mut Object, esv: ESV, prop: &Property) -> bool {
        // Ignore all messages to other objects in the same node.
        if deoj.code() != self.dev.code() {
            return false;
        }

        match esv {
            ESV::ReadRequest | ESV::NotificationRequest => {
                let prop_code = prop.code();
                match prop_code {
                    0x80 /* Operating status */ => {
                        // The operating status is already turned on.
                        return true;
                    }
                    0xB0 /* Illuminance level setting */ => {
                        return true;
                    }
                    _ => {
                        // Allows all other read requests
                        return true;
                    }
                }
            }
            ESV::WriteRequest | ESV::WriteReadRequest => {
                let prop_code = prop.code();
                let prop_bytes = prop.data();
                match prop_code {
                    0x80 /* Operating status */ => {
                        let prop_u32 = Bytes::to_u32(prop_bytes);
                        match prop_u32 {
                            0x30 /* On */=> {
                                self.sensehat.lock().unwrap().text("ON", Colour::WHITE, Colour::WHITE).unwrap();
                                return true;
                            }
                            0x31 /* Off */=> {
                                self.sensehat.lock().unwrap().clear().unwrap();
                                return true;
                            }
                            _ => {
                                return false;
                            }
                        }
                    }
                    0xB0 /* Illuminance level setting */ => {
                        return false;
                    }
                    _ => {
                // Denies all other write requests
                return false;
                    }
                }
            }
            _ => {
                return false;
            }
        }
    }
}
