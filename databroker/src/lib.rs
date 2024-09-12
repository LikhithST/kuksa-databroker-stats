/********************************************************************************
* Copyright (c) 2022 Contributors to the Eclipse Foundation
*
* See the NOTICE file(s) distributed with this work for additional
* information regarding copyright ownership.
*
* This program and the accompanying materials are made available under the
* terms of the Apache License 2.0 which is available at
* http://www.apache.org/licenses/LICENSE-2.0
*
* SPDX-License-Identifier: Apache-2.0
********************************************************************************/

// use std::collections::HashMap;
use std::collections::HashSet;
// use std::iter::FromIterator;
// use std::pin::Pin;
// use std::string;
use databroker_proto::kuksa::val::v1 as proto;

pub mod authorization;
pub mod broker;
pub mod glob;
pub mod grpc;
pub mod permissions;
pub mod query;
pub mod types;
pub mod vss;

#[cfg(feature = "viss")]
pub mod viss;

use std::fmt::Write;

use tracing::info;
use tracing_subscriber::filter::EnvFilter;

pub fn init_logging() {
    let mut output = String::from("Init logging from RUST_LOG");
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|err| {
        output.write_fmt(format_args!(" ({err})")).unwrap();
        // If no environment variable set, this is the default
        EnvFilter::new("info")
    });
    let _ = tracing_subscriber::fmt::Subscriber::builder()
        .with_env_filter(filter)
        .try_init()
        .is_err();

    info!("{}", output);
}


pub fn get_trace_id(fields: &HashSet<proto::Field>, entry: &proto::DataEntry) -> String {
    // Check if the `MetadataDescription` field is in the set
    let metadata_des = if fields.contains(&proto::Field::MetadataDescription) {
        // Try to get the metadata, and if found, try to get the description
        match &entry.metadata {
            Some(metadata) => match &metadata.description {
                Some(description) => Some(description.clone()), // Clone the description
                None => None,
            },
            None => None,
        }
    } else {
        None
    };

    // Return the description or an empty string if it's not found
    metadata_des.unwrap_or_else(|| "".to_string())
    // String::from("Hello_worls")
}

