// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`AssociateServiceRoleToAccount`](crate::operation::associate_service_role_to_account::builders::AssociateServiceRoleToAccountFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`role_arn(impl ::std::convert::Into<String>)`](crate::operation::associate_service_role_to_account::builders::AssociateServiceRoleToAccountFluentBuilder::role_arn) / [`set_role_arn(Option<String>)`](crate::operation::associate_service_role_to_account::builders::AssociateServiceRoleToAccountFluentBuilder::set_role_arn): The ARN of the service role you wish to associate with your account.
    /// - On success, responds with [`AssociateServiceRoleToAccountOutput`](crate::operation::associate_service_role_to_account::AssociateServiceRoleToAccountOutput) with field(s):
    ///   - [`associated_at(Option<String>)`](crate::operation::associate_service_role_to_account::AssociateServiceRoleToAccountOutput::associated_at): The time when the service role was associated with the account.
    /// - On failure, responds with [`SdkError<AssociateServiceRoleToAccountError>`](crate::operation::associate_service_role_to_account::AssociateServiceRoleToAccountError)
    pub fn associate_service_role_to_account(
        &self,
    ) -> crate::operation::associate_service_role_to_account::builders::AssociateServiceRoleToAccountFluentBuilder {
        crate::operation::associate_service_role_to_account::builders::AssociateServiceRoleToAccountFluentBuilder::new(self.handle.clone())
    }
}
