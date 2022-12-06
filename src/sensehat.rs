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

use echonet::Node;
use sensehat::SenseHat;
use std::sync::{Arc, Mutex};

pub struct SenseHatNode<'a> {
    node: Arc<Mutex<Node>>,
    sensehat: Option<SenseHat<'a>>,
}

impl SenseHatNode<'_> {
    pub fn new() -> SenseHatNode<'static> {
        let mut node = SenseHatNode {
            node: Node::new(),
            sensehat: None,
        };
        let sensehat = SenseHat::new();
        if sensehat.is_ok() {
            node.sensehat = Some(sensehat.unwrap());
        }
        node
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
