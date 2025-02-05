// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeCustomVocabularyMetadata`](crate::operation::describe_custom_vocabulary_metadata::builders::DescribeCustomVocabularyMetadataFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`bot_id(impl ::std::convert::Into<String>)`](crate::operation::describe_custom_vocabulary_metadata::builders::DescribeCustomVocabularyMetadataFluentBuilder::bot_id) / [`set_bot_id(Option<String>)`](crate::operation::describe_custom_vocabulary_metadata::builders::DescribeCustomVocabularyMetadataFluentBuilder::set_bot_id): <p>The unique identifier of the bot that contains the custom vocabulary.</p>
    ///   - [`bot_version(impl ::std::convert::Into<String>)`](crate::operation::describe_custom_vocabulary_metadata::builders::DescribeCustomVocabularyMetadataFluentBuilder::bot_version) / [`set_bot_version(Option<String>)`](crate::operation::describe_custom_vocabulary_metadata::builders::DescribeCustomVocabularyMetadataFluentBuilder::set_bot_version): <p>The bot version of the bot to return metadata for.</p>
    ///   - [`locale_id(impl ::std::convert::Into<String>)`](crate::operation::describe_custom_vocabulary_metadata::builders::DescribeCustomVocabularyMetadataFluentBuilder::locale_id) / [`set_locale_id(Option<String>)`](crate::operation::describe_custom_vocabulary_metadata::builders::DescribeCustomVocabularyMetadataFluentBuilder::set_locale_id): <p>The locale to return the custom vocabulary information for. The locale must be <code>en_GB</code>.</p>
    /// - On success, responds with [`DescribeCustomVocabularyMetadataOutput`](crate::operation::describe_custom_vocabulary_metadata::DescribeCustomVocabularyMetadataOutput) with field(s):
    ///   - [`bot_id(Option<String>)`](crate::operation::describe_custom_vocabulary_metadata::DescribeCustomVocabularyMetadataOutput::bot_id): <p>The identifier of the bot that contains the custom vocabulary.</p>
    ///   - [`bot_version(Option<String>)`](crate::operation::describe_custom_vocabulary_metadata::DescribeCustomVocabularyMetadataOutput::bot_version): <p>The version of the bot that contains the custom vocabulary to describe.</p>
    ///   - [`locale_id(Option<String>)`](crate::operation::describe_custom_vocabulary_metadata::DescribeCustomVocabularyMetadataOutput::locale_id): <p>The locale that contains the custom vocabulary to describe.</p>
    ///   - [`custom_vocabulary_status(Option<CustomVocabularyStatus>)`](crate::operation::describe_custom_vocabulary_metadata::DescribeCustomVocabularyMetadataOutput::custom_vocabulary_status): <p>The status of the custom vocabulary. If the status is <code>Ready</code> the custom vocabulary is ready to use.</p>
    ///   - [`creation_date_time(Option<DateTime>)`](crate::operation::describe_custom_vocabulary_metadata::DescribeCustomVocabularyMetadataOutput::creation_date_time): <p>The date and time that the custom vocabulary was created.</p>
    ///   - [`last_updated_date_time(Option<DateTime>)`](crate::operation::describe_custom_vocabulary_metadata::DescribeCustomVocabularyMetadataOutput::last_updated_date_time): <p>The date and time that the custom vocabulary was last updated.</p>
    /// - On failure, responds with [`SdkError<DescribeCustomVocabularyMetadataError>`](crate::operation::describe_custom_vocabulary_metadata::DescribeCustomVocabularyMetadataError)
    pub fn describe_custom_vocabulary_metadata(
        &self,
    ) -> crate::operation::describe_custom_vocabulary_metadata::builders::DescribeCustomVocabularyMetadataFluentBuilder {
        crate::operation::describe_custom_vocabulary_metadata::builders::DescribeCustomVocabularyMetadataFluentBuilder::new(self.handle.clone())
    }
}
