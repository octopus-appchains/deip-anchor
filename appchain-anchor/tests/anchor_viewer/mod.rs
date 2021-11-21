use appchain_anchor::types::{
    AnchorEventHistory, AnchorSettings, AnchorStatus, AppchainCommitment, AppchainDelegator,
    AppchainNotificationHistory, AppchainSettings, AppchainState, AppchainValidator, IndexRange,
    ProtocolSettings, RewardHistory, StakingHistory, UnbondedStake, ValidatorProfile,
    ValidatorSetInfo, ValidatorSetProcessingStatus, WrappedAppchainToken,
};
use appchain_anchor::AppchainAnchorContract;

use near_sdk::json_types::U64;
use near_sdk::AccountId;
use near_sdk_sim::{view, ContractAccount, UserAccount};

pub fn get_anchor_settings(anchor: &ContractAccount<AppchainAnchorContract>) -> AnchorSettings {
    let view_result = view!(anchor.get_anchor_settings());
    if view_result.is_err() {
        println!("{:#?}", view_result);
    }
    assert!(view_result.is_ok());
    view_result.unwrap_json::<AnchorSettings>()
}

pub fn get_appchain_settings(anchor: &ContractAccount<AppchainAnchorContract>) -> AppchainSettings {
    let view_result = view!(anchor.get_appchain_settings());
    if view_result.is_err() {
        println!("{:#?}", view_result);
    }
    assert!(view_result.is_ok());
    view_result.unwrap_json::<AppchainSettings>()
}

pub fn get_protocol_settings(anchor: &ContractAccount<AppchainAnchorContract>) -> ProtocolSettings {
    let view_result = view!(anchor.get_protocol_settings());
    if view_result.is_err() {
        println!("{:#?}", view_result);
    }
    assert!(view_result.is_ok());
    view_result.unwrap_json::<ProtocolSettings>()
}

pub fn get_wrapped_appchain_token(
    anchor: &ContractAccount<AppchainAnchorContract>,
) -> WrappedAppchainToken {
    let view_result = view!(anchor.get_wrapped_appchain_token());
    if view_result.is_err() {
        println!("{:#?}", view_result);
    }
    assert!(view_result.is_ok());
    view_result.unwrap_json::<WrappedAppchainToken>()
}

pub fn get_appchain_state(anchor: &ContractAccount<AppchainAnchorContract>) -> AppchainState {
    let view_result = view!(anchor.get_appchain_state());
    if view_result.is_err() {
        println!("{:#?}", view_result);
    }
    assert!(view_result.is_ok());
    view_result.unwrap_json::<AppchainState>()
}

pub fn get_anchor_status(anchor: &ContractAccount<AppchainAnchorContract>) -> AnchorStatus {
    let view_result = view!(anchor.get_anchor_status());
    if view_result.is_err() {
        println!("{:#?}", view_result);
    }
    assert!(view_result.is_ok());
    view_result.unwrap_json::<AnchorStatus>()
}

pub fn get_processing_status_of(
    anchor: &ContractAccount<AppchainAnchorContract>,
    index: u64,
) -> ValidatorSetProcessingStatus {
    let view_result = view!(anchor.get_processing_status_of(U64::from(index)));
    if view_result.is_err() {
        println!("{:#?}", view_result);
    }
    assert!(view_result.is_ok());
    view_result.unwrap_json::<ValidatorSetProcessingStatus>()
}

pub fn get_validator_set_info_of(
    anchor: &ContractAccount<AppchainAnchorContract>,
    index: u64,
) -> ValidatorSetInfo {
    let view_result = view!(anchor.get_validator_set_info_of(U64::from(index)));
    if view_result.is_err() {
        println!("{:#?}", view_result);
    }
    assert!(view_result.is_ok());
    view_result.unwrap_json::<ValidatorSetInfo>()
}

pub fn get_index_range_of_anchor_event_history(
    anchor: &ContractAccount<AppchainAnchorContract>,
) -> IndexRange {
    let view_result = view!(anchor.get_index_range_of_anchor_event_history());
    if view_result.is_err() {
        println!("{:#?}", view_result);
    }
    assert!(view_result.is_ok());
    view_result.unwrap_json::<IndexRange>()
}

pub fn get_anchor_event_history(
    anchor: &ContractAccount<AppchainAnchorContract>,
    index: u64,
) -> Option<AnchorEventHistory> {
    let view_result = view!(anchor.get_anchor_event_history(Some(U64::from(index))));
    if view_result.is_err() {
        println!("{:#?}", view_result);
    }
    assert!(view_result.is_ok());
    view_result.unwrap_json::<Option<AnchorEventHistory>>()
}

pub fn get_anchor_event_histories(
    anchor: &ContractAccount<AppchainAnchorContract>,
    start_index: u64,
    max_count: Option<U64>,
) -> Vec<AnchorEventHistory> {
    let view_result = view!(anchor.get_anchor_event_histories(U64::from(start_index), max_count));
    if view_result.is_err() {
        println!("{:#?}", view_result);
    }
    assert!(view_result.is_ok());
    view_result.unwrap_json::<Vec<AnchorEventHistory>>()
}

pub fn get_index_range_of_appchain_notification_history(
    anchor: &ContractAccount<AppchainAnchorContract>,
) -> IndexRange {
    let view_result = view!(anchor.get_index_range_of_appchain_notification_history());
    if view_result.is_err() {
        println!("{:#?}", view_result);
    }
    assert!(view_result.is_ok());
    view_result.unwrap_json::<IndexRange>()
}

