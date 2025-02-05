// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListTestSetRecords`](crate::operation::list_test_set_records::builders::ListTestSetRecordsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_test_set_records::builders::ListTestSetRecordsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`test_set_id(impl ::std::convert::Into<String>)`](crate::operation::list_test_set_records::builders::ListTestSetRecordsFluentBuilder::test_set_id) / [`set_test_set_id(Option<String>)`](crate::operation::list_test_set_records::builders::ListTestSetRecordsFluentBuilder::set_test_set_id): <p>The identifier of the test set to list its test set records.</p>
    ///   - [`max_results(i32)`](crate::operation::list_test_set_records::builders::ListTestSetRecordsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_test_set_records::builders::ListTestSetRecordsFluentBuilder::set_max_results): <p>The maximum number of test set records to return in each page. If there are fewer records than the max page size, only the actual number of records are returned.</p>
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::list_test_set_records::builders::ListTestSetRecordsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_test_set_records::builders::ListTestSetRecordsFluentBuilder::set_next_token): <p>If the response from the ListTestSetRecords operation contains more results than specified in the maxResults parameter, a token is returned in the response. Use that token in the nextToken parameter to return the next page of results.</p>
    /// - On success, responds with [`ListTestSetRecordsOutput`](crate::operation::list_test_set_records::ListTestSetRecordsOutput) with field(s):
    ///   - [`test_set_records(Option<Vec<TestSetTurnRecord>>)`](crate::operation::list_test_set_records::ListTestSetRecordsOutput::test_set_records): <p>The list of records from the test set.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_test_set_records::ListTestSetRecordsOutput::next_token): <p>A token that indicates whether there are more records to return in a response to the ListTestSetRecords operation. If the nextToken field is present, you send the contents as the nextToken parameter of a ListTestSetRecords operation request to get the next page of records.</p>
    /// - On failure, responds with [`SdkError<ListTestSetRecordsError>`](crate::operation::list_test_set_records::ListTestSetRecordsError)
    pub fn list_test_set_records(&self) -> crate::operation::list_test_set_records::builders::ListTestSetRecordsFluentBuilder {
        crate::operation::list_test_set_records::builders::ListTestSetRecordsFluentBuilder::new(self.handle.clone())
    }
}
