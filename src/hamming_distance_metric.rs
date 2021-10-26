use std::ffi::OsString;

use bk_tree::Metric;
use triple_accel::hamming::hamming_simd_parallel;

pub(crate) struct HammingDistance;

impl Metric<(OsString, [u8; 8])> for HammingDistance {
    fn distance(&self, a: &(OsString, [u8; 8]), b: &(OsString, [u8; 8])) -> u32 {
        hamming_simd_parallel(&a.1, &b.1)
    }

    fn threshold_distance(
        &self,
        a: &(OsString, [u8; 8]),
        b: &(OsString, [u8; 8]),
        _threshold: u32,
    ) -> Option<u32> {
        Some(self.distance(a, b))
    }
}
