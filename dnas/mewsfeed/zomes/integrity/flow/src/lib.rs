pub mod flow;
pub use flow::*;
use hdi::prelude::*;

#[derive(Serialize, Deserialize)]
#[hdk_link_types]
pub enum LinkTypes {
    Setup,
}

#[hdk_entry_helper]
#[derive(Clone, PartialEq, Eq)]
pub enum TestType {
    Reply,
}

#[hdk_extern]
pub fn genesis_self_check(_data: GenesisSelfCheckData) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}

#[allow(unused_variables)]
#[hdk_extern]
pub fn validate(op: Op) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}

// #[derive(Serialize, Deserialize)]
// #[hdk_link_types]
// pub enum LinkTypes {
//     FollowerToCreators,
//     CreatorToFollowers,
// }
// #[hdk_extern]
// pub fn genesis_self_check(_data: GenesisSelfCheckData) -> ExternResult<ValidateCallbackResult> {
//     Ok(ValidateCallbackResult::Valid)
// }
// pub fn validate_agent_joining(
//     _agent_pub_key: AgentPubKey,
//     _membrane_proof: &Option<MembraneProof>,
// ) -> ExternResult<ValidateCallbackResult> {
//     Ok(ValidateCallbackResult::Valid)
// }
// #[allow(unused_variables)]
// #[hdk_extern]
// pub fn validate(op: Op) -> ExternResult<ValidateCallbackResult> {
//     match op.flattened::<(), LinkTypes>()? {
//         FlatOp::StoreEntry(store_entry) => match store_entry {
//             OpEntry::CreateEntry { app_entry, action } => Ok(ValidateCallbackResult::Valid),
//             OpEntry::UpdateEntry {
//                 app_entry, action, ..
//             } => Ok(ValidateCallbackResult::Valid),
//             _ => Ok(ValidateCallbackResult::Valid),
//         },
//         FlatOp::RegisterUpdate(update_entry) => match update_entry {
//             OpUpdate::Entry {
//                 original_action,
//                 original_app_entry,
//                 app_entry,
//                 action,
//             } => Ok(ValidateCallbackResult::Invalid(
//                 "There are no entry types in this integrity zome".to_string(),
//             )),
//             _ => Ok(ValidateCallbackResult::Valid),
//         },
//         FlatOp::RegisterDelete(delete_entry) => match delete_entry {
//             OpDelete::Entry {
//                 original_action,
//                 original_app_entry,
//                 action,
//             } => Ok(ValidateCallbackResult::Valid),
//             _ => Ok(ValidateCallbackResult::Valid),
//         },
//         FlatOp::RegisterCreateLink {
//             link_type,
//             base_address,
//             target_address,
//             tag,
//             action,
//         } => match link_type {
//             LinkTypes::FollowerToCreators => {
//                 validate_create_link_follower_to_creators(action, base_address, target_address, tag)
//             }
//             LinkTypes::CreatorToFollowers => {
//                 validate_create_link_creator_to_followers(action, base_address, target_address, tag)
//             }
//         },
//         FlatOp::RegisterDeleteLink {
//             link_type,
//             base_address,
//             target_address,
//             tag,
//             original_action,
//             action,
//         } => match link_type {
//             LinkTypes::FollowerToCreators => validate_delete_link_follower_to_creators(
//                 action,
//                 original_action,
//                 base_address,
//                 target_address,
//                 tag,
//             ),
//             LinkTypes::CreatorToFollowers => validate_delete_link_creator_to_followers(
//                 action,
//                 original_action,
//                 base_address,
//                 target_address,
//                 tag,
//             ),
//         },
//         FlatOp::StoreRecord(store_record) => match store_record {
//             OpRecord::CreateEntry { app_entry, action } => Ok(ValidateCallbackResult::Valid),
//             OpRecord::UpdateEntry {
//                 original_action_hash,
//                 app_entry,
//                 action,
//                 ..
//             } => Ok(ValidateCallbackResult::Valid),
//             OpRecord::DeleteEntry {
//                 original_action_hash,
//                 action,
//                 ..
//             } => Ok(ValidateCallbackResult::Valid),
//             OpRecord::CreateLink {
//                 base_address,
//                 target_address,
//                 tag,
//                 link_type,
//                 action,
//             } => match link_type {
//                 LinkTypes::FollowerToCreators => validate_create_link_follower_to_creators(
//                     action,
//                     base_address,
//                     target_address,
//                     tag,
//                 ),
//                 LinkTypes::CreatorToFollowers => validate_create_link_creator_to_followers(
//                     action,
//                     base_address,
//                     target_address,
//                     tag,
//                 ),
//             },
//             OpRecord::DeleteLink {
//                 original_action_hash,
//                 base_address,
//                 action,
//             } => {
//                 let record = must_get_valid_record(original_action_hash)?;
//                 let create_link = match record.action() {
//                     Action::CreateLink(create_link) => create_link.clone(),
//                     _ => {
//                         return Ok(ValidateCallbackResult::Invalid(
//                             "The action that a DeleteLink deletes must be a CreateLink".to_string(),
//                         ));
//                     }
//                 };
//                 let link_type =
//                     match LinkTypes::from_type(create_link.zome_index, create_link.link_type)? {
//                         Some(lt) => lt,
//                         None => {
//                             return Ok(ValidateCallbackResult::Valid);
//                         }
//                     };
//                 match link_type {
//                     LinkTypes::FollowerToCreators => validate_delete_link_follower_to_creators(
//                         action,
//                         create_link.clone(),
//                         base_address,
//                         create_link.target_address,
//                         create_link.tag,
//                     ),
//                     LinkTypes::CreatorToFollowers => validate_delete_link_creator_to_followers(
//                         action,
//                         create_link.clone(),
//                         base_address,
//                         create_link.target_address,
//                         create_link.tag,
//                     ),
//                 }
//             }
//             OpRecord::CreatePrivateEntry { .. } => Ok(ValidateCallbackResult::Valid),
//             OpRecord::UpdatePrivateEntry { .. } => Ok(ValidateCallbackResult::Valid),
//             OpRecord::CreateCapClaim { .. } => Ok(ValidateCallbackResult::Valid),
//             OpRecord::CreateCapGrant { .. } => Ok(ValidateCallbackResult::Valid),
//             OpRecord::UpdateCapClaim { .. } => Ok(ValidateCallbackResult::Valid),
//             OpRecord::UpdateCapGrant { .. } => Ok(ValidateCallbackResult::Valid),
//             OpRecord::Dna { .. } => Ok(ValidateCallbackResult::Valid),
//             OpRecord::OpenChain { .. } => Ok(ValidateCallbackResult::Valid),
//             OpRecord::CloseChain { .. } => Ok(ValidateCallbackResult::Valid),
//             OpRecord::InitZomesComplete { .. } => Ok(ValidateCallbackResult::Valid),
//             _ => Ok(ValidateCallbackResult::Valid),
//         },
//         FlatOp::RegisterAgentActivity(agent_activity) => match agent_activity {
//             OpActivity::CreateAgent { agent, action } => {
//                 let previous_action = must_get_action(action.prev_action)?;
//                 match previous_action.action() {
//                         Action::AgentValidationPkg(
//                             AgentValidationPkg { membrane_proof, .. },
//                         ) => validate_agent_joining(agent, membrane_proof),
//                         _ => {
//                             Ok(
//                                 ValidateCallbackResult::Invalid(
//                                     "The previous action for a `CreateAgent` action must be an `AgentValidationPkg`"
//                                         .to_string(),
//                                 ),
//                             )
//                         }
//                     }
//             }
//             _ => Ok(ValidateCallbackResult::Valid),
//         },
//     }
// }
