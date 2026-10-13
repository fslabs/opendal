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

#![cfg(not(target_arch = "wasm32"))]

use opendal_core::Operator;
use opendal_service_fs::Fs;

fn operator(root: &std::path::Path, deferred: bool) -> Operator {
    Operator::new(
        Fs::default()
            .root(root.to_str().unwrap())
            .defer_sync(deferred),
    )
    .unwrap()
}

#[tokio::test]
async fn create_only_never_replaces_an_existing_object() {
    for deferred in [false, true] {
        for append in [false, true] {
            let tmp = tempfile::tempdir().unwrap();
            let op = operator(tmp.path(), deferred);
            op.write_with("object", b"original".to_vec())
                .if_not_exists(true)
                .append(append)
                .await
                .unwrap();
            let error = op
                .write_with("object", b"replacement".to_vec())
                .if_not_exists(true)
                .append(append)
                .await
                .unwrap_err();
            assert_eq!(error.kind(), opendal_core::ErrorKind::ConditionNotMatch);
            assert_eq!(
                std::fs::read(tmp.path().join("object")).unwrap(),
                b"original"
            );
        }
    }
}

#[tokio::test]
async fn streaming_writes_preserve_all_chunks_and_metadata() {
    for deferred in [false, true] {
        for append in [false, true] {
            let tmp = tempfile::tempdir().unwrap();
            let op = operator(tmp.path(), deferred);
            let mut writer = op
                .writer_with("directory/object")
                .append(append)
                .if_not_exists(true)
                .await
                .unwrap();
            writer.write(b"first".to_vec()).await.unwrap();
            writer.write(b"second".to_vec()).await.unwrap();
            let metadata = writer.close().await.unwrap();
            assert_eq!(metadata.content_length(), 11);
            assert!(metadata.last_modified().is_some());
            assert_eq!(
                op.read("directory/object").await.unwrap().to_vec(),
                b"firstsecond"
            );
        }
    }
}

#[tokio::test]
async fn deferred_writes_keep_the_storage_root_boundary() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path().join("root");
    let op = operator(&root, true);
    assert!(op.write("../escaped", b"invalid".to_vec()).await.is_err());
    assert!(!tmp.path().join("escaped").exists());
}

#[tokio::test]
async fn atomic_write_directory_remains_supported() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path().join("root");
    let staging = tmp.path().join("staging");
    let op = Operator::new(
        Fs::default()
            .root(root.to_str().unwrap())
            .atomic_write_dir(staging.to_str().unwrap())
            .defer_sync(true),
    )
    .unwrap();
    op.write_with("object", b"complete".to_vec())
        .if_not_exists(true)
        .await
        .unwrap();
    assert_eq!(std::fs::read(root.join("object")).unwrap(), b"complete");
    assert_eq!(std::fs::read_dir(staging).unwrap().count(), 0);
}

#[test]
fn durability_deferral_is_disabled_by_default() {
    assert!(!opendal_service_fs::FsConfig::default().defer_sync);
}

#[tokio::test]
async fn write_failure_reports_the_object_path_and_os_error() {
    let tmp = tempfile::tempdir().unwrap();
    let op = operator(tmp.path(), true);
    std::fs::write(tmp.path().join("blocked"), b"file").unwrap();
    let error = op
        .write("blocked/object", b"data".to_vec())
        .await
        .unwrap_err();
    let message = error.to_string();
    assert!(message.contains("blocked/object"), "{message}");
    assert!(message.contains("os error"), "{message}");
}
