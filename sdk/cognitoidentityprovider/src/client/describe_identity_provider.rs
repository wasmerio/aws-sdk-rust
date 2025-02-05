// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeIdentityProvider`](crate::operation::describe_identity_provider::builders::DescribeIdentityProviderFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`user_pool_id(impl ::std::convert::Into<String>)`](crate::operation::describe_identity_provider::builders::DescribeIdentityProviderFluentBuilder::user_pool_id) / [`set_user_pool_id(Option<String>)`](crate::operation::describe_identity_provider::builders::DescribeIdentityProviderFluentBuilder::set_user_pool_id): <p>The user pool ID.</p>
    ///   - [`provider_name(impl ::std::convert::Into<String>)`](crate::operation::describe_identity_provider::builders::DescribeIdentityProviderFluentBuilder::provider_name) / [`set_provider_name(Option<String>)`](crate::operation::describe_identity_provider::builders::DescribeIdentityProviderFluentBuilder::set_provider_name): <p>The IdP name.</p>
    /// - On success, responds with [`DescribeIdentityProviderOutput`](crate::operation::describe_identity_provider::DescribeIdentityProviderOutput) with field(s):
    ///   - [`identity_provider(Option<IdentityProviderType>)`](crate::operation::describe_identity_provider::DescribeIdentityProviderOutput::identity_provider): <p>The identity provider details.</p>
    /// - On failure, responds with [`SdkError<DescribeIdentityProviderError>`](crate::operation::describe_identity_provider::DescribeIdentityProviderError)
    pub fn describe_identity_provider(&self) -> crate::operation::describe_identity_provider::builders::DescribeIdentityProviderFluentBuilder {
        crate::operation::describe_identity_provider::builders::DescribeIdentityProviderFluentBuilder::new(self.handle.clone())
    }
}
