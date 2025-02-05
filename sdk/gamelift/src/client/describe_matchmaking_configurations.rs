// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeMatchmakingConfigurations`](crate::operation::describe_matchmaking_configurations::builders::DescribeMatchmakingConfigurationsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::describe_matchmaking_configurations::builders::DescribeMatchmakingConfigurationsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`names(Vec<String>)`](crate::operation::describe_matchmaking_configurations::builders::DescribeMatchmakingConfigurationsFluentBuilder::names) / [`set_names(Option<Vec<String>>)`](crate::operation::describe_matchmaking_configurations::builders::DescribeMatchmakingConfigurationsFluentBuilder::set_names): <p>A unique identifier for the matchmaking configuration(s) to retrieve. You can use either the configuration name or ARN value. To request all existing configurations, leave this parameter empty.</p>
    ///   - [`rule_set_name(impl ::std::convert::Into<String>)`](crate::operation::describe_matchmaking_configurations::builders::DescribeMatchmakingConfigurationsFluentBuilder::rule_set_name) / [`set_rule_set_name(Option<String>)`](crate::operation::describe_matchmaking_configurations::builders::DescribeMatchmakingConfigurationsFluentBuilder::set_rule_set_name): <p>A unique identifier for the matchmaking rule set. You can use either the rule set name or ARN value. Use this parameter to retrieve all matchmaking configurations that use this rule set.</p>
    ///   - [`limit(i32)`](crate::operation::describe_matchmaking_configurations::builders::DescribeMatchmakingConfigurationsFluentBuilder::limit) / [`set_limit(Option<i32>)`](crate::operation::describe_matchmaking_configurations::builders::DescribeMatchmakingConfigurationsFluentBuilder::set_limit): <p>The maximum number of results to return. Use this parameter with <code>NextToken</code> to get results as a set of sequential pages. This parameter is limited to 10.</p>
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::describe_matchmaking_configurations::builders::DescribeMatchmakingConfigurationsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::describe_matchmaking_configurations::builders::DescribeMatchmakingConfigurationsFluentBuilder::set_next_token): <p>A token that indicates the start of the next sequential page of results. Use the token that is returned with a previous call to this operation. To start at the beginning of the result set, do not specify a value.</p>
    /// - On success, responds with [`DescribeMatchmakingConfigurationsOutput`](crate::operation::describe_matchmaking_configurations::DescribeMatchmakingConfigurationsOutput) with field(s):
    ///   - [`configurations(Option<Vec<MatchmakingConfiguration>>)`](crate::operation::describe_matchmaking_configurations::DescribeMatchmakingConfigurationsOutput::configurations): <p>A collection of requested matchmaking configurations.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::describe_matchmaking_configurations::DescribeMatchmakingConfigurationsOutput::next_token): <p>A token that indicates where to resume retrieving results on the next call to this operation. If no token is returned, these results represent the end of the list.</p>
    /// - On failure, responds with [`SdkError<DescribeMatchmakingConfigurationsError>`](crate::operation::describe_matchmaking_configurations::DescribeMatchmakingConfigurationsError)
    pub fn describe_matchmaking_configurations(
        &self,
    ) -> crate::operation::describe_matchmaking_configurations::builders::DescribeMatchmakingConfigurationsFluentBuilder {
        crate::operation::describe_matchmaking_configurations::builders::DescribeMatchmakingConfigurationsFluentBuilder::new(self.handle.clone())
    }
}
