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

use echonet::protocol::{Property, ESV};
use echonet::util::Bytes;
use echonet::{Device, Node, Object, RequestHandler};

/// 3.1.17 Temperature sensor class (0x0011)
pub struct Temperature<'a> {
    pub dev: Device,
    sensehat: Arc<Mutex<SenseHat<'a>>>,
}

impl Temperature<'_> {
    pub fn new(
        node: Arc<Mutex<Node>>,
        sensehat: Arc<Mutex<SenseHat<'static>>>,
    ) -> Arc<Mutex<Temperature<'static>>> {
        let m = Arc::new(Mutex::new(Temperature {
            dev: Device::new_with_node(0x001101, node),
            sensehat: sensehat,
        }));
        m.lock().unwrap().dev.set_request_handler(m.clone());
        m
    }
}

impl RequestHandler for Temperature<'_> {
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
                    0xE0 /* Measured temperature value */ => {
                        // Gets the latest temperture value from Sense HAT.
                        let temp = self.sensehat.lock().unwrap().get_temperature_from_pressure();
                        if temp.is_err() {
                            return false;
                        }
                        let temp = temp.unwrap();
                        info!("Temperature = {}", temp);
                        // Sets the latest air pressure value to the destination object.
                        let pval = (temp.as_celsius() * 10.0) as u16;
                        let mut pbytes: [u8; 2] = [0;2];
                        Bytes::from_u32(pval.into(), &mut pbytes);
                        deoj.set_property_data(prop_code, &pbytes);
                        return true;
                    }
                    _ => {
                        // Allows all other read requests
                        return true;
                    }
                }
            }
            _ => {
                // Denies all write requests
                return false;
            }
        }
    }
}
