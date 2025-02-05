// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`SendSSHPublicKey`](crate::operation::send_ssh_public_key::builders::SendSSHPublicKeyFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`instance_id(impl ::std::convert::Into<String>)`](crate::operation::send_ssh_public_key::builders::SendSSHPublicKeyFluentBuilder::instance_id) / [`set_instance_id(Option<String>)`](crate::operation::send_ssh_public_key::builders::SendSSHPublicKeyFluentBuilder::set_instance_id): <p>The ID of the EC2 instance.</p>
    ///   - [`instance_os_user(impl ::std::convert::Into<String>)`](crate::operation::send_ssh_public_key::builders::SendSSHPublicKeyFluentBuilder::instance_os_user) / [`set_instance_os_user(Option<String>)`](crate::operation::send_ssh_public_key::builders::SendSSHPublicKeyFluentBuilder::set_instance_os_user): <p>The OS user on the EC2 instance for whom the key can be used to authenticate.</p>
    ///   - [`ssh_public_key(impl ::std::convert::Into<String>)`](crate::operation::send_ssh_public_key::builders::SendSSHPublicKeyFluentBuilder::ssh_public_key) / [`set_ssh_public_key(Option<String>)`](crate::operation::send_ssh_public_key::builders::SendSSHPublicKeyFluentBuilder::set_ssh_public_key): <p>The public key material. To use the public key, you must have the matching private key.</p>
    ///   - [`availability_zone(impl ::std::convert::Into<String>)`](crate::operation::send_ssh_public_key::builders::SendSSHPublicKeyFluentBuilder::availability_zone) / [`set_availability_zone(Option<String>)`](crate::operation::send_ssh_public_key::builders::SendSSHPublicKeyFluentBuilder::set_availability_zone): <p>The Availability Zone in which the EC2 instance was launched.</p>
    /// - On success, responds with [`SendSshPublicKeyOutput`](crate::operation::send_ssh_public_key::SendSshPublicKeyOutput) with field(s):
    ///   - [`request_id(Option<String>)`](crate::operation::send_ssh_public_key::SendSshPublicKeyOutput::request_id): <p>The ID of the request. Please provide this ID when contacting AWS Support for assistance.</p>
    ///   - [`success(bool)`](crate::operation::send_ssh_public_key::SendSshPublicKeyOutput::success): <p>Is true if the request succeeds and an error otherwise.</p>
    /// - On failure, responds with [`SdkError<SendSSHPublicKeyError>`](crate::operation::send_ssh_public_key::SendSSHPublicKeyError)
    pub fn send_ssh_public_key(&self) -> crate::operation::send_ssh_public_key::builders::SendSSHPublicKeyFluentBuilder {
        crate::operation::send_ssh_public_key::builders::SendSSHPublicKeyFluentBuilder::new(self.handle.clone())
    }
}
