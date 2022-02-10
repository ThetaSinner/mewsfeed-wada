use hdk::prelude::holo_hash::*;
use hdk::prelude::*;
use regex::Regex;

#[hdk_entry(id = "mew")]
pub struct Mew(String);

entry_defs![
    PathEntry::entry_def(),
    MewContent::entry_def(),
    FullMew::entry_def()
];


#[derive(Debug, Serialize, Deserialize, SerializedBytes)]
#[serde(rename_all = "camelCase")]
enum MewType {
    Original(MewContent),
    Reply(EntryHash,MewContent),
    ReMew(EntryHash),
    MewMew(EntryHash,MewContent), // QuoteTweet
}

#[hdk_entry(id = "mew_content")]
#[serde(rename_all = "camelCase")]
struct MewContent {
    mew: String, // "Visit this web site ^link by @user about #hashtag to earn $cashtag! Also read this humm earth post ^link (as an HRL link)" 
//   mew_links: Vec<LinkTypes>, // [^links in the mewstring in sequence]
//    mew_images: Vec<EntryHash>, //Vec of image links hashes to retrieve
}
#[hdk_entry(id = "full_mew")]
#[serde(rename_all = "camelCase")]
pub struct FullMew {
  mew_type: MewType,
  mew: Option<MewContent>,
}

const MEWS_PATH_SEGMENT: &str = "mews";
const FOLLOWERS_PATH_SEGMENT: &str = "followers";
const FOLLOWING_PATH_SEGMENT: &str = "following";
// const LICKS_PATH_SEGMENT: &str = "licks";
// const MENTIONS_PATH_SEGMENT: &str = "mentions";
// const IMAGES_PATH_SEGMENT: &str = "images";

fn get_my_mews_base(base_type: &str, ensure: bool) -> ExternResult<EntryHash> {
    let me: AgentPubKey = agent_info()?.agent_latest_pubkey;
    get_mews_base(me.into(), base_type, ensure)
}

fn get_mews_base(agent: AgentPubKeyB64, base_type: &str, ensure: bool) -> ExternResult<EntryHash> {
    let path = Path::from(format!("agent.{}.{}", agent, base_type));
    if ensure {
        path.ensure()?;
    }
    let anchor_hash = path.path_entry_hash()?;
    Ok(anchor_hash)
}

#[derive(Debug, Serialize, Deserialize, SerializedBytes)]
#[serde(rename_all = "camelCase")]
enum MewTypeInput {
    Original,
    Reply(EntryHashB64),
    ReMew(EntryHashB64),
    MewMew(EntryHashB64), // QuoteTweet
}

#[derive(Debug, Serialize, Deserialize, SerializedBytes)]
#[serde(rename_all = "camelCase")]
pub struct CreateMewInput {
  mew_type: MewTypeInput,
  mew: Option<String>,
}

#[hdk_extern]
// TODO: we want a parsing function here to identify user references, tag references, etc to build posts, links, etc
pub fn create_mew(mew: CreateMewInput) -> ExternResult<HeaderHashB64> {
    // TODO: enforce mew type is correct
    // check the type
    let mew_header_hash = match mew.mew_type {
        MewTypeInput::Original => {
            match mew.mew {
                Some(mew_string) => create_original_mew(mew_string)?,
                None => return Err(WasmError::Guest(String::from("mew must contain text")))
            }
        },
        MewTypeInput::Reply(original_entry_hash) => {
            match mew.mew {
                Some(mew_string) => create_reply_mew(mew_string, original_entry_hash)?,
                None => return Err(WasmError::Guest(String::from("reply mew must contain text")))
            }
        },
        MewTypeInput::ReMew(original_entry_hash) => {
            match mew.mew {
                Some(_) => return Err(WasmError::Guest(String::from("remew cannot contain text. Try MewMew-ing."))),
                None => create_remew(original_entry_hash)?
            }
        },
        MewTypeInput::MewMew(original_entry_hash) => {
            match mew.mew {
                Some(mew_string) => create_mewmew(mew_string, original_entry_hash)?,
                None => return Err(WasmError::Guest(String::from("mewmew must contain text")))
            }
        },

    };
    Ok(mew_header_hash)
}

pub fn create_original_mew(mew: String) -> ExternResult<HeaderHashB64> {
    let content = MewContent{mew: mew.clone()};
    let _header_hash = create_entry(&content)?;

    let full = FullMew{
        mew_type: MewType::Original(content),
        mew: None
    };
    let full_header_hash = create_entry(&full)?;
    let hash = hash_entry(&full)?;


    let base = get_my_mews_base(MEWS_PATH_SEGMENT, true)?;

    // TODO: maybe return the link_hh later if we need to delete
    let _link_hh = create_link(base, hash.clone(), ())?;
    parse_mew_text(mew, hash)?;
    Ok(full_header_hash.into())
}

