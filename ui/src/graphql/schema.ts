import { gql } from '@apollo/client/core';

// TODO: define your own schema

export const calendarEventsTypeDefs = gql`
  extend type Agent {
    pendingInvitesTo: [HolochainEntry!]!
    committedTo: [HolochainEntry!]!
  }

  extend type Mutation {
    # Commitments
    inviteToCommit(agentsIds: [ID!]!): Boolean
    acceptInvitationAndCommit(entryId: ID!): Boolean
    rejectInvitation(entryId: ID!): Boolean
  }
`;
