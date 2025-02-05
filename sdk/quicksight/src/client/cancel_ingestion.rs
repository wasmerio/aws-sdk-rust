// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CancelIngestion`](crate::operation::cancel_ingestion::builders::CancelIngestionFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`aws_account_id(impl ::std::convert::Into<String>)`](crate::operation::cancel_ingestion::builders::CancelIngestionFluentBuilder::aws_account_id) / [`set_aws_account_id(Option<String>)`](crate::operation::cancel_ingestion::builders::CancelIngestionFluentBuilder::set_aws_account_id): <p>The Amazon Web Services account ID.</p>
    ///   - [`data_set_id(impl ::std::convert::Into<String>)`](crate::operation::cancel_ingestion::builders::CancelIngestionFluentBuilder::data_set_id) / [`set_data_set_id(Option<String>)`](crate::operation::cancel_ingestion::builders::CancelIngestionFluentBuilder::set_data_set_id): <p>The ID of the dataset used in the ingestion.</p>
    ///   - [`ingestion_id(impl ::std::convert::Into<String>)`](crate::operation::cancel_ingestion::builders::CancelIngestionFluentBuilder::ingestion_id) / [`set_ingestion_id(Option<String>)`](crate::operation::cancel_ingestion::builders::CancelIngestionFluentBuilder::set_ingestion_id): <p>An ID for the ingestion.</p>
    /// - On success, responds with [`CancelIngestionOutput`](crate::operation::cancel_ingestion::CancelIngestionOutput) with field(s):
    ///   - [`arn(Option<String>)`](crate::operation::cancel_ingestion::CancelIngestionOutput::arn): <p>The Amazon Resource Name (ARN) for the data ingestion.</p>
    ///   - [`ingestion_id(Option<String>)`](crate::operation::cancel_ingestion::CancelIngestionOutput::ingestion_id): <p>An ID for the ingestion.</p>
    ///   - [`request_id(Option<String>)`](crate::operation::cancel_ingestion::CancelIngestionOutput::request_id): <p>The Amazon Web Services request ID for this operation.</p>
    ///   - [`status(i32)`](crate::operation::cancel_ingestion::CancelIngestionOutput::status): <p>The HTTP status of the request.</p>
    /// - On failure, responds with [`SdkError<CancelIngestionError>`](crate::operation::cancel_ingestion::CancelIngestionError)
    pub fn cancel_ingestion(&self) -> crate::operation::cancel_ingestion::builders::CancelIngestionFluentBuilder {
        crate::operation::cancel_ingestion::builders::CancelIngestionFluentBuilder::new(self.handle.clone())
    }
}
