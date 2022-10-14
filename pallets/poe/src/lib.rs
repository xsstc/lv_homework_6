#![cfg_attr(not(feature = "std"), no_std)] /// 必须声明在第一行，前面不能有任何内容 如果依赖库有std标准库，就用标准库，没有就不用即no_std

/// 导出poe模块定义的内容，比如存储单元，给外部使用
pub use pallet::*;

/// 必须引入以下两个宏，才能对poe模块进行单元测试
#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

// 性能测试 
// 2、在lib.rs中引入性能测试宏
#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

// 性能测试 
// 5-1、集成weight
pub mod weights; // 引入weights模块
pub use weights::WeightInfo;

#[frame_support::pallet]
pub mod pallet {
	pub use frame_support::pallet_prelude::*; //包含了get等接口和一些常用的宏
	pub use frame_system::pallet_prelude::*;  //包含了ensure_signed等
	pub use sp_std::prelude::*;  //包含了集合类型
	
	use super::WeightInfo; // 5-1

	/// 定义配置
	/// 模块配置接口Config（在rust中使用trait来定义接口）
	/// Config接口继承自frame_system::Config，这样Config就拥有了系统定义的数据类型，比如BlockNumber，哈希类型Hashing，AccountId
	/// #[pallet::config]为宏
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// 事件
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// 存证内容的最大长度
		/// 因为MaxClaimLength是一个常量，所以这里我们需要用到#[pallet::constant]常量宏，来声明这是一个常量
		#[pallet::constant]
		type MaxClaimLength: Get<u32>;

		// 性能测试 
		// 5-2、集成weight
		// 在trait中定义WeightInfo，通过接口的形式将weight信息传递给可调用函数
		type WeightInfo: WeightInfo;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)] 
	pub struct Pallet<T>(_); // 结构体

	/// 定义存储
	#[pallet::storage]
	/// 定义存证结构
	pub type Proofs<T: Config> = StorageMap<
		_,
		Blake2_128Concat, // 键 哈希算法
		BoundedVec<u8, T::MaxClaimLength>, // 在新版中，不能直接使用Vec，而是使用更安全的BoundedVec
		(T::AccountId, T::BlockNumber), // 值 定义成了一个元组格式
	>;

	// 定义事件
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {

		/// 创建存证 [who, hash]
		ClaimCreated(T::AccountId, Vec<u8>),

		/// 撤销存证 [who, hash]
		ClaimRevoked(T::AccountId, Vec<u8>),

		/// 转移存证 [who, to, hash]
		ClaimTransfered(T::AccountId, T::AccountId, Vec<u8>),
	}

	// 定义Error
	#[pallet::error]
	pub enum Error<T> {
		/// 存证已经存在
		ClaimAlreadyExist,
		/// 存证内容太长
		ClaimTooLong,
		/// 存证不存在
		ClaimNotExist,
		/// 不是存证owner
		NotClaimOwner,
	}

	/// 定义钩子
	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	/// 定义可调用函数
	/// 可调度函数允许用户与pallet交互并调用状态更改
	/// 这些函数具体化为“外部函数”，通常与交易进行比较
	/// Dispatchable 函数必须设置权重，并且必须返回 DispatchResult
	#[pallet::call]
	impl<T: Config> Pallet<T> {

		/// 创建存证
		// 性能测试 
		// 5-3、集成weight
		// 应用weight到可调用函数，读取weight.rs文件定义的weight的数量
		// #[pallet::weight(0)]
		#[pallet::weight(T::WeightInfo::create_claim(claim.len() as u32))]
		/// 加pub后，为公共方法，默认为private
		pub fn create_claim(origin: OriginFor<T>, claim: Vec<u8>) -> DispatchResultWithPostInfo {
			
			// 验签
			let sender = ensure_signed(origin)?;
			
			// 校验数据格式与长度
			let bounded_claim = BoundedVec::<u8, T::MaxClaimLength>::try_from(claim.clone())
								.map_err(|_| Error::<T>::ClaimTooLong)?;
			
			// 检查是否已经存在该存证 
			// 只有条件为true时，才不会报后面的Error，ensure!()相当于solidity中的require()
			ensure!(!Proofs::<T>::contains_key(&bounded_claim), Error::<T>::ClaimAlreadyExist);

			// 插入数据
			Proofs::<T>::insert(
				&bounded_claim, // 键
				(sender.clone(), frame_system::Pallet::<T>::block_number()), // 值，元组格式
			);

			// 触发事件
			Self::deposit_event(Event::ClaimCreated(sender, claim));

			// 返回OK
			Ok(().into())

		}

		/// 撤销存证
		#[pallet::weight(T::WeightInfo::revoke_claim(claim.len() as u32))]
		pub fn revoke_claim(origin: OriginFor<T>, claim: Vec<u8>) -> DispatchResultWithPostInfo {
			
			// 验签
			let sender = ensure_signed(origin)?;
			
			// 校验数据格式与长度
			let bounded_claim = BoundedVec::<u8, T::MaxClaimLength>::try_from(claim.clone())
				.map_err(|_| Error::<T>::ClaimTooLong)?;

			// 检查是否已经存在该存证
			let (owner, _) = Proofs::<T>::get(&bounded_claim).ok_or(Error::<T>::ClaimNotExist)?;

			// 检查当前用户是否为该存证的owner
			ensure!(owner == sender, Error::<T>::NotClaimOwner);

			// 删除存证
			Proofs::<T>::remove(&bounded_claim);
			
			// 触发事件
			Self::deposit_event(Event::ClaimRevoked(sender, claim));

			// 返回OK
			Ok(().into())

		}

		/// 转移存证
		#[pallet::weight(T::WeightInfo::transfer_claim(claim.len() as u32))]
		pub fn transfer_claim(origin: OriginFor<T>, claim: Vec<u8>, dest: T::AccountId) -> DispatchResultWithPostInfo {
			
			// 验签
			let sender = ensure_signed(origin)?;
			
			// 校验数据格式与长度
			let bounded_claim = BoundedVec::<u8, T::MaxClaimLength>::try_from(claim.clone())
				.map_err(|_| Error::<T>::ClaimTooLong)?;

			// 检查是否已经存在该存证
			let (owner, _) = Proofs::<T>::get(&bounded_claim).ok_or(Error::<T>::ClaimNotExist)?;

			// 检查当前用户是否为该存证的owner
			ensure!(owner == sender, Error::<T>::NotClaimOwner);

			// 转移存证
			Proofs::<T>::insert(&bounded_claim, (&dest, frame_system::Pallet::<T>::block_number()));
			
			// 触发事件
			Self::deposit_event(Event::ClaimTransfered(sender, dest, claim));

			// 返回OK
			Ok(().into())

		}

		
	}

}
