// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListTables`](crate::operation::list_tables::builders::ListTablesFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_tables::builders::ListTablesFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::list_tables::builders::ListTablesFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_tables::builders::ListTablesFluentBuilder::set_next_token): <p>The pagination token. To resume pagination, provide the <code>NextToken</code> value as an argument of a subsequent API invocation.</p>
    ///   - [`max_results(i32)`](crate::operation::list_tables::builders::ListTablesFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_tables::builders::ListTablesFluentBuilder::set_max_results): <p>The total number of tables to return in the output. If the total number of tables available is more than the value specified, a <code>NextToken</code> is provided in the output. To resume pagination, provide the <code>NextToken</code> value as an argument of a subsequent API invocation.</p>
    ///   - [`keyspace_name(impl ::std::convert::Into<String>)`](crate::operation::list_tables::builders::ListTablesFluentBuilder::keyspace_name) / [`set_keyspace_name(Option<String>)`](crate::operation::list_tables::builders::ListTablesFluentBuilder::set_keyspace_name): <p>The name of the keyspace.</p>
    /// - On success, responds with [`ListTablesOutput`](crate::operation::list_tables::ListTablesOutput) with field(s):
    ///   - [`next_token(Option<String>)`](crate::operation::list_tables::ListTablesOutput::next_token): <p>A token to specify where to start paginating. This is the <code>NextToken</code> from a previously truncated response.</p>
    ///   - [`tables(Option<Vec<TableSummary>>)`](crate::operation::list_tables::ListTablesOutput::tables): <p>A list of tables.</p>
    /// - On failure, responds with [`SdkError<ListTablesError>`](crate::operation::list_tables::ListTablesError)
    pub fn list_tables(&self) -> crate::operation::list_tables::builders::ListTablesFluentBuilder {
        crate::operation::list_tables::builders::ListTablesFluentBuilder::new(self.handle.clone())
    }
}
