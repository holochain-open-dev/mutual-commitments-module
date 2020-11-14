use hc_utils::WrappedAgentPubKey;
use hdk3::prelude::*;

mod utils;

pub fn error<T>(reason: &str) -> ExternResult<T> {
    Err(HdkError::Wasm(WasmError::Zome(String::from(reason))))
}

/** Invitations and commitments **/

#[derive(Clone, Serialize, Deserialize, SerializedBytes)]
pub struct InviteAgentsToCommit {
    entry_hash: EntryHash,
    invited_agents: Vec<WrappedAgentPubKey>,
}
#[hdk_extern]
pub fn invite_agents_to_commit(
    invite_agents_to_commit_input: InviteAgentsToCommit,
) -> ExternResult<()> {
    for invited_agent in invite_agents_to_commit_input.invited_agents {
        create_link!(
            pub_key_to_entry_hash(invited_agent.0),
            invite_agents_to_commit_input.entry_hash,
            utils::link_tag("is_invited_to")
        )?;
        create_link!(
            invite_agents_to_commit_input.entry_hash,
            pub_key_to_entry_hash(invited_agent.0),
            utils::link_tag("has_invitee")
        )?;
    }

    Ok(())
}

#[hdk_extern]
pub fn accept_invitation_and_commit(entry_hash: EntryHash) -> ExternResult<()> {
    let agent_info = agent_info!()?;

    delete_my_invitations_to(entry_hash.clone())?;

    let my_address = pub_key_to_entry_hash(agent_info.agent_initial_pubkey);

    create_link!(my_address, entry_hash, utils::link_tag("has_committed_to")?)?;
    create_link!(
        entry_hash,
        my_address,
        utils::link_tag("has_commitment_from")?
    )?;
}

#[hdk_extern]
pub fn decline_invitation_and_commit(entry_hash: EntryHash) -> ExternResult<()> {
    delete_my_invitations_to(entry_hash)
}

#[derive(Serialize, Deserialize, SerializedBytes, Clone, Debug)]
struct GetEntriesOutput(Vec<EntryHash>);
#[hdk_extern]
pub fn get_my_invitations(_: ()) -> ExternResult<GetEntriesOutput> {
    let agent_info = agent_info!()?;

    let my_address = pub_key_to_entry_hash(agent_info.agent_initial_pubkey);

    let links = get_links!(my_address, utils::link_tag("is_invited_to")?)?;

    links
        .into_inner()
        .into_iter()
        .map(|link| link.target)
        .collect()
}

#[hdk_extern]
pub fn get_my_commitments(_: ()) -> ExternResult<GetEntriesOutput> {
    let agent_info = agent_info!()?;

    let my_address = pub_key_to_entry_hash(agent_info.agent_initial_pubkey);

    let links = get_links!(my_address, utils::link_tag("has_committed_to")?)?;

    links
        .into_inner()
        .into_iter()
        .map(|link| link.target)
        .collect()
}

/** Helper functions */

fn delete_my_invitations_to(entry_hash: EntryHash) -> ExternResult<()> {
    let agent_info = agent_info!()?;

    let my_address = pub_key_to_entry_hash(agent_info.agent_initial_pubkey);

    let links = get_links!(my_address, utils::link_tag("is_invited_to")?)?;

    let invitation_links: Vec<Link> = links
        .into_inner()
        .into_iter()
        .filter(|link| link.target.eq(entry_hash))
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
