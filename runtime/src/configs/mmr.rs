use crate::*;

impl pallet_mmr::Config for Runtime {
	const INDEXING_PREFIX: &'static [u8] = b"mmr";
	type Hashing = <Runtime as frame_system::Config>::Hashing;
	type Hash = <Runtime as frame_system::Config>::Hash;
	type LeafData = frame_system::Pallet<Self>;
	type OnNewRoot = ();
	type WeightInfo = ();
}
