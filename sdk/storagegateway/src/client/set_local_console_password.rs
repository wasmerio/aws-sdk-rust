// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`SetLocalConsolePassword`](crate::operation::set_local_console_password::builders::SetLocalConsolePasswordFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`gateway_arn(impl ::std::convert::Into<String>)`](crate::operation::set_local_console_password::builders::SetLocalConsolePasswordFluentBuilder::gateway_arn) / [`set_gateway_arn(Option<String>)`](crate::operation::set_local_console_password::builders::SetLocalConsolePasswordFluentBuilder::set_gateway_arn): <p>The Amazon Resource Name (ARN) of the gateway. Use the <code>ListGateways</code> operation to return a list of gateways for your account and Amazon Web Services Region.</p>
    ///   - [`local_console_password(impl ::std::convert::Into<String>)`](crate::operation::set_local_console_password::builders::SetLocalConsolePasswordFluentBuilder::local_console_password) / [`set_local_console_password(Option<String>)`](crate::operation::set_local_console_password::builders::SetLocalConsolePasswordFluentBuilder::set_local_console_password): <p>The password you want to set for your VM local console.</p>
    /// - On success, responds with [`SetLocalConsolePasswordOutput`](crate::operation::set_local_console_password::SetLocalConsolePasswordOutput) with field(s):
    ///   - [`gateway_arn(Option<String>)`](crate::operation::set_local_console_password::SetLocalConsolePasswordOutput::gateway_arn): <p>The Amazon Resource Name (ARN) of the gateway. Use the <code>ListGateways</code> operation to return a list of gateways for your account and Amazon Web Services Region.</p>
    /// - On failure, responds with [`SdkError<SetLocalConsolePasswordError>`](crate::operation::set_local_console_password::SetLocalConsolePasswordError)
    pub fn set_local_console_password(&self) -> crate::operation::set_local_console_password::builders::SetLocalConsolePasswordFluentBuilder {
        crate::operation::set_local_console_password::builders::SetLocalConsolePasswordFluentBuilder::new(self.handle.clone())
    }
}
