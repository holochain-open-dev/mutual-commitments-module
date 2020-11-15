import { Resolvers } from '@apollo/client/core';
import { AppWebsocket, CellId } from '@holochain/conductor-api';
export declare function mutualCommitmentsResolvers(appWebsocket: AppWebsocket, cellId: CellId, zomeName?: string): Resolvers;
