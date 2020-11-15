import { Resolvers } from '@apollo/client/core';
import { AppWebsocket, CellId } from '@holochain/conductor-api';
export declare function entryCertificationsResolvers(appWebsocket: AppWebsocket, cellId: CellId, zomeName?: string): Resolvers;
