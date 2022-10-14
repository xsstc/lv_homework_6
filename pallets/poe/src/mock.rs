//mock文件的作用：为单元测试提供环境与初始配置
use crate as pallet_poe; // 声明为pallet_poe
use frame_support::traits::{ConstU16, ConstU32, ConstU64}; // 引入ConstU32
use frame_system as system;
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
        
        // 加载PoeModule 
        // 这里的名称必须与runtime/src/lib.rs中的construct_runtime定义的一致
		PoeModule: pallet_poe::{Pallet, Call, Storage, Event<T>},
	}
);

// 用于Test的runtime配置接口实现
impl system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type Origin = Origin;
	type Call = Call;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64; //在单元测试时，账号可以用数字来表示
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = ConstU64<250>;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ConstU16<42>;
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

// parameter_types! {
// 	pub const MaxClaimLength: u32 = 2; //存证内容最大长度，这里被测试用例使用了
// }

// 为Test实现配置接口
impl pallet_poe::Config for Test {

    // 定义存证类型，最大长度为512
    type MaxClaimLength = ConstU32<2>; // 为了测试方便，最大长度临时改成2

	type Event = Event;

	// 性能测试 
	// 5-5、集成weight
	type WeightInfo = ();
}

// 根据mock runtime构建创世存储
// 用于测试的帮助方法，对区块的初始状态进行配置
pub fn new_test_ext() -> sp_io::TestExternalities {
	system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
}