pub fn create_reply_mew(mew: String, original_entry_hash: EntryHashB64) -> ExternResult<HeaderHashB64> {
    let content = MewContent{mew: mew.clone()};
    let _header_hash = create_entry(&content)?;

    let full = FullMew{
        mew_type: MewType::Reply(original_entry_hash.clone().into(), content),
        mew: None
    };
    let full_header_hash = create_entry(&full)?;
    let hash = hash_entry(&full)?;


    let base = get_my_mews_base(MEWS_PATH_SEGMENT, true)?;

    // TODO: maybe return the link_hh later if we need to delete
    let _link_hh = create_link(base, hash.clone(), ())?;
    // link off original entry as comment
    let _reply_link_hh = create_link(original_entry_hash.into(), hash.clone(), LinkTag::new("reply"))?;
    parse_mew_text(mew, hash)?;
    Ok(full_header_hash.into())
}
pub fn create_remew(original_entry_hash: EntryHashB64) -> ExternResult<HeaderHashB64> {
    let full = FullMew{
        mew_type: MewType::ReMew(original_entry_hash.clone().into()),
        mew: None
    };
    let full_header_hash = create_entry(&full)?;
    let hash = hash_entry(&full)?;


    let base = get_my_mews_base(MEWS_PATH_SEGMENT, true)?;

    // TODO: maybe return the link_hh later if we need to delete
    let _link_hh = create_link(base, hash.clone(), ())?;
    // link off original entry as comment
    let _remew_link_hh = create_link(original_entry_hash.into(), hash.clone(), LinkTag::new("remew"))?;
    Ok(full_header_hash.into())
}
pub fn create_mewmew(mew: String, original_entry_hash: EntryHashB64) -> ExternResult<HeaderHashB64> {
    let content = MewContent{mew: mew.clone()};
    let _header_hash = create_entry(&content)?;

    let full = FullMew{
        mew_type: MewType::MewMew(original_entry_hash.clone().into(), content),
        mew: None
    };
    let full_header_hash = create_entry(&full)?;
    let hash = hash_entry(&full)?;


    let base = get_my_mews_base(MEWS_PATH_SEGMENT, true)?;

    // TODO: maybe return the link_hh later if we need to delete
    let _link_hh = create_link(base, hash.clone(), ())?;
    // link off original entry as comment
    let _mewmew_link_hh = create_link(original_entry_hash.into(), hash.clone(), LinkTag::new("remew"))?;
    parse_mew_text(mew, hash)?;
    Ok(full_header_hash.into())
}
// TODO: open question: do we want to allow edits, "deletes"?

#[hdk_extern]
pub fn get_mew(header_hash: HeaderHashB64) -> ExternResult<FullMew> {
    let element = get(HeaderHash::from(header_hash), GetOptions::default())?
        .ok_or(WasmError::Guest(String::from("Mew not found")))?;

    let mew: FullMew = element
        .entry()
        .to_app_option()?
        .ok_or(WasmError::Guest(String::from("Malformed mew")))?;

    Ok(mew)
}
// #[hdk_extern]
// pub fn get_feed_mew_and_context(header_hash: HeaderHashB64) -> ExternResult<FullMew> {
//     let element = get(HeaderHash::from(header_hash), GetOptions::default())?
//         .ok_or(WasmError::Guest(String::from("Mew not found")))?;

//     let mew: FullMew = element
//         .entry()
//         .to_app_option()?
//         .ok_or(WasmError::Guest(String::from("Malformed mew")))?;

//     Ok(mew)
// }

#[derive(Debug, Serialize, Deserialize, SerializedBytes)]
#[serde(rename_all = "camelCase")]
pub struct FeedOptions {
    pub option: String,
}

#[derive(Debug, Serialize, Deserialize, SerializedBytes)]
#[serde(rename_all = "camelCase")]
pub struct FeedMew {
    pub mew: FullMew,
    pub header: Header,
}

#[hdk_extern]
pub fn mews_by(agent: AgentPubKeyB64) -> ExternResult<Vec<FeedMew>> {
    let base = get_mews_base(agent, MEWS_PATH_SEGMENT, false)?;
    let links = get_links(base, None)?;
    let get_input = links
        .into_iter()
        .map(|link| GetInput::new(link.target.into(), GetOptions::default()))
        .collect();

    let mew_elements = HDK.with(|hdk| hdk.borrow().get(get_input))?;
    let feed: Vec<FeedMew> = mew_elements
        .into_iter()
        .filter_map(|me| me)
        .filter_map(|element| match element.entry().to_app_option() {
            Ok(Some(g)) => Some(FeedMew {
                mew: g,
                header: element.header().clone(),
            }),
            _ => None,
        })
        .collect();
    Ok(feed)
}

#[hdk_extern]
pub fn mews_feed(_options: FeedOptions) -> ExternResult<Vec<FeedMew>> {
    let mut feed = Vec::new();
    let me = agent_info()?.agent_latest_pubkey;
    feed.append(&mut mews_by(AgentPubKeyB64::from(me))?);
    for agent in my_following(())?.into_iter() {
        feed.append(&mut mews_by(agent)?);
    }
    // TODO don't really need to sort, could merge for efficiency
    // sort by timestamp in descending order
    feed.sort_by(|a, b| b.header.timestamp().cmp(&a.header.timestamp()));

    Ok(feed)
}

