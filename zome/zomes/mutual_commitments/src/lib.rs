use hc_utils::{WrappedAgentPubKey, WrappedEntryHash};
use hdk3::prelude::link::Link;
use hdk3::prelude::*;

mod utils;

pub fn error<T>(reason: &str) -> ExternResult<T> {
    Err(HdkError::Wasm(WasmError::Zome(String::from(reason))))
}

/** Invitations and commitments **/

#[hdk_extern]
pub fn who_am_i(_: ()) -> ExternResult<WrappedAgentPubKey> {
    let agent_info = agent_info!()?;

    Ok(WrappedAgentPubKey(agent_info.agent_initial_pubkey))
}

#[derive(Clone, Serialize, Deserialize, SerializedBytes)]
pub struct InviteAgentsToCommit {
    entry_hash: WrappedEntryHash,
    invited_agents: Vec<WrappedAgentPubKey>,
}
#[hdk_extern]
pub fn invite_agents_to_commit(
    invite_agents_to_commit_input: InviteAgentsToCommit,
) -> ExternResult<()> {
    for invited_agent in invite_agents_to_commit_input.invited_agents {
        create_link!(
            pub_key_to_entry_hash(invited_agent.0.clone()),
            invite_agents_to_commit_input.entry_hash.0.clone(),
            utils::link_tag("is_invited_to")?
        )?;
        create_link!(
            invite_agents_to_commit_input.entry_hash.0.clone(),
            pub_key_to_entry_hash(invited_agent.0.clone()),
            utils::link_tag("has_invitee")?
        )?;
    }

    Ok(())
}

#[hdk_extern]
pub fn accept_invitation_and_commit(entry_hash: WrappedEntryHash) -> ExternResult<()> {
    let agent_info = agent_info!()?;

    delete_my_invitations_to(entry_hash.0.clone())?;

    let my_address = pub_key_to_entry_hash(agent_info.agent_initial_pubkey);

    create_link!(
        my_address.clone(),
        entry_hash.0.clone(),
        utils::link_tag("has_committed_to")?
    )?;
    create_link!(
        entry_hash.0,
        my_address,
        utils::link_tag("has_commitment_from")?
    )?;

    Ok(())
}

#[hdk_extern]
pub fn decline_invitation_and_commit(entry_hash: WrappedEntryHash) -> ExternResult<()> {
    delete_my_invitations_to(entry_hash.0)
}

#[derive(Serialize, Deserialize, SerializedBytes, Clone, Debug)]
pub struct GetEntriesOutput(Vec<WrappedEntryHash>);
#[hdk_extern]
pub fn get_my_invitations(_: ()) -> ExternResult<GetEntriesOutput> {
    let agent_info = agent_info!()?;

    let my_address = pub_key_to_entry_hash(agent_info.agent_initial_pubkey);

    let links = get_links!(my_address, utils::link_tag("is_invited_to")?)?;

    let entries = links
        .into_inner()
        .into_iter()
        .map(|link| WrappedEntryHash(link.target))
        .collect();

    Ok(GetEntriesOutput(entries))
}

#[hdk_extern]
pub fn get_my_commitments(_: ()) -> ExternResult<GetEntriesOutput> {
    let agent_info = agent_info!()?;

    let my_address = pub_key_to_entry_hash(agent_info.agent_initial_pubkey);

    let links = get_links!(my_address, utils::link_tag("has_committed_to")?)?;

    let entries = links
        .into_inner()
        .into_iter()
        .map(|link| WrappedEntryHash(link.target))
        .collect();

    Ok(GetEntriesOutput(entries))
}

/** Helper functions */

fn delete_my_invitations_to(entry_hash: EntryHash) -> ExternResult<()> {
    let agent_info = agent_info!()?;

    let my_address = pub_key_to_entry_hash(agent_info.agent_initial_pubkey);

    let links = get_links!(my_address, utils::link_tag("is_invited_to")?)?;

    let invitation_links: Vec<Link> = links
        .into_inner()
        .into_iter()
        .filter(|link| link.target.eq(&entry_hash))
        .collect();

    for invitation_link in invitation_links {
        delete_link!(invitation_link.create_link_hash)?;
    }

    Ok(())
}

fn entry_hash_to_pub_key(entry_hash: EntryHash) -> AgentPubKey {
    let bytes = entry_hash.into_inner();
    AgentPubKey::from_raw_bytes(bytes)
}

fn pub_key_to_entry_hash(agent_pub_key: AgentPubKey) -> EntryHash {
    let agent_address: AnyDhtHash = agent_pub_key.into();
    agent_address.into()
}
