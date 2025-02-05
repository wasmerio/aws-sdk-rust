// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListFileShares`](crate::operation::list_file_shares::builders::ListFileSharesFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_file_shares::builders::ListFileSharesFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`gateway_arn(impl ::std::convert::Into<String>)`](crate::operation::list_file_shares::builders::ListFileSharesFluentBuilder::gateway_arn) / [`set_gateway_arn(Option<String>)`](crate::operation::list_file_shares::builders::ListFileSharesFluentBuilder::set_gateway_arn): <p>The Amazon Resource Name (ARN) of the gateway whose file shares you want to list. If this field is not present, all file shares under your account are listed.</p>
    ///   - [`limit(i32)`](crate::operation::list_file_shares::builders::ListFileSharesFluentBuilder::limit) / [`set_limit(Option<i32>)`](crate::operation::list_file_shares::builders::ListFileSharesFluentBuilder::set_limit): <p>The maximum number of file shares to return in the response. The value must be an integer with a value greater than zero. Optional.</p>
    ///   - [`marker(impl ::std::convert::Into<String>)`](crate::operation::list_file_shares::builders::ListFileSharesFluentBuilder::marker) / [`set_marker(Option<String>)`](crate::operation::list_file_shares::builders::ListFileSharesFluentBuilder::set_marker): <p>Opaque pagination token returned from a previous ListFileShares operation. If present, <code>Marker</code> specifies where to continue the list from after a previous call to ListFileShares. Optional.</p>
    /// - On success, responds with [`ListFileSharesOutput`](crate::operation::list_file_shares::ListFileSharesOutput) with field(s):
    ///   - [`marker(Option<String>)`](crate::operation::list_file_shares::ListFileSharesOutput::marker): <p>If the request includes <code>Marker</code>, the response returns that value in this field.</p>
    ///   - [`next_marker(Option<String>)`](crate::operation::list_file_shares::ListFileSharesOutput::next_marker): <p>If a value is present, there are more file shares to return. In a subsequent request, use <code>NextMarker</code> as the value for <code>Marker</code> to retrieve the next set of file shares.</p>
    ///   - [`file_share_info_list(Option<Vec<FileShareInfo>>)`](crate::operation::list_file_shares::ListFileSharesOutput::file_share_info_list): <p>An array of information about the S3 File Gateway's file shares.</p>
    /// - On failure, responds with [`SdkError<ListFileSharesError>`](crate::operation::list_file_shares::ListFileSharesError)
    pub fn list_file_shares(&self) -> crate::operation::list_file_shares::builders::ListFileSharesFluentBuilder {
        crate::operation::list_file_shares::builders::ListFileSharesFluentBuilder::new(self.handle.clone())
    }
}
