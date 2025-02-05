// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ExecuteScheduledQuery`](crate::operation::execute_scheduled_query::builders::ExecuteScheduledQueryFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`scheduled_query_arn(impl ::std::convert::Into<String>)`](crate::operation::execute_scheduled_query::builders::ExecuteScheduledQueryFluentBuilder::scheduled_query_arn) / [`set_scheduled_query_arn(Option<String>)`](crate::operation::execute_scheduled_query::builders::ExecuteScheduledQueryFluentBuilder::set_scheduled_query_arn): <p>ARN of the scheduled query.</p>
    ///   - [`invocation_time(DateTime)`](crate::operation::execute_scheduled_query::builders::ExecuteScheduledQueryFluentBuilder::invocation_time) / [`set_invocation_time(Option<DateTime>)`](crate::operation::execute_scheduled_query::builders::ExecuteScheduledQueryFluentBuilder::set_invocation_time): <p>The timestamp in UTC. Query will be run as if it was invoked at this timestamp. </p>
    ///   - [`client_token(impl ::std::convert::Into<String>)`](crate::operation::execute_scheduled_query::builders::ExecuteScheduledQueryFluentBuilder::client_token) / [`set_client_token(Option<String>)`](crate::operation::execute_scheduled_query::builders::ExecuteScheduledQueryFluentBuilder::set_client_token): <p>Not used. </p>
    /// - On success, responds with [`ExecuteScheduledQueryOutput`](crate::operation::execute_scheduled_query::ExecuteScheduledQueryOutput)
    /// - On failure, responds with [`SdkError<ExecuteScheduledQueryError>`](crate::operation::execute_scheduled_query::ExecuteScheduledQueryError)
    pub fn execute_scheduled_query(&self) -> crate::operation::execute_scheduled_query::builders::ExecuteScheduledQueryFluentBuilder {
        crate::operation::execute_scheduled_query::builders::ExecuteScheduledQueryFluentBuilder::new(self.handle.clone())
    }
}
