import { Resolvers } from '@apollo/client/core';
import { AppWebsocket, CellId } from '@holochain/conductor-api';

export function mutualCommitmentsResolvers(
  appWebsocket: AppWebsocket,
  cellId: CellId,
  zomeName = 'mutual_commitments'
): Resolvers {
  function callZome(fnName: string, payload: any) {
    return appWebsocket.callZome({
      cap: null as any,
      cell_id: cellId,
      zome_name: zomeName,
      fn_name: fnName,
      payload,
      provenance: cellId[1],
    });
  }

  return {
    MutualCommitmentEntry: {
      async invitedAgents(entry) {
        const agents = await callZome('get_invited_agents_for', entry.id);
        return agents.map((agent: string) => ({ id: agent }));
      },
      async committedAgents(entry) {
        const agents = await callZome('get_committed_agents_for', entry.id);
        return agents.map((agent: string) => ({ id: agent }));
      },
    },
    Agent: {
      async pendingCommitmentInvites(agent) {
        const entries = await callZome('get_agent_invitations', agent.id);

        return entries.map((entry: any) => ({
          id: entry,
        }));
      },
      async committedTo(agent) {
        const entries = await callZome('get_agent_commitments', agent.id);

        return entries.map((entry: any) => ({
          id: entry,
        }));
      },
    },
    Mutation: {
      async inviteToCommit(_, { entryId, agentsIds }) {
        await callZome('invite_agents_to_commit', {
          entry_hash: entryId,
          agents_to_invite: agentsIds,
        });

        return true;
      },
      async acceptInvitationAndCommit(_, { entryId }) {
        await callZome('accept_invitation_and_commit', entryId);

        return true;
      },
      async declineInvitation(_, { entryId }) {
        await callZome('decline_invitation', entryId);

        return true;
      },
    },
  };
}