pub fn get_appchain_notification_history(
    anchor: &ContractAccount<AppchainAnchorContract>,
    index: u64,
) -> Option<AppchainNotificationHistory> {
    let view_result = view!(anchor.get_appchain_notification_history(Some(U64::from(index))));
    if view_result.is_err() {
        println!("{:#?}", view_result);
    }
    assert!(view_result.is_ok());
    view_result.unwrap_json::<Option<AppchainNotificationHistory>>()
}

pub fn get_appchain_notification_histories(
    anchor: &ContractAccount<AppchainAnchorContract>,
    start_index: u64,
    quantity: Option<U64>,
) -> Vec<AppchainNotificationHistory> {
    let view_result =
        view!(anchor.get_appchain_notification_histories(U64::from(start_index), quantity));
    if view_result.is_err() {
        println!("{:#?}", view_result);
    }
    assert!(view_result.is_ok());
    view_result.unwrap_json::<Vec<AppchainNotificationHistory>>()
}

pub fn get_index_range_of_staking_history(
    anchor: &ContractAccount<AppchainAnchorContract>,
) -> IndexRange {
    let view_result = view!(anchor.get_index_range_of_staking_history());
    if view_result.is_err() {
        println!("{:#?}", view_result);
    }
    assert!(view_result.is_ok());
    view_result.unwrap_json::<IndexRange>()
}

pub fn get_staking_history(
    anchor: &ContractAccount<AppchainAnchorContract>,
    index: u64,
) -> Option<StakingHistory> {
    let view_result = view!(anchor.get_staking_history(Some(U64::from(index))));
    if view_result.is_err() {
        println!("{:#?}", view_result);
    }
    assert!(view_result.is_ok());
    view_result.unwrap_json::<Option<StakingHistory>>()
}

pub fn get_validator_list_of(
    anchor: &ContractAccount<AppchainAnchorContract>,
    index: Option<u64>,
) -> Vec<AppchainValidator> {
    let index = match index {
        Some(index) => Some(U64::from(index)),
        None => None,
    };
    let view_result = view!(anchor.get_validator_list_of(index));
    if view_result.is_err() {
        println!("{:#?}", view_result);
    }
    assert!(view_result.is_ok());
    view_result.unwrap_json::<Vec<AppchainValidator>>()
}

pub fn get_validator_profile(
    anchor: &ContractAccount<AppchainAnchorContract>,
    account_id: &AccountId,
) -> Option<ValidatorProfile> {
    let view_result = view!(anchor.get_validator_profile(account_id.clone()));
    if view_result.is_err() {
        println!("{:#?}", view_result);
    }
    assert!(view_result.is_ok());
    view_result.unwrap_json::<Option<ValidatorProfile>>()
}

pub fn get_validator_profile_by_id_in_appchain(
    anchor: &ContractAccount<AppchainAnchorContract>,
    account_id_in_appchain: &String,
) -> Option<ValidatorProfile> {
    let view_result =
        view!(anchor.get_validator_profile_by_id_in_appchain(account_id_in_appchain.clone()));
    if view_result.is_err() {
        println!("{:#?}", view_result);
    }
    assert!(view_result.is_ok());
    view_result.unwrap_json::<Option<ValidatorProfile>>()
}

pub fn get_delegators_of_validator_in_era(
    anchor: &ContractAccount<AppchainAnchorContract>,
    index: u64,
    validator: &UserAccount,
) -> Vec<AppchainDelegator> {
    let view_result = view!(anchor.get_delegators_of_validator_in_era(
        Some(U64::from(index)),
        validator.valid_account_id().to_string()
    ));
    if view_result.is_err() {
        println!("{:#?}", view_result);
    }
    assert!(view_result.is_ok());
    view_result.unwrap_json::<Vec<AppchainDelegator>>()
}

pub fn get_unbonded_stakes_of(
    anchor: &ContractAccount<AppchainAnchorContract>,
    account: &UserAccount,
) -> Vec<UnbondedStake> {
    let view_result = view!(anchor.get_unbonded_stakes_of(account.valid_account_id().to_string()));
    if view_result.is_err() {
        println!("{:#?}", view_result);
    }
    assert!(view_result.is_ok());
    view_result.unwrap_json::<Vec<UnbondedStake>>()
}

pub fn get_validator_rewards_of(
    anchor: &ContractAccount<AppchainAnchorContract>,
    start_era: u64,
    end_era: u64,
    validator: &UserAccount,
) -> Vec<RewardHistory> {
    let view_result = view!(anchor.get_validator_rewards_of(
        U64::from(start_era),
        U64::from(end_era),
        validator.valid_account_id().to_string()
    ));
    if view_result.is_err() {
        println!("{:#?}", view_result);
    }
    assert!(view_result.is_ok());
    view_result.unwrap_json::<Vec<RewardHistory>>()
}

pub fn get_delegator_rewards_of(
    anchor: &ContractAccount<AppchainAnchorContract>,
    start_era: u64,
    end_era: u64,
    delegator: &UserAccount,
    validator: &UserAccount,
) -> Vec<RewardHistory> {
    let view_result = view!(anchor.get_delegator_rewards_of(
        U64::from(start_era),
        U64::from(end_era),
        delegator.valid_account_id().to_string(),
        validator.valid_account_id().to_string()
    ));
    if view_result.is_err() {
        println!("{:#?}", view_result);
    }
    assert!(view_result.is_ok());
    view_result.unwrap_json::<Vec<RewardHistory>>()
}

pub fn get_latest_commitment_of_appchain(
    anchor: &ContractAccount<AppchainAnchorContract>,
) -> Option<AppchainCommitment> {
    let view_result = view!(anchor.get_latest_commitment_of_appchain());
    if view_result.is_err() {
        println!("{:#?}", view_result);
    }
    assert!(view_result.is_ok());
    view_result.unwrap_json::<Option<AppchainCommitment>>()
}
