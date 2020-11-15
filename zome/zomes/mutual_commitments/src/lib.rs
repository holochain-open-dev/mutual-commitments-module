use hc_utils::{WrappedAgentPubKey, WrappedEntryHash};
use hdk3::prelude::link::Link;
use hdk3::prelude::*;

mod handlers;
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
    agents_to_invite: Vec<WrappedAgentPubKey>,
}
#[hdk_extern]
pub fn invite_agents_to_commit(
    invite_agents_to_commit_input: InviteAgentsToCommit,
) -> ExternResult<()> {
    for agent_to_invite in invite_agents_to_commit_input.agents_to_invite {
        create_link!(
            utils::pub_key_to_entry_hash(agent_to_invite.0.clone()),
            invite_agents_to_commit_input.entry_hash.0.clone(),
            utils::link_tag("is_invited_to")?
        )?;
        create_link!(
            invite_agents_to_commit_input.entry_hash.0.clone(),
            utils::pub_key_to_entry_hash(agent_to_invite.0.clone()),
            utils::link_tag("invited_agent")?
        )?;
    }

    Ok(())
}

#[hdk_extern]
pub fn accept_invitation_and_commit(entry_hash: WrappedEntryHash) -> ExternResult<()> {
    let agent_info = agent_info!()?;

    delete_my_invitations_to(entry_hash.0.clone())?;

    let my_address = utils::pub_key_to_entry_hash(agent_info.agent_initial_pubkey);

    create_link!(
        my_address.clone(),
        entry_hash.0.clone(),
        utils::link_tag("has_committed_to")?
    )?;
    create_link!(
        entry_hash.0,
        my_address,
        utils::link_tag("committed_agent")?
    )?;

    Ok(())
}

#[hdk_extern]
pub fn decline_invitation(entry_hash: WrappedEntryHash) -> ExternResult<()> {
    delete_my_invitations_to(entry_hash.0)
}

#[derive(Serialize, Deserialize, SerializedBytes, Clone, Debug)]
pub struct GetAgentsOutput(Vec<WrappedAgentPubKey>);
#[hdk_extern]
pub fn get_committed_agents_for(entry_hash: WrappedEntryHash) -> ExternResult<GetAgentsOutput> {
    let committed_agents =
        handlers::get_links_from_entry_wrapped(entry_hash.0, utils::link_tag("committed_agent")?)?;

    Ok(GetAgentsOutput(committed_agents))
}

#[hdk_extern]
pub fn get_invited_agents_for(entry_hash: WrappedEntryHash) -> ExternResult<GetAgentsOutput> {
    let committed_agents =
        handlers::get_links_from_entry_wrapped(entry_hash.0, utils::link_tag("invited_agent")?)?;

    Ok(GetAgentsOutput(committed_agents))
}

#[derive(Serialize, Deserialize, SerializedBytes, Clone, Debug)]
pub struct GetEntriesOutput(Vec<WrappedEntryHash>);
#[hdk_extern]
pub fn get_my_invitations(_: ()) -> ExternResult<GetEntriesOutput> {
    let agent_info = agent_info!()?;

    let committed_entries = handlers::get_links_from_agent_wrapped(
        agent_info.agent_initial_pubkey,
        utils::link_tag("is_invited_to")?,
    )?;

    Ok(GetEntriesOutput(committed_entries))
}

#[hdk_extern]
pub fn get_agent_commitments(agent_pub_key: WrappedAgentPubKey) -> ExternResult<GetEntriesOutput> {
    let committed_entries = handlers::get_links_from_agent_wrapped(
        agent_pub_key.0,
        utils::link_tag("has_committed_to")?,
    )?;

    Ok(GetEntriesOutput(committed_entries))
}

#[hdk_extern]
pub fn get_my_commitments(_: ()) -> ExternResult<GetEntriesOutput> {
    let agent_info = agent_info!()?;
    let committed_entries = handlers::get_links_from_agent_wrapped(
        agent_info.agent_initial_pubkey,
        utils::link_tag("has_committed_to")?,
    )?;

    Ok(GetEntriesOutput(committed_entries))
}

#[hdk_extern]
pub fn get_agent_invitations(agent_pub_key: WrappedAgentPubKey) -> ExternResult<GetEntriesOutput> {
    let committed_entries =
        handlers::get_links_from_agent_wrapped(agent_pub_key.0, utils::link_tag("is_invited_to")?)?;

    Ok(GetEntriesOutput(committed_entries))
}

/** Helper functions */

fn delete_my_invitations_to(entry_hash: EntryHash) -> ExternResult<()> {
    let agent_info = agent_info!()?;

    let my_address = utils::pub_key_to_entry_hash(agent_info.agent_initial_pubkey);

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
