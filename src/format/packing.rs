use std::fmt::Debug;

pub trait SamplePackingType: Debug + Send + Sync {
	fn packing_type() -> SamplePacking;
}

#[derive(Debug)]
pub struct InterleavedPacking;

impl SamplePackingType for InterleavedPacking {
	fn packing_type() -> SamplePacking {
		SamplePacking::Interleaved
	}
}

#[derive(Debug)]
pub struct DeinterleavedPacking;

impl SamplePackingType for DeinterleavedPacking {
	fn packing_type() -> SamplePacking {
		SamplePacking::Deinterleaved
	}
}

pub enum SamplePacking {
	Interleaved,
	Deinterleaved,
}
