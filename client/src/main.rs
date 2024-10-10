//
// Copyright (c) 2023 ZettaScale Technology
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at
// http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
// which is available at https://www.apache.org/licenses/LICENSE-2.0.
//
// SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
//
// Contributors:
//   ZettaScale Zenoh Team, <zenoh@zettascale.tech>
//
use std::str::FromStr;
use zenoh::prelude::r#async::*;


#[tokio::main]
async fn main() {

    let key_expr = "myhome/kitchen/temp";
    let peers = [EndPoint::from_str("tcp/127.0.0.1:7447").unwrap()];
    let config = config::client(peers);

    println!("Opening session...");
    let session = zenoh::open(config).res().await.unwrap().into_arc();

    println!("Declaring Subscriber on '{}'...", key_expr);
    let subscriber = session.declare_subscriber(key_expr).res()
    .await
    .unwrap();

    let _ = tokio::task::spawn(async move {
        while let Ok(sample) = subscriber.recv_async().await {
            let service_definition = sample.value.to_string();
            println!("{}", service_definition)
        }
    }).await;
    println!("Finished Zenoh subscriber");
}