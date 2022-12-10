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

use std::env;
use std::io::Error;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::{thread, time};

use echonet::log::Logger;

use echonet_sensehat::SenseHatNode;

fn main() -> Result<(), Error> {
    for arg in env::args() {
        print!("{}", arg);
        match arg.as_str() {
            "-v" => {
                Logger::init();
            }
            &_ => {}
        }
    }

    let mut node = SenseHatNode::new();

    node.start();

    let term = Arc::new(AtomicBool::new(false));
    signal_hook::flag::register(signal_hook::consts::SIGTERM, Arc::clone(&term))?;
    while !term.load(Ordering::Relaxed) {
        thread::sleep(time::Duration::from_secs(1));
    }

    node.stop();

    Ok(())
}
