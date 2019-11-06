//  Copyright 2017 Palantir Technologies, Inc.
//
//  Licensed under the Apache License, Version 2.0 (the "License");
//  you may not use this file except in compliance with the License.
//  You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.

//! Span reporters.
use crate::Span;
use log::info;
use std::sync::Arc;

/// A reporter consumes Zipkin spans and reports them.
///
/// For example, the reporter may log the span information to a file, or send
/// it over the network to a collection service.
pub trait Report {
    /// Reports a span.
    fn report(&self, span: Span);
}

impl<T> Report for Arc<T>
where
    T: ?Sized + Report,
{
    fn report(&self, span: Span) {
        (**self).report(span)
    }
}

impl<T> Report for Box<T>
where
    T: ?Sized + Report,
{
    fn report(&self, span: Span) {
        (**self).report(span)
    }
}

/// A `Report`er which does nothing.
pub struct NopReporter;

impl Report for NopReporter {
    fn report(&self, _: Span) {}
}

/// A `Report`er which logs the `Span` at the `info` level.
///
/// The `Span` is simply logged in its `Debug` representation which is not
/// stable, so this reporter is only useful for testing.
pub struct LoggingReporter;

impl Report for LoggingReporter {
    fn report(&self, span: Span) {
        info!("{:?}", span);
    }
}
