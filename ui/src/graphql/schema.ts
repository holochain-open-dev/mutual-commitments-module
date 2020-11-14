import { gql } from '@apollo/client/core';

export const mutualCommitmentsTypeDef = gql`
  extend type Agent {
    pendingCommitmentInvites: [HolochainEntry!]!
    committedTo: [HolochainEntry!]!
  }

  extend type Mutation {
    # Commitments
    inviteToCommit(entryId: ID!, agentsIds: [ID!]!): Boolean
    acceptInvitationAndCommit(entryId: ID!): Boolean
    declineInvitation(entryId: ID!): Boolean
  }
`;
