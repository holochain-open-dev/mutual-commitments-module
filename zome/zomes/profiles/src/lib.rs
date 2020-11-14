extern crate profiles;

use hc_utils::WrappedEntryHash;
use hdk3::prelude::*;

// This extension of the zome is only to be able to test the mutual_commitments zome

#[hdk_extern]
pub fn get_my_profile_entry_hash(_: ()) -> ExternResult<WrappedEntryHash> {
    let agent_info = agent_info!()?;
    let agent_address: AnyDhtHash = agent_info.agent_initial_pubkey.clone().into();

    let links = get_links!(agent_address.into(), link_tag("profile")?)?;

    let link = links.into_inner().first().unwrap().clone();

    Ok(WrappedEntryHash(link.target))
}

#[derive(Serialize, Deserialize, SerializedBytes)]
struct StringLinkTag(String);
pub fn link_tag(tag: &str) -> ExternResult<LinkTag> {
    let sb: SerializedBytes = StringLinkTag(tag.into()).try_into()?;
    Ok(LinkTag(sb.bytes().clone()))
}
