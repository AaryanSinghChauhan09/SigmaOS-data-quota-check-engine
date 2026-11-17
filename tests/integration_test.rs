// Integration test for SigmaOS-data-quota-check-engine
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use sigmaos_data_quota_check_engine as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
