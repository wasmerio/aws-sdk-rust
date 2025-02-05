// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteProfile`](crate::operation::delete_profile::builders::DeleteProfileFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`profile_id(impl ::std::convert::Into<String>)`](crate::operation::delete_profile::builders::DeleteProfileFluentBuilder::profile_id) / [`set_profile_id(Option<String>)`](crate::operation::delete_profile::builders::DeleteProfileFluentBuilder::set_profile_id): <p>The identifier of the profile that you are deleting.</p>
    /// - On success, responds with [`DeleteProfileOutput`](crate::operation::delete_profile::DeleteProfileOutput)
    /// - On failure, responds with [`SdkError<DeleteProfileError>`](crate::operation::delete_profile::DeleteProfileError)
    pub fn delete_profile(&self) -> crate::operation::delete_profile::builders::DeleteProfileFluentBuilder {
        crate::operation::delete_profile::builders::DeleteProfileFluentBuilder::new(self.handle.clone())
    }
}
