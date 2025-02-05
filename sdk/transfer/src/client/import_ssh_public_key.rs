// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ImportSshPublicKey`](crate::operation::import_ssh_public_key::builders::ImportSshPublicKeyFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`server_id(impl ::std::convert::Into<String>)`](crate::operation::import_ssh_public_key::builders::ImportSshPublicKeyFluentBuilder::server_id) / [`set_server_id(Option<String>)`](crate::operation::import_ssh_public_key::builders::ImportSshPublicKeyFluentBuilder::set_server_id): <p>A system-assigned unique identifier for a server.</p>
    ///   - [`ssh_public_key_body(impl ::std::convert::Into<String>)`](crate::operation::import_ssh_public_key::builders::ImportSshPublicKeyFluentBuilder::ssh_public_key_body) / [`set_ssh_public_key_body(Option<String>)`](crate::operation::import_ssh_public_key::builders::ImportSshPublicKeyFluentBuilder::set_ssh_public_key_body): <p>The public key portion of an SSH key pair.</p>  <p>Transfer Family accepts RSA, ECDSA, and ED25519 keys.</p>
    ///   - [`user_name(impl ::std::convert::Into<String>)`](crate::operation::import_ssh_public_key::builders::ImportSshPublicKeyFluentBuilder::user_name) / [`set_user_name(Option<String>)`](crate::operation::import_ssh_public_key::builders::ImportSshPublicKeyFluentBuilder::set_user_name): <p>The name of the Transfer Family user that is assigned to one or more servers.</p>
    /// - On success, responds with [`ImportSshPublicKeyOutput`](crate::operation::import_ssh_public_key::ImportSshPublicKeyOutput) with field(s):
    ///   - [`server_id(Option<String>)`](crate::operation::import_ssh_public_key::ImportSshPublicKeyOutput::server_id): <p>A system-assigned unique identifier for a server.</p>
    ///   - [`ssh_public_key_id(Option<String>)`](crate::operation::import_ssh_public_key::ImportSshPublicKeyOutput::ssh_public_key_id): <p>The name given to a public key by the system that was imported.</p>
    ///   - [`user_name(Option<String>)`](crate::operation::import_ssh_public_key::ImportSshPublicKeyOutput::user_name): <p>A user name assigned to the <code>ServerID</code> value that you specified.</p>
    /// - On failure, responds with [`SdkError<ImportSshPublicKeyError>`](crate::operation::import_ssh_public_key::ImportSshPublicKeyError)
    pub fn import_ssh_public_key(&self) -> crate::operation::import_ssh_public_key::builders::ImportSshPublicKeyFluentBuilder {
        crate::operation::import_ssh_public_key::builders::ImportSshPublicKeyFluentBuilder::new(self.handle.clone())
    }
}
