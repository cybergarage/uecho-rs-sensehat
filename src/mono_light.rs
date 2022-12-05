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

use std::sync::Arc;
use std::sync::Mutex;

use echonet::protocol::{Esv, Property};
use echonet::util::Bytes;
use echonet::{Device, Node, RequestHandler};

/// 3.3.33 Mono functional lighting (0x0291)
pub struct MonoLight {
    dev: Device,
}

impl MonoLight {
    pub fn new(node: Arc<Mutex<Node>>) -> Arc<Mutex<MonoLight>> {
        let m = Arc::new(Mutex::new(MonoLight {
            dev: Device::new_with_node(0x029101, node),
        }));
        // m.lock().unwrap().dev.set_request_handler(m.clone());
        m
    }

    pub fn start(&mut self) -> bool {
        self.dev.start()
    }

    pub fn stop(&mut self) -> bool {
        self.dev.stop()
    }
}

impl RequestHandler for MonoLight {
    fn property_request_received(&mut self, deoj: u32, esv: Esv, prop: &Property) -> bool {
        // Ignore all messages to other objects in the same node.
        if deoj != self.dev.code() {
            return false;
        }

        match esv {
            Esv::ReadRequest | Esv::NotificationRequest => {
                let prop_code = prop.code();
                match prop_code {
                    0x80 /* Operating status */ => {
                        return true;
                    }
                    0xB0 /* Illuminance level setting */ => {
                        return true;
                    }
                    _ => {
                        return false;
                    }
                }
            }
            Esv::WriteRequest | Esv::WriteReadRequest => {
                let prop_code = prop.code();
                let prop_bytes = prop.data();
                match prop_code {
                    0x80 /* Operating status */ => {
                        let prop_u32 = Bytes::to_u32(prop_bytes);
                        match prop_u32 {
                            0x30 /* On */=> {
                                return true;
                            }
                            0x31 /* Off */=> {
                                return true;
                            }
                            _ => {
                                return false;
                            }
                        }
                    }
                    0xB0 /* Illuminance level setting */ => {
                        return true;
                    }
                    _ => {
                        return false;
                    }
                }
            }
            _ => {}
        }
        true
    }
}
