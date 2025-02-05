// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetAccountSettings`](crate::operation::get_account_settings::builders::GetAccountSettingsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`account_id(impl ::std::convert::Into<String>)`](crate::operation::get_account_settings::builders::GetAccountSettingsFluentBuilder::account_id) / [`set_account_id(Option<String>)`](crate::operation::get_account_settings::builders::GetAccountSettingsFluentBuilder::set_account_id): <p>The Amazon Chime account ID.</p>
    /// - On success, responds with [`GetAccountSettingsOutput`](crate::operation::get_account_settings::GetAccountSettingsOutput) with field(s):
    ///   - [`account_settings(Option<AccountSettings>)`](crate::operation::get_account_settings::GetAccountSettingsOutput::account_settings): <p>The Amazon Chime account settings.</p>
    /// - On failure, responds with [`SdkError<GetAccountSettingsError>`](crate::operation::get_account_settings::GetAccountSettingsError)
    pub fn get_account_settings(&self) -> crate::operation::get_account_settings::builders::GetAccountSettingsFluentBuilder {
        crate::operation::get_account_settings::builders::GetAccountSettingsFluentBuilder::new(self.handle.clone())
    }
}
