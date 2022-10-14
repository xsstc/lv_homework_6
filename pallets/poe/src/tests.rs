use super::*; // 引入Poe模块
use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok, BoundedVec, traits::Get}; // 引入BoundedVec

#[test]
// 测试创建存证，应该操作成功
fn create_claim_works() {
    new_test_ext().execute_with(|| {

        let claim = vec![0, 1];

        // Origin::signed(1)，这里的1，是因为mock.rs文件中定义了type AccountId = u64;
        // 用u64类型的任意一个数字来表示当前用户账号
        assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));

        // 解析存证的值
        let bounded_claim = BoundedVec::<u8, <Test as Config>::MaxClaimLength>::try_from(claim.clone()).unwrap();

        assert_eq!(
            Proofs::<Test>::get(&bounded_claim),
            Some((1, frame_system::Pallet::<Test>::block_number()))
        );
    })
}

#[test]
// 测试创建超过最大长度限制的存证，应该操作失败
fn create_claim_failed_when_claim_too_long() {
    new_test_ext().execute_with(|| {

        let claim: Vec<u8> = vec![0, 1, 2]; //[0, 1]时刚刚好，[0, 1, 2]时就超出最大长度2了，见mock.rs中的配置

        // assert_noop!()宏不会对链上存储进行任何修改
        assert_noop!(
            PoeModule::create_claim(Origin::signed(1), claim.clone()),
            Error::<Test>::ClaimTooLong
        );

        // 获取MaxClaimLength的写法
        // ConstU32实现了Get方法（见frame_support/src/traits/misc.rs），因此可以使用as Get<u32>转成u32
        assert_eq!(<<Test as Config>::MaxClaimLength as Get<u32>>::get(), 2);
    })
}

#[test]
// 测试重复创建同样的存证，应该操作失败
fn create_claim_failed_when_claim_already_exist() {
    new_test_ext().execute_with(|| {

        let claim: Vec<u8> = vec![0, 1];

        // 创建存证
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());

        // assert_noop!()宏不会对链上存储进行任何修改
        assert_noop!(
            PoeModule::create_claim(Origin::signed(1), claim.clone()), // 再次创建同样的存证
            Error::<Test>::ClaimAlreadyExist
        );
    })
}

#[test]
// 测试撤销一个已经存在的存证，应该操作成功
fn revoke_claim_works() {
    new_test_ext().execute_with(|| {

        let claim = vec![0, 1];

        // 创建存证
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());

        // 撤销该存证
        assert_ok!(PoeModule::revoke_claim(Origin::signed(1), claim.clone()));

        // 解析存证的值
        let bounded_claim = BoundedVec::<u8, <Test as Config>::MaxClaimLength>::try_from(claim.clone()).unwrap();

        // 此时存证应该不存在，为None
        assert_eq!(Proofs::<Test>::get(&bounded_claim), None);
        
    })
}

#[test]
// 测试撤销一个不存在的存证，应该操作失败
fn revoke_claim_when_claim_not_exists() {
    new_test_ext().execute_with(|| {

        let claim = vec![0, 1];

        assert_noop!(
            PoeModule::revoke_claim(Origin::signed(1), claim.clone()), // 撤销不存在的存证
            Error::<Test>::ClaimNotExist
        );
        
    })
}

#[test]
// 测试非owner用户撤销一个存证，应该操作失败
fn revoke_claim_when_not_claim_owner() {
    new_test_ext().execute_with(|| {

        let claim = vec![0, 1];

        // 使用用户1创建存证
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());

        assert_noop!(
            PoeModule::revoke_claim(Origin::signed(2), claim.clone()), // 使用用户2撤销存证
            Error::<Test>::NotClaimOwner
        );
        
    })
}

#[test]
// 测试转移一个已经存在的存证，应该操作成功
fn transfer_claim_works() {
    new_test_ext().execute_with(|| {

        let claim = vec![0, 1];

        // 使用用户1创建存证
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());

        // 用户1转移该存证给用户2
        assert_ok!(PoeModule::transfer_claim(Origin::signed(1), claim.clone(), 2));

        // 解析存证的值
        let bounded_claim = BoundedVec::<u8, <Test as Config>::MaxClaimLength>::try_from(claim.clone()).unwrap();

        assert_eq!(
            Proofs::<Test>::get(&bounded_claim),
            Some((2, frame_system::Pallet::<Test>::block_number()))
        );
        
    })
}

#[test]
// 测试转移一个不存在的存证，应该操作失败
fn transfer_claim_when_claim_not_exists() {
    new_test_ext().execute_with(|| {

        let claim = vec![0, 1];

        assert_noop!(
            PoeModule::transfer_claim(Origin::signed(1), claim.clone(), 2), // 用户1转移该存证给用户2
            Error::<Test>::ClaimNotExist
        );
        
    })
}

#[test]
// 测试非owner用户转移一个存在的存证，应该操作失败
fn transfer_claim_when_not_claim_owner() {
    new_test_ext().execute_with(|| {

        let claim = vec![0, 1];

        // 使用用户1创建存证
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());

        assert_noop!(
            PoeModule::transfer_claim(Origin::signed(2), claim.clone(), 3), // 用户2转移该存证给用户3
            Error::<Test>::NotClaimOwner
        );
        
    })
}