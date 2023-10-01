use chrono::{Utc, NaiveDateTime, DateTime};
use flow_integrity::*;
use hdk::prelude::*;
use regex_lite::Regex;

// const RE_MENTION: &'static str = r"@\S+";
// const RE_CASH_TAG: &'static str = r"\$([0-9]+)([A-Z]+)";
// const RE_BETWEEN: &'static str = r"between ?([0-9]{4}\/[0-9]{2}\/[0-9]{2})-([0-9]{4}\/[0-9]{2}\/[0-9]{2})";

#[hdk_extern]
pub fn prompt_for_next_flow(msg: String) -> ExternResult<Vec<String>> {
    if msg.is_empty() {
        info!("Skipping empty input input");
        return Ok(vec![]);
    }

    if !msg.starts_with("%") {
        return Err(wasm_error!("First msg must start with %"));
    }

    let request_prefix = Regex::new(r"^%Request:.*").unwrap();
    let promise_prefix = Regex::new(r"^%Promise:.*").unwrap();
    let thanks_prefix = Regex::new(r"^%Thanks:.*").unwrap();

    if request_prefix.is_match(msg.as_str()) {
        prompt_request_msg(msg.as_str())
    } else if promise_prefix.is_match(msg.as_str()) {
        prompt_promise_msg(msg.as_str())
    } else if thanks_prefix.is_match(msg.as_str()) {
        // Nothing to prompt
        Ok(vec![])
    } else {
        Ok(vec![
            "%Request:".to_string(),
            "%Promise:".to_string(),
            "%Thanks:".to_string(),
        ])
    }
}

// Expects input starting with "%Request:"
fn prompt_request_msg(msg: &str) -> ExternResult<Vec<String>> {
    // Right at the start, can either prompt
    let request_start = Regex::new(r"%Request: ?").unwrap();
    if request_start.is_match(msg) {
        return Ok(vec![
            "@username".to_string(),
            "$0.1BTC".to_string(),
            make_between_prompt()?,
        ]);
    }

    let request_with_mention = Regex::new(r"%Request: ?(@\S+)").unwrap();
    if request_with_mention.is_match(msg) {
        return Ok(vec![
            "$0.1BTC".to_string(),
            make_between_prompt()?,
        ]);
    }

    let request_with_mention_and_cash_tag = Regex::new(r"%Request: ?(@\S+) \$([0-9]+)([A-Z]+) ?").unwrap();
    if request_with_mention_and_cash_tag.is_match(msg) {
        return Ok(vec![
            make_between_prompt()?,
        ]);
    }

    let request_skipped_mention_with_cash_tag = Regex::new(r"%Request: \$([0-9]+)([A-Z]+) ?").unwrap();
    if request_skipped_mention_with_cash_tag.is_match(msg) {
        return Ok(vec![
            make_between_prompt()?,
        ]);
    }

    let request_with_between_no_msg = Regex::new(r"%Request: ?(@\S+)? (\$([0-9\.]+)([A-Z]+))? ?between ?([0-9]{4}\/[0-9]{2}\/[0-9]{2})-([0-9]{4}\/[0-9]{2}\/[0-9]{2}) ?").unwrap();
    if request_with_between_no_msg.is_match(msg) {
        return Ok(vec![
            "Make me a coffee".to_string(),
        ]);
    }

    Ok(vec![])
}

// Expects input starting with "%Promise:"
fn prompt_promise_msg(msg: &str) -> ExternResult<Vec<String>> {
    // Right at the start, can either prompt
    let promise_start = Regex::new(r"%Promise: ?").unwrap();
    if promise_start.is_match(msg) {
        return Ok(vec![
            "@username".to_string(),
        ]);
    }

    let promise_with_mention = Regex::new(r"%Promise: ?(@\S+ ?").unwrap();
    if promise_with_mention.is_match(msg) {
        return Ok(vec![
            "$0.1BTC".to_string(),
            "Make me a coffee".to_string(),
        ]);
    }

    let promise_with_mention_and_cash_tag = Regex::new(r"%Promise: ?(@\S+) \$([0-9]+)([A-Z]+) ?").unwrap();
    if promise_with_mention_and_cash_tag.is_match(msg) {
        return Ok(vec![
            "Make me a coffee".to_string(),
        ]);
    }

    Ok(vec![])
}

fn make_between_prompt() -> ExternResult<String> {
    let now = sys_time()?;
    let later = now.checked_add(&std::time::Duration::from_secs(60 * 60 * 24 * 5)).ok_or(wasm_error!("failed to add duration"))?;

    let now_dt = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp_opt(now.0, 0).unwrap(), Utc);
    let later_dt = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp_opt(later.0, 0).unwrap(), Utc);

    Ok(format!("between {}-{}", now_dt.format("%Y/%m/%d").to_string(), later_dt.format("%Y/%m/%d").to_string()))
}

