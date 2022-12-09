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

use log::*;
use sensehat::SenseHat;

use echonet::protocol::{Esv, Property};
use echonet::util::Bytes;
use echonet::{Device, Node, Object, RequestHandler};

/// 3.1.18 Humidity sensor class (0x0012)
pub struct Humidity<'a> {
    pub dev: Device,
    sensehat: Arc<Mutex<SenseHat<'a>>>,
}

impl Humidity<'_> {
    pub fn new(
        node: Arc<Mutex<Node>>,
        sensehat: Arc<Mutex<SenseHat<'static>>>,
    ) -> Arc<Mutex<Humidity<'static>>> {
        let m = Arc::new(Mutex::new(Humidity {
            dev: Device::new_with_node(0x001201, node),
            sensehat: sensehat,
        }));
        m.lock().unwrap().dev.set_request_handler(m.clone());
        m
    }
}

impl RequestHandler for Humidity<'_> {
    fn property_request_received(&mut self, deoj: &mut Object, esv: Esv, prop: &Property) -> bool {
        // Ignore all messages to other objects in the same node.
        if deoj.code() != self.dev.code() {
            return false;
        }

        match esv {
            Esv::ReadRequest | Esv::NotificationRequest => {
                let prop_code = prop.code();
                match prop_code {
                    0x80 /* Operating status */ => {
                        // The operating status is already turned on.
                        return true;
                    }
                    0xE0 /* Measured value of relative humidity */ => {
                        // Gets the latest humidity value from Sense HAT.
                        let hum = self.sensehat.lock().unwrap().get_humidity();
                        if hum.is_err() {
                            return false;
                        }
                        let hum = hum.unwrap();
                        info!("Humidity = {}", hum);
                        // Sets the latest humidity value to the destination object.
                        let pval = hum.as_percent() as u8;
                        let mut pbytes: [u8; 1] = [0;1];
                        Bytes::from_u32(pval.into(), &mut pbytes);
                        deoj.set_property_data(prop_code, &pbytes);
                        return true;
                    }
                    _ => {
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
