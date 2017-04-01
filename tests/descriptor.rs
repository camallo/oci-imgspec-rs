extern crate oci_imgspec;
extern crate serde_json;

use std::{io, fs};
use oci_imgspec::v1;

#[test]
fn test_descriptor_example1() {
    let f = fs::File::open("tests/fixtures/example-descriptor1.json").expect("Missing fixture");
    let bufrd = io::BufReader::new(f);
    let _desc: v1::Descriptor = serde_json::from_reader(bufrd).unwrap();
}

#[test]
fn test_descriptor_example2() {
    let f = fs::File::open("tests/fixtures/example-descriptor2.json").expect("Missing fixture");
    let bufrd = io::BufReader::new(f);
    let _desc: v1::Descriptor = serde_json::from_reader(bufrd).unwrap();
}