#[hdk_extern]
pub fn process_thread(thread: Vec<String>) -> ExternResult<()> {

    // let request_format = Regex::new(r"^%Request: ?(@\S+)? (\$([0-9\.]+)([A-Z]+))? ?between ?([0-9]{4}\/[0-9]{2}\/[0-9]{2})-([0-9]{4}\/[0-9]{2}\/[0-9]{2}) ?(.*)").unwrap();

    Ok(())
}

// #[hdk_extern]
// pub fn add_creator_for_follower(input: AddCreatorForFollowerInput) -> ExternResult<()> {
//     create_link(
//         input.base_follower.clone(),
//         input.target_creator.clone(),
//         LinkTypes::FollowerToCreators,
//         (),
//     )?;
//     create_link(
//         input.target_creator,
//         input.base_follower,
//         LinkTypes::CreatorToFollowers,
//         (),
//     )?;

//     Ok(())
// }

// #[hdk_extern]
// pub fn get_creators_for_follower(
//     input: GetCreatorsForFollowerInput,
// ) -> ExternResult<Vec<AgentPubKey>> {
//     let links = get_links(input.follower, LinkTypes::FollowerToCreators, None)?;
//     let links_page = paginate_by_agentpubkey(links, input.page)?;

//     let agents: Vec<AgentPubKey> = links_page
//         .into_iter()
//         .filter_map(|link| EntryHash::try_from(link.target).ok())
//         .map(AgentPubKey::from)
//         .collect();

//     Ok(agents)
// }

// #[hdk_extern]
// pub fn get_followers_for_creator(
//     input: GetFollowersForCreatorInput,
// ) -> ExternResult<Vec<AgentPubKey>> {
//     let links = get_follower_links_for_creator(input)?;

//     let agents: Vec<AgentPubKey> = links
//         .into_iter()
//         .filter_map(|link| EntryHash::try_from(link.target).ok())
//         .map(AgentPubKey::from)
//         .collect();

//     Ok(agents)
// }

// #[hdk_extern]
// pub fn count_creators_for_follower(follower: AgentPubKey) -> ExternResult<usize> {
//     let query = LinkQuery::new(
//         follower,
//         LinkTypeFilter::single_type(
//             ZomeIndex(2),
//             LinkType(0), // LinkTypes::FollowerToCreators
//         ),
//     );
//     count_links(query)
// }

// #[hdk_extern]
// pub fn count_followers_for_creator(creator: AgentPubKey) -> ExternResult<usize> {
//     let query = LinkQuery::new(
//         creator,
//         LinkTypeFilter::single_type(
//             ZomeIndex(2),
//             LinkType(1), // LinkTypes::CreatorToFollowers
//         ),
//     );
//     count_links(query)
// }

// #[hdk_extern]
// pub fn get_follower_links_for_creator(
//     input: GetFollowersForCreatorInput,
// ) -> ExternResult<Vec<Link>> {
//     let mut links = get_links(input.creator, LinkTypes::CreatorToFollowers, None)?;
//     links.dedup_by_key(|l| l.target.clone());
//     let links_page = paginate_by_agentpubkey(links, input.page)?;

//     Ok(links_page)
// }

// #[hdk_extern]
// pub fn get_follower_link_details_for_creator(creator: AgentPubKey) -> ExternResult<LinkDetails> {
//     let links = get_link_details(creator, LinkTypes::CreatorToFollowers, None)?;

//     Ok(links)
// }

// #[hdk_extern]
// pub fn remove_creator_for_follower(input: RemoveCreatorForFollowerInput) -> ExternResult<()> {
//     let links = get_links(
//         input.base_follower.clone(),
//         LinkTypes::FollowerToCreators,
//         None,
//     )?;

//     for link in links {
//         let entry_hash =
//             EntryHash::try_from(link.target.clone()).map_err(|err| wasm_error!(err))?;
//         if AgentPubKey::from(entry_hash).eq(&input.target_creator) {
//             delete_link(link.create_link_hash)?;
//         }
//     }

//     let links = get_links(
//         input.target_creator.clone(),
//         LinkTypes::CreatorToFollowers,
//         None,
//     )?;

//     for link in links {
//         let entry_hash =
//             EntryHash::try_from(link.target.clone()).map_err(|err| wasm_error!(err))?;
//         if AgentPubKey::from(entry_hash).eq(&input.base_follower) {
//             delete_link(link.create_link_hash)?;
//         }
//     }

//     Ok(())
// }

// #[hdk_extern]
// pub fn follow(agent: AgentPubKey) -> ExternResult<()> {
//     add_creator_for_follower(AddCreatorForFollowerInput {
//         base_follower: agent_info()?.agent_initial_pubkey,
//         target_creator: agent,
//     })
// }

// #[hdk_extern]
// pub fn unfollow(agent: AgentPubKey) -> ExternResult<()> {
//     remove_creator_for_follower(RemoveCreatorForFollowerInput {
//         base_follower: agent_info()?.agent_initial_pubkey,
//         target_creator: agent,
//     })
// }
