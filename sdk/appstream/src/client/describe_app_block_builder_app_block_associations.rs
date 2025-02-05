// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeAppBlockBuilderAppBlockAssociations`](crate::operation::describe_app_block_builder_app_block_associations::builders::DescribeAppBlockBuilderAppBlockAssociationsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::describe_app_block_builder_app_block_associations::builders::DescribeAppBlockBuilderAppBlockAssociationsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`app_block_arn(impl ::std::convert::Into<String>)`](crate::operation::describe_app_block_builder_app_block_associations::builders::DescribeAppBlockBuilderAppBlockAssociationsFluentBuilder::app_block_arn) / [`set_app_block_arn(Option<String>)`](crate::operation::describe_app_block_builder_app_block_associations::builders::DescribeAppBlockBuilderAppBlockAssociationsFluentBuilder::set_app_block_arn): <p>The ARN of the app block.</p>
    ///   - [`app_block_builder_name(impl ::std::convert::Into<String>)`](crate::operation::describe_app_block_builder_app_block_associations::builders::DescribeAppBlockBuilderAppBlockAssociationsFluentBuilder::app_block_builder_name) / [`set_app_block_builder_name(Option<String>)`](crate::operation::describe_app_block_builder_app_block_associations::builders::DescribeAppBlockBuilderAppBlockAssociationsFluentBuilder::set_app_block_builder_name): <p>The name of the app block builder.</p>
    ///   - [`max_results(i32)`](crate::operation::describe_app_block_builder_app_block_associations::builders::DescribeAppBlockBuilderAppBlockAssociationsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::describe_app_block_builder_app_block_associations::builders::DescribeAppBlockBuilderAppBlockAssociationsFluentBuilder::set_max_results): <p>The maximum size of each page of results.</p>
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::describe_app_block_builder_app_block_associations::builders::DescribeAppBlockBuilderAppBlockAssociationsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::describe_app_block_builder_app_block_associations::builders::DescribeAppBlockBuilderAppBlockAssociationsFluentBuilder::set_next_token): <p>The pagination token used to retrieve the next page of results for this operation.</p>
    /// - On success, responds with [`DescribeAppBlockBuilderAppBlockAssociationsOutput`](crate::operation::describe_app_block_builder_app_block_associations::DescribeAppBlockBuilderAppBlockAssociationsOutput) with field(s):
    ///   - [`app_block_builder_app_block_associations(Option<Vec<AppBlockBuilderAppBlockAssociation>>)`](crate::operation::describe_app_block_builder_app_block_associations::DescribeAppBlockBuilderAppBlockAssociationsOutput::app_block_builder_app_block_associations): <p>This list of app block builders associated with app blocks.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::describe_app_block_builder_app_block_associations::DescribeAppBlockBuilderAppBlockAssociationsOutput::next_token): <p>The pagination token used to retrieve the next page of results for this operation.</p>
    /// - On failure, responds with [`SdkError<DescribeAppBlockBuilderAppBlockAssociationsError>`](crate::operation::describe_app_block_builder_app_block_associations::DescribeAppBlockBuilderAppBlockAssociationsError)
    pub fn describe_app_block_builder_app_block_associations(
        &self,
    ) -> crate::operation::describe_app_block_builder_app_block_associations::builders::DescribeAppBlockBuilderAppBlockAssociationsFluentBuilder {
        crate::operation::describe_app_block_builder_app_block_associations::builders::DescribeAppBlockBuilderAppBlockAssociationsFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