#[hdk_extern]
pub fn follow(agent: AgentPubKeyB64) -> ExternResult<()> {
    let me_target: EntryHash = agent_info()?.agent_latest_pubkey.into();
    let them_target: EntryHash = AgentPubKey::from(agent.clone()).into();

    let me = get_my_mews_base(FOLLOWING_PATH_SEGMENT, true)?;
    let _link_hh = create_link(me, them_target, ())?;

    let them = get_mews_base(agent, FOLLOWERS_PATH_SEGMENT, true)?;
    let _link_hh = create_link(them, me_target, ())?;
    Ok(())
}

#[hdk_extern]
pub fn unfollow(agent: AgentPubKeyB64) -> ExternResult<()> {
    let them_target: EntryHash = AgentPubKey::from(agent.clone()).into();
    let me = get_my_mews_base(FOLLOWING_PATH_SEGMENT, true)?;
    let links = get_links(me.clone(), None)?;
    for link in links {
        if link.target == them_target {
            let _deleted_link_hh = delete_link(link.create_link_hash)?;
            break;
        }
    }
    
    let me_target: EntryHash = agent_info()?.agent_latest_pubkey.into();
    let them = get_mews_base(agent, FOLLOWERS_PATH_SEGMENT, true)?;
    let links = get_links(them.clone(), None)?;
    for link in links {
        if link.target == me_target {
            let _deleted_link_hh = delete_link(link.create_link_hash)?;
            break;
        }
    }

    Ok(())
}

#[hdk_extern]
pub fn my_following(_: ()) -> ExternResult<Vec<AgentPubKeyB64>> {
    let me: AgentPubKeyB64 = agent_info()?.agent_latest_pubkey.into();
    follow_inner(me, FOLLOWING_PATH_SEGMENT)
}
#[hdk_extern]
pub fn my_followers(_: ()) -> ExternResult<Vec<AgentPubKeyB64>> {
    let me: AgentPubKeyB64 = agent_info()?.agent_latest_pubkey.into();
    follow_inner(me, FOLLOWERS_PATH_SEGMENT)
}

fn follow_inner(agent: AgentPubKeyB64, base_type: &str) -> ExternResult<Vec<AgentPubKeyB64>> {
    let base = get_mews_base(agent, base_type, false)?;
    let links = get_links(base, None)?;
    Ok(links
        .into_iter()
        .map(|link| AgentPubKey::from(link.target).into())
        .collect())
}

// get who's following an agent
#[hdk_extern]
pub fn following(agent: AgentPubKeyB64) -> ExternResult<Vec<AgentPubKeyB64>> {
    follow_inner(agent, FOLLOWING_PATH_SEGMENT)
}

/// get whos followers are of an agent
#[hdk_extern]
pub fn followers(agent: AgentPubKeyB64) -> ExternResult<Vec<AgentPubKeyB64>> {
    follow_inner(agent, FOLLOWERS_PATH_SEGMENT)
}

pub fn get_mews_from_path(path: Path) -> ExternResult<Vec<FeedMew>> {
    path.ensure()?;
    let path_hash = path.path_entry_hash()?;

    let links = get_links(path_hash, None)?;
    let get_input = links
        .into_iter()
        .map(|link| GetInput::new(link.target.into(), GetOptions::default()))
        .collect();

    let mew_elements = HDK.with(|hdk| hdk.borrow().get(get_input))?;

    let hashtag_feed: Vec<FeedMew> = mew_elements
        .into_iter()
        .filter_map(|me| me)
        .filter_map(|element| match element.entry().to_app_option() {
            Ok(Some(g)) => Some(FeedMew {
                mew: g,
                header: element.header().clone(),
            }),
            _ => None,
        })
        .collect();
    Ok(hashtag_feed)
}
#[hdk_extern]
pub fn get_mews_with_hashtag(hashtag: String) -> ExternResult<Vec<FeedMew>> {
    let path = Path::from(format!("hashtags.{}", hashtag));
    Ok(get_mews_from_path(path)?)
}
#[hdk_extern]
pub fn get_mews_with_cashtag(hashtag: String) -> ExternResult<Vec<FeedMew>> {
    let path = Path::from(format!("cashtags.{}", hashtag));
    Ok(get_mews_from_path(path)?)
}

pub fn parse_mew_text(mew_text: String, mew_hash: EntryHash) -> ExternResult<()> {
    let hashtag_regex = Regex::new(r"#\w+").unwrap();
    let cashtag_regex = Regex::new(r"\$\w+").unwrap();
    for mat in hashtag_regex.find_iter(&mew_text.clone()) {
        let hashtag = mat.as_str();
        let path = Path::from(format!("hashtags.{}", hashtag));
        path.ensure()?;
        let path_hash = path.path_entry_hash()?;
        let _link_hh = create_link(path_hash, mew_hash.clone(), ())?;
    }
    for mat in cashtag_regex.find_iter(&mew_text.clone()) {
        let cashtag = mat.as_str();
        let path = Path::from(format!("cashtags.{}", cashtag));
        path.ensure()?;
        let path_hash = path.path_entry_hash()?;
        let _link_hh = create_link(path_hash, mew_hash.clone(), ())?;
    }
    Ok(())
}
