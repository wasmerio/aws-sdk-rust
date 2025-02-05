// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetAccountStatus`](crate::operation::get_account_status::builders::GetAccountStatusFluentBuilder) operation.
    ///
    /// - The fluent builder takes no input, just [`send`](crate::operation::get_account_status::builders::GetAccountStatusFluentBuilder::send) it.
    /// - On success, responds with [`GetAccountStatusOutput`](crate::operation::get_account_status::GetAccountStatusOutput) with field(s):
    ///   - [`status(Option<AccountStatus>)`](crate::operation::get_account_status::GetAccountStatusOutput::status): <p> The status of the Amazon Web Services account. </p>
    /// - On failure, responds with [`SdkError<GetAccountStatusError>`](crate::operation::get_account_status::GetAccountStatusError)
    pub fn get_account_status(&self) -> crate::operation::get_account_status::builders::GetAccountStatusFluentBuilder {
        crate::operation::get_account_status::builders::GetAccountStatusFluentBuilder::new(self.handle.clone())
    }
}
