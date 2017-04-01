extern crate oci_imgspec;
extern crate serde_json;

use std::{io, fs};
use oci_imgspec::v1;

#[test]
fn test_maniflist_example() {
    let f = fs::File::open("tests/fixtures/example-manifest-list.json").expect("Missing fixture");
    let bufrd = io::BufReader::new(f);
    let _ml: v1::ManifestList = serde_json::from_reader(bufrd).unwrap();
}
