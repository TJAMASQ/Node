// Copyright (c) 2017-2019, Substratum LLC (https://substratum.net) and/or its affiliates. All rights reserved.
use crate::server_initializer::LoggerInitializerWrapper;
use crate::sub_lib::main_tools::StdStreams;
use std::marker::Send;
use tokio::prelude::Future;

pub trait SocketServer: Send + Future<Item = (), Error = ()> {
    fn name(&self) -> String;
    fn initialize_as_privileged(
        &mut self,
        args: &Vec<String>,
        streams: &mut StdStreams<'_>,
        logger_initializer: &mut Box<dyn LoggerInitializerWrapper>,
    );
    fn initialize_as_unprivileged(&mut self, streams: &mut StdStreams<'_>);
}
