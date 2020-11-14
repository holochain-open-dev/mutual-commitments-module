use crate::utils;
use hc_utils::{WrappedAgentPubKey, WrappedEntryHash};
use hdk3::prelude::*;

pub fn get_links_from_agent_wrapped(
    agent_pub_key: AgentPubKey,
    link_tag: LinkTag,
) -> ExternResult<Vec<WrappedEntryHash>> {
    let agent_address = utils::pub_key_to_entry_hash(agent_pub_key);

    let links = get_links!(agent_address, link_tag)?;

    let entries = links
        .into_inner()
        .into_iter()
        .map(|link| WrappedEntryHash(link.target))
        .collect();

    Ok(entries)
}

pub fn get_links_from_entry_wrapped(
    entry_hash: EntryHash,
    link_tag: LinkTag,
) -> ExternResult<Vec<WrappedAgentPubKey>> {
    let links = get_links!(entry_hash, link_tag)?;

    let entries = links
        .into_inner()
        .into_iter()
        .map(|link| WrappedAgentPubKey(utils::entry_hash_to_pub_key(link.target)))
        .collect();

    Ok(entries)
}
