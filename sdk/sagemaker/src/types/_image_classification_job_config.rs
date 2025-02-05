// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Stores the configuration information for the image classification problem of an AutoML job V2.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ImageClassificationJobConfig {
    /// <p>How long a job is allowed to run, or how many candidates a job is allowed to generate.</p>
    pub completion_criteria: ::std::option::Option<crate::types::AutoMlJobCompletionCriteria>,
}
impl ImageClassificationJobConfig {
    /// <p>How long a job is allowed to run, or how many candidates a job is allowed to generate.</p>
    pub fn completion_criteria(&self) -> ::std::option::Option<&crate::types::AutoMlJobCompletionCriteria> {
        self.completion_criteria.as_ref()
    }
}
impl ImageClassificationJobConfig {
    /// Creates a new builder-style object to manufacture [`ImageClassificationJobConfig`](crate::types::ImageClassificationJobConfig).
    pub fn builder() -> crate::types::builders::ImageClassificationJobConfigBuilder {
        crate::types::builders::ImageClassificationJobConfigBuilder::default()
    }
}

/// A builder for [`ImageClassificationJobConfig`](crate::types::ImageClassificationJobConfig).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct ImageClassificationJobConfigBuilder {
    pub(crate) completion_criteria: ::std::option::Option<crate::types::AutoMlJobCompletionCriteria>,
}
impl ImageClassificationJobConfigBuilder {
    /// <p>How long a job is allowed to run, or how many candidates a job is allowed to generate.</p>
    pub fn completion_criteria(mut self, input: crate::types::AutoMlJobCompletionCriteria) -> Self {
        self.completion_criteria = ::std::option::Option::Some(input);
        self
    }
    /// <p>How long a job is allowed to run, or how many candidates a job is allowed to generate.</p>
    pub fn set_completion_criteria(mut self, input: ::std::option::Option<crate::types::AutoMlJobCompletionCriteria>) -> Self {
        self.completion_criteria = input;
        self
    }
    /// <p>How long a job is allowed to run, or how many candidates a job is allowed to generate.</p>
    pub fn get_completion_criteria(&self) -> &::std::option::Option<crate::types::AutoMlJobCompletionCriteria> {
        &self.completion_criteria
    }
    /// Consumes the builder and constructs a [`ImageClassificationJobConfig`](crate::types::ImageClassificationJobConfig).
    pub fn build(self) -> crate::types::ImageClassificationJobConfig {
        crate::types::ImageClassificationJobConfig {
            completion_criteria: self.completion_criteria,
        }
    }
}
