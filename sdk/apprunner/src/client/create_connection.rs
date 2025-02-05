// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateConnection`](crate::operation::create_connection::builders::CreateConnectionFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`connection_name(impl ::std::convert::Into<String>)`](crate::operation::create_connection::builders::CreateConnectionFluentBuilder::connection_name) / [`set_connection_name(Option<String>)`](crate::operation::create_connection::builders::CreateConnectionFluentBuilder::set_connection_name): <p>A name for the new connection. It must be unique across all App Runner connections for the Amazon Web Services account in the Amazon Web Services Region.</p>
    ///   - [`provider_type(ProviderType)`](crate::operation::create_connection::builders::CreateConnectionFluentBuilder::provider_type) / [`set_provider_type(Option<ProviderType>)`](crate::operation::create_connection::builders::CreateConnectionFluentBuilder::set_provider_type): <p>The source repository provider.</p>
    ///   - [`tags(Vec<Tag>)`](crate::operation::create_connection::builders::CreateConnectionFluentBuilder::tags) / [`set_tags(Option<Vec<Tag>>)`](crate::operation::create_connection::builders::CreateConnectionFluentBuilder::set_tags): <p>A list of metadata items that you can associate with your connection resource. A tag is a key-value pair.</p>
    /// - On success, responds with [`CreateConnectionOutput`](crate::operation::create_connection::CreateConnectionOutput) with field(s):
    ///   - [`connection(Option<Connection>)`](crate::operation::create_connection::CreateConnectionOutput::connection): <p>A description of the App Runner connection that's created by this request.</p>
    /// - On failure, responds with [`SdkError<CreateConnectionError>`](crate::operation::create_connection::CreateConnectionError)
    pub fn create_connection(&self) -> crate::operation::create_connection::builders::CreateConnectionFluentBuilder {
        crate::operation::create_connection::builders::CreateConnectionFluentBuilder::new(self.handle.clone())
    }
}
