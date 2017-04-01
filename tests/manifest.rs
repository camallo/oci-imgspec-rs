extern crate oci_imgspec;
extern crate serde_json;

use std::{io, fs};
use oci_imgspec::v1;

#[test]
fn test_manifest_example() {
    let f = fs::File::open("tests/fixtures/example-manifest.json").expect("Missing fixture");
    let bufrd = io::BufReader::new(f);
    let _manif: v1::Manifest = serde_json::from_reader(bufrd).unwrap();
}
