export function mutualCommitmentsResolvers(appWebsocket, cellId, zomeName = 'mutual_commitments') {
    function callZome(fnName, payload) {
        return appWebsocket.callZome({
            cap: null,
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
                return agents.map((agent) => ({ id: agent }));
            },
            async committedAgents(entry) {
                const agents = await callZome('get_committed_agents_for', entry.id);
                return agents.map((agent) => ({ id: agent }));
            },
        },
        Agent: {
            async pendingCommitmentInvites(agent) {
                const entries = await callZome('get_agent_invitations', agent.id);
                return entries.map((entry) => ({
                    id: entry,
                }));
            },
            async committedTo(agent) {
                const entries = await callZome('get_agent_commitments', agent.id);
                return entries.map((entry) => ({
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
//# sourceMappingURL=resolvers.js.map