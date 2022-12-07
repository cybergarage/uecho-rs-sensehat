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

use echonet::{Device, Node};
use sensehat::SenseHat;
use std::sync::{Arc, Mutex};

use crate::air_pressure::AirPressure;

pub struct SenseHatNode<'a> {
    node: Arc<Mutex<Node>>,
    sensehat: Arc<Mutex<SenseHat<'a>>>,
    air: Arc<Mutex<AirPressure<'a>>>,
}

impl SenseHatNode<'_> {
    pub fn new() -> SenseHatNode<'static> {
        let node = Node::new();
        let sensehat = SenseHat::new();
        let sensehat = Arc::new(Mutex::new(sensehat.unwrap()));
        let air = AirPressure::new(node.clone(), sensehat.clone());
        SenseHatNode {
            node: node,
            sensehat: sensehat,
            air: air,
        }
    }

    pub fn start(&mut self) -> bool {
        let mut node = self.node.lock().unwrap();
        if !node.start() {
            return false;
        }
        true
    }

    pub fn stop(&mut self) -> bool {
        let mut node = self.node.lock().unwrap();
        if !node.stop() {
            return false;
        }
        true
    }
}
