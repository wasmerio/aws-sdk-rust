// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteEdgeConfiguration`](crate::operation::delete_edge_configuration::builders::DeleteEdgeConfigurationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`stream_name(impl ::std::convert::Into<String>)`](crate::operation::delete_edge_configuration::builders::DeleteEdgeConfigurationFluentBuilder::stream_name) / [`set_stream_name(Option<String>)`](crate::operation::delete_edge_configuration::builders::DeleteEdgeConfigurationFluentBuilder::set_stream_name): <p>The name of the stream from which to delete the edge configuration. Specify either the <code>StreamName</code> or the <code>StreamARN</code>.</p>
    ///   - [`stream_arn(impl ::std::convert::Into<String>)`](crate::operation::delete_edge_configuration::builders::DeleteEdgeConfigurationFluentBuilder::stream_arn) / [`set_stream_arn(Option<String>)`](crate::operation::delete_edge_configuration::builders::DeleteEdgeConfigurationFluentBuilder::set_stream_arn): <p>The Amazon Resource Name (ARN) of the stream. Specify either the <code>StreamName</code> or the <code>StreamARN</code>.</p>
    /// - On success, responds with [`DeleteEdgeConfigurationOutput`](crate::operation::delete_edge_configuration::DeleteEdgeConfigurationOutput)
    /// - On failure, responds with [`SdkError<DeleteEdgeConfigurationError>`](crate::operation::delete_edge_configuration::DeleteEdgeConfigurationError)
    pub fn delete_edge_configuration(&self) -> crate::operation::delete_edge_configuration::builders::DeleteEdgeConfigurationFluentBuilder {
        crate::operation::delete_edge_configuration::builders::DeleteEdgeConfigurationFluentBuilder::new(self.handle.clone())
    }
}
