import { gql } from '@apollo/client/core';

export const entryCertificationsTypeDef = gql`
  interface CertificatedEntry implements HolochainEntry {
    id: ID!

    certifications: [Certification!]!
  }

  enum CertificationStatus {
    Pending
    Rejected
    Accepted
  }

  type Certification {
    id: ID!
    certificator: Agent!
    initiative: Initiative!

    status: CertificationStatus!
  }

  extend type Agent {
    pendingCertifications: [Certification!]!
  }

  extend type Mutation {
    requestForCertification(
      entryId: ID!
      certificatorsIds: [ID!]!
    ): Certification!
    certifyInitiative(certificationId: ID!): Certification!
    rejectCertification(certificationId: ID!): Certification!
  }
`;
