export function entryCertificationsResolvers(appWebsocket, cellId, zomeName = 'entry_certifications') {
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
        CertificatedEntry: {
            async certifications(entry) {
                const certifications = await callZome('get_certifications_for', entry.id);
                return certifications.map(([certificationId, certification]) => ({
                    id: certificationId,
                    ...certification,
                }));
            },
        },
        Agent: {
            async pendingCertifications(agent) {
                const entries = await callZome('get_agent_invitations', agent.id);
                return entries.map((entry) => ({
                    id: entry,
                }));
            },
        },
        Mutation: {
            async requestForCertification(_, { entryId, certificatorsIds }) {
                await callZome('request_for_certification', {
                    entry_hash: entryId,
                    certificators_pub_keys: certificatorsIds,
                });
                return true;
            },
            async certifyInitiative(_, { certificationId }) {
                await callZome('certify_initiative', certificationId);
                return true;
            },
            async rejectCertification(_, { certificationId }) {
                await callZome('reject_initiative', certificationId);
                return true;
            },
        },
    };
}
//# sourceMappingURL=resolvers.js.map