// Copyright 2020 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under the MIT license <LICENSE-MIT
// http://opensource.org/licenses/MIT> or the Modified BSD license <LICENSE-BSD
// https://opensource.org/licenses/BSD-3-Clause>, at your option. This file may not be copied,
// modified, or distributed except according to those terms. Please review the Licences for the
// specific language governing permissions and limitations relating to use of the SAFE Network
// Software.

#[macro_use]
extern crate duct;

use anyhow::{anyhow, Result};
use sn_api::fetch::SafeData;
use sn_cmd_test_utilities::util::{
    create_preload_and_get_keys, get_random_nrs_string, parse_dog_output,
    parse_files_put_or_sync_output, safeurl_from,
};

const TEST_FILE: &str = "./testdata/test.md";

#[test]
fn calling_safe_dog_files_container_nrsurl() -> Result<()> {
    let content = cmd!(
        env!("CARGO_BIN_EXE_safe"),
        "files",
        "put",
        TEST_FILE,
        "--json"
    )
    .read()
    .map_err(|e| anyhow!(e.to_string()))?;
    let (container_xorurl, _files_map) = parse_files_put_or_sync_output(&content);

    let nrsurl = format!("safe://{}", get_random_nrs_string());
    let _ = cmd!(
        env!("CARGO_BIN_EXE_safe"),
        "nrs",
        "create",
        &nrsurl,
        "-l",
        &container_xorurl,
    )
    .read()
    .map_err(|e| anyhow!(e.to_string()))?;

    let dog_output = cmd!(env!("CARGO_BIN_EXE_safe"), "dog", &nrsurl, "--json",)
        .read()
        .map_err(|e| anyhow!(e.to_string()))?;

    let (url, mut content): (String, Vec<SafeData>) =
        serde_json::from_str(&dog_output).expect("Failed to parse output of `safe dog` on file");
    assert_eq!(url, nrsurl);

    if let Some(SafeData::FilesContainer { resolved_from, .. }) = content.pop() {
        assert_eq!(resolved_from, container_xorurl);
        Ok(())
    } else {
        panic!("Content retrieved was unexpected: {:?}", content);
    }
}

#[test]
fn calling_safe_dog_files_container_nrsurl_jsoncompact() -> Result<()> {
    let content = cmd!(
        env!("CARGO_BIN_EXE_safe"),
        "files",
        "put",
        TEST_FILE,
        "--output=jsoncompact"
    )
    .read()
    .map_err(|e| anyhow!(e.to_string()))?;
    let (container_xorurl, _files_map) = parse_files_put_or_sync_output(&content);

    let nrsurl = format!("safe://{}", get_random_nrs_string());
    let _ = cmd!(
        env!("CARGO_BIN_EXE_safe"),
        "nrs",
        "create",
        &nrsurl,
        "-l",
        &container_xorurl,
    )
    .read()
    .map_err(|e| anyhow!(e.to_string()))?;

    let dog_output = cmd!(
        env!("CARGO_BIN_EXE_safe"),
        "dog",
        &nrsurl,
        "--output=jsoncompact",
    )
    .read()
    .map_err(|e| anyhow!(e.to_string()))?;

    let (url, mut content): (String, Vec<SafeData>) =
        serde_json::from_str(&dog_output).expect("Failed to parse output of `safe dog`");
    assert_eq!(url, nrsurl);

    if let Some(SafeData::FilesContainer { resolved_from, .. }) = content.pop() {
        assert_eq!(resolved_from, container_xorurl);
        Ok(())
    } else {
        panic!("Content retrieved was unexpected: {:?}", content);
    }
}

#[test]
fn calling_safe_dog_files_container_nrsurl_yaml() -> Result<()> {
    let content = cmd!(
        env!("CARGO_BIN_EXE_safe"),
        "files",
        "put",
        TEST_FILE,
        "--json"
    )
    .read()
    .map_err(|e| anyhow!(e.to_string()))?;
    let (container_xorurl, _files_map) = parse_files_put_or_sync_output(&content);

    let nrsurl = format!("safe://{}", get_random_nrs_string());
    let _ = cmd!(
        env!("CARGO_BIN_EXE_safe"),
        "nrs",
        "create",
        &nrsurl,
        "-l",
        &container_xorurl,
    )
    .read()
    .map_err(|e| anyhow!(e.to_string()))?;

    let dog_output = cmd!(env!("CARGO_BIN_EXE_safe"), "dog", &nrsurl, "--output=yaml",)
        .read()
        .map_err(|e| anyhow!(e.to_string()))?;

    let (url, mut content): (String, Vec<SafeData>) =
        serde_yaml::from_str(&dog_output).expect("Failed to parse output of `safe dog`");
    assert_eq!(url, nrsurl);

    if let Some(SafeData::FilesContainer { resolved_from, .. }) = content.pop() {
        assert_eq!(resolved_from, container_xorurl);
        Ok(())
    } else {
        panic!("Content retrieved was unexpected: {:?}", content);
    }
}

