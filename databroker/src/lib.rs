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
pub mod open_telemetry;

use std::fmt::Write;

use tracing::info;
use tracing_subscriber::filter::EnvFilter;
use open_telemetry::init_trace;
use opentelemetry::global;
use opentelemetry::sdk::propagation::TraceContextPropagator;
use tracing_subscriber::layer::SubscriberExt;


pub fn init_logging() {
    let mut output = String::from("Init logging from RUST_LOG");
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|err| {
        output.write_fmt(format_args!(" ({err})")).unwrap();
        // If no environment variable set, this is the default
        EnvFilter::new("info")
    });

       // Set OpenTelemetry trace propagator
       global::set_text_map_propagator(TraceContextPropagator::new());

       // Initialize OpenTelemetry tracer
       let tracer = init_trace().expect("Failed to initialize tracer");
   
       // Create telemetry layer
       let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    // Set up the tracing subscriber with OpenTelemetry and log formatting
       let subscriber = tracing_subscriber::fmt::Subscriber::builder()
        // .with_env_filter(filter)  // Apply environment filter for log level
        .with_max_level(tracing::Level::INFO)  // You can adjust this log level as needed
        .finish()
        .with(telemetry);  // Add telemetry layer

    // Set the subscriber as the global default for tracing
    tracing::subscriber::set_global_default(subscriber)
        .expect("Unable to install global logging subscriber");

    info!("{}", output);
}


