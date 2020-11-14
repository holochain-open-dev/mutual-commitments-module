import { ApolloClient } from '@apollo/client/core';
import { HodCalendarEvent } from './elements/hod-calendar-event';
import { mutualCommitmentsTypeDef } from './graphql/schema';
import { setupApolloClientElement } from './utils';

// TODO: define your dependencies
export interface MutualCommitmentsModuleDependencies {
  apolloClient: ApolloClient<any>;
}

// TODO: add all your elements and dependencies
export class MutualCommitmentsModule {
  constructor(protected dependencies: MutualCommitmentsModuleDependencies) {
    this.checkApolloClientTypeDefs(dependencies.apolloClient);
  }

  /** Public methods */

  install() {
    customElements.define(
      'hod-calendar-event',
      setupApolloClientElement(HodCalendarEvent, this.dependencies.apolloClient)
    );
  }

  static isInstalled(): boolean {
    return customElements.get('hod-calendar-event');
  }

  /** Private helpers */
  private checkApolloClientTypeDefs(apolloClient: ApolloClient<any>): void {
    if (
      !Array.isArray(apolloClient.typeDefs) ||
      !apolloClient.typeDefs.includes(mutualCommitmentsTypeDef as any)
    )
      throw new Error(
        'Error initializing Module: ApolloClient must be initialized using an array of typeDefs containing the mutualCommitmentsTypeDefs'
      );
  }
}
