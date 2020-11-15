import { gql } from '@apollo/client/core';
export const INVITE_TO_COMMIT = gql `
  mutation InviteToCommit($entryId: ID!, $agentsIds: [ID!]!) {
    inviteToCommit(entryId: $entryId, agentsIds: $agentsIds)
  }
`;
export const ACCEPT_INVITATION_AND_COMMIT = gql `
  mutation AcceptInvitationAndCommit($entryId: ID!) {
    acceptInvitationAndCommit(entryId: $entryId)
  }
`;
export const DECLINE_INVITATION = gql `
  mutation DeclineInvitation($entryId: ID!) {
    declineInvitation(entryId: $entryId)
  }
`;
//# sourceMappingURL=queries.js.map