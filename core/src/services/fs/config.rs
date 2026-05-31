// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

use std::fmt::Debug;

use serde::Deserialize;
use serde::Serialize;

/// config for file system
#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(default)]
#[non_exhaustive]
pub struct FsConfig {
    /// root dir for backend
    pub root: Option<String>,

    /// tmp dir for atomic write
    pub atomic_write_dir: Option<String>,

    /// Skip the per-write `fsync` (`File::sync_all`) performed on writer close.
    ///
    /// opendal normally fsyncs every write for crash durability. When the
    /// caller's source of truth is upstream — e.g. a read-through cache that can
    /// re-fetch a corrupted/partial file — that per-write fsync is pure overhead
    /// and serializes badly behind the filesystem journal under concurrency.
    /// Set this to skip it.
    ///
    /// Atomic *visibility* is unaffected: when `atomic_write_dir` is set the
    /// tempfile + rename still makes each write appear atomically to readers;
    /// only crash *durability* is dropped. Defaults to `false` (fsync on).
    pub disable_write_sync: bool,
}
