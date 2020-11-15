import { ApolloClient } from '@apollo/client/core';
export interface MutualCommitmentsModuleDependencies {
    apolloClient: ApolloClient<any>;
}
export declare class MutualCommitmentsModule {
    protected dependencies: MutualCommitmentsModuleDependencies;
    constructor(dependencies: MutualCommitmentsModuleDependencies);
    /** Public methods */
    install(): void;
    static isInstalled(): boolean;
    /** Private helpers */
    private checkApolloClientTypeDefs;
}