#[test]
fn calling_safe_dog_safekey_nrsurl() -> Result<()> {
    let (safekey_xorurl, _sk) = create_preload_and_get_keys("0")?;

    let nrsurl = format!("safe://{}", get_random_nrs_string());
    let _ = cmd!(
        env!("CARGO_BIN_EXE_safe"),
        "nrs",
        "create",
        &nrsurl,
        "-l",
        &safekey_xorurl,
    )
    .read()
    .map_err(|e| anyhow!(e.to_string()))?;

    let dog_output = cmd!(env!("CARGO_BIN_EXE_safe"), "dog", &nrsurl, "--json",)
        .read()
        .map_err(|e| anyhow!(e.to_string()))?;

    let (url, mut content): (String, Vec<SafeData>) =
        serde_json::from_str(&dog_output).expect("Failed to parse output of `safe dog` on file");
    assert_eq!(url, nrsurl);

    if let Some(SafeData::SafeKey { resolved_from, .. }) = content.pop() {
        assert_eq!(resolved_from, safekey_xorurl);
        Ok(())
    } else {
        Err(anyhow!("Content retrieved was unexpected: {:?}", content))
    }
}

#[test]
fn calling_safe_dog_nrs_url_with_subnames() -> Result<()> {
    let (safekey_xorurl, _sk) = create_preload_and_get_keys("0")?;

    let pub_name = get_random_nrs_string();
    let nrsurl = format!("safe://subname.{}", pub_name);
    let _ = cmd!(
        env!("CARGO_BIN_EXE_safe"),
        "nrs",
        "create",
        &nrsurl,
        "-l",
        &safekey_xorurl,
    )
    .read()
    .map_err(|e| anyhow!(e.to_string()))?;

    // let's check the output with NRS-URL first
    let dog_output = cmd!(env!("CARGO_BIN_EXE_safe"), "dog", &nrsurl, "--json",)
        .read()
        .map_err(|e| anyhow!(e.to_string()))?;

    let (url, safe_data_vec) = parse_dog_output(&dog_output);
    assert_eq!(url, nrsurl);
    let mut safeurl = safeurl_from(&nrsurl)?;
    safeurl.set_sub_names("").map_err(|e| anyhow!(e))?;
    let nrs_map_xorurl = safeurl.to_xorurl_string();

    if let SafeData::NrsMapContainer {
        resolved_from,
        xorurl,
        public_name,
        ..
    } = &safe_data_vec[0]
    {
        assert_eq!(*resolved_from, nrsurl);
        assert_eq!(*xorurl, nrs_map_xorurl);
        assert_eq!(*public_name, Some(pub_name));
    } else {
        panic!("Content retrieved was unexpected: {:?}", safe_data_vec);
    }

    // let's now check the output with its XOR-URL
    let dog_output = cmd!(env!("CARGO_BIN_EXE_safe"), "dog", &nrs_map_xorurl, "--json",)
        .read()
        .map_err(|e| anyhow!(e.to_string()))?;

    let (url, safe_data_vec) = parse_dog_output(&dog_output);
    assert_eq!(url, *nrs_map_xorurl);
    if let SafeData::NrsMapContainer {
        resolved_from,
        xorurl,
        public_name,
        ..
    } = &safe_data_vec[0]
    {
        assert_eq!(*resolved_from, nrs_map_xorurl);
        assert_eq!(*xorurl, nrs_map_xorurl);
        // it doesn't know the public name as it was resolved from a XOR-URL
        assert_eq!(*public_name, None);
        Ok(())
    } else {
        panic!("Content retrieved was unexpected: {:?}", safe_data_vec);
    }
}
