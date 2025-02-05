// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteStreamingSession`](crate::operation::delete_streaming_session::builders::DeleteStreamingSessionFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`client_token(impl ::std::convert::Into<String>)`](crate::operation::delete_streaming_session::builders::DeleteStreamingSessionFluentBuilder::client_token) / [`set_client_token(Option<String>)`](crate::operation::delete_streaming_session::builders::DeleteStreamingSessionFluentBuilder::set_client_token): <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. If you don’t specify a client token, the Amazon Web Services SDK automatically generates a client token and uses it for the request to ensure idempotency.</p>
    ///   - [`session_id(impl ::std::convert::Into<String>)`](crate::operation::delete_streaming_session::builders::DeleteStreamingSessionFluentBuilder::session_id) / [`set_session_id(Option<String>)`](crate::operation::delete_streaming_session::builders::DeleteStreamingSessionFluentBuilder::set_session_id): <p>The streaming session ID.</p>
    ///   - [`studio_id(impl ::std::convert::Into<String>)`](crate::operation::delete_streaming_session::builders::DeleteStreamingSessionFluentBuilder::studio_id) / [`set_studio_id(Option<String>)`](crate::operation::delete_streaming_session::builders::DeleteStreamingSessionFluentBuilder::set_studio_id): <p>The studio ID. </p>
    /// - On success, responds with [`DeleteStreamingSessionOutput`](crate::operation::delete_streaming_session::DeleteStreamingSessionOutput) with field(s):
    ///   - [`session(Option<StreamingSession>)`](crate::operation::delete_streaming_session::DeleteStreamingSessionOutput::session): <p>The session.</p>
    /// - On failure, responds with [`SdkError<DeleteStreamingSessionError>`](crate::operation::delete_streaming_session::DeleteStreamingSessionError)
    pub fn delete_streaming_session(&self) -> crate::operation::delete_streaming_session::builders::DeleteStreamingSessionFluentBuilder {
        crate::operation::delete_streaming_session::builders::DeleteStreamingSessionFluentBuilder::new(self.handle.clone())
    }
}
