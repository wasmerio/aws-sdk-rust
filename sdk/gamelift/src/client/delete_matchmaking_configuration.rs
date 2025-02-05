// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteMatchmakingConfiguration`](crate::operation::delete_matchmaking_configuration::builders::DeleteMatchmakingConfigurationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`name(impl ::std::convert::Into<String>)`](crate::operation::delete_matchmaking_configuration::builders::DeleteMatchmakingConfigurationFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::delete_matchmaking_configuration::builders::DeleteMatchmakingConfigurationFluentBuilder::set_name): <p>A unique identifier for the matchmaking configuration. You can use either the configuration name or ARN value.</p>
    /// - On success, responds with [`DeleteMatchmakingConfigurationOutput`](crate::operation::delete_matchmaking_configuration::DeleteMatchmakingConfigurationOutput)
    /// - On failure, responds with [`SdkError<DeleteMatchmakingConfigurationError>`](crate::operation::delete_matchmaking_configuration::DeleteMatchmakingConfigurationError)
    pub fn delete_matchmaking_configuration(
        &self,
    ) -> crate::operation::delete_matchmaking_configuration::builders::DeleteMatchmakingConfigurationFluentBuilder {
        crate::operation::delete_matchmaking_configuration::builders::DeleteMatchmakingConfigurationFluentBuilder::new(self.handle.clone())
    }
}
