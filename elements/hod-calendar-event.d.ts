import { ApolloClient } from '@apollo/client/core';
import { LitElement } from 'lit-element';
export declare abstract class HodCalendarEvent extends LitElement {
    /** Public attributes */
    /**
     * This is a description of a property with an attribute with exactly the same name: "color".
     */
    title: string;
    /** Dependencies */
    abstract get _apolloClient(): ApolloClient<any>;
    /** Private properties */
    _counter: number;
    static styles: import("lit-element").CSSResult;
    __increment(): void;
    render(): import("lit-element").TemplateResult;
}
