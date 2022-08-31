// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
use std::fmt::Write;

/// See [`SendSerialConsoleSshPublicKeyInput`](crate::input::SendSerialConsoleSshPublicKeyInput).
pub mod send_serial_console_ssh_public_key_input {

    /// A builder for [`SendSerialConsoleSshPublicKeyInput`](crate::input::SendSerialConsoleSshPublicKeyInput).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) instance_id: std::option::Option<std::string::String>,
        pub(crate) serial_port: std::option::Option<i32>,
        pub(crate) ssh_public_key: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The ID of the EC2 instance.</p>
        pub fn instance_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.instance_id = Some(input.into());
            self
        }
        /// <p>The ID of the EC2 instance.</p>
        pub fn set_instance_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.instance_id = input;
            self
        }
        /// <p>The serial port of the EC2 instance. Currently only port 0 is supported.</p>
        /// <p>Default: 0</p>
        pub fn serial_port(mut self, input: i32) -> Self {
            self.serial_port = Some(input);
            self
        }
        /// <p>The serial port of the EC2 instance. Currently only port 0 is supported.</p>
        /// <p>Default: 0</p>
        pub fn set_serial_port(mut self, input: std::option::Option<i32>) -> Self {
            self.serial_port = input;
            self
        }
        /// <p>The public key material. To use the public key, you must have the matching private key. For information about the supported key formats and lengths, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-key-pairs.html#how-to-generate-your-own-key-and-import-it-to-aws">Requirements for key pairs</a> in the <i>Amazon EC2 User Guide</i>.</p>
        pub fn ssh_public_key(mut self, input: impl Into<std::string::String>) -> Self {
            self.ssh_public_key = Some(input.into());
            self
        }
        /// <p>The public key material. To use the public key, you must have the matching private key. For information about the supported key formats and lengths, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-key-pairs.html#how-to-generate-your-own-key-and-import-it-to-aws">Requirements for key pairs</a> in the <i>Amazon EC2 User Guide</i>.</p>
        pub fn set_ssh_public_key(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.ssh_public_key = input;
            self
        }
        /// Consumes the builder and constructs a [`SendSerialConsoleSshPublicKeyInput`](crate::input::SendSerialConsoleSshPublicKeyInput).
        pub fn build(
            self,
        ) -> Result<
            crate::input::SendSerialConsoleSshPublicKeyInput,
            aws_smithy_http::operation::BuildError,
        > {
            Ok(crate::input::SendSerialConsoleSshPublicKeyInput {
                instance_id: self.instance_id,
                serial_port: self.serial_port.unwrap_or_default(),
                ssh_public_key: self.ssh_public_key,
            })
        }
    }
}
#[doc(hidden)]
pub type SendSerialConsoleSshPublicKeyInputOperationOutputAlias =
    crate::operation::SendSerialConsoleSSHPublicKey;
#[doc(hidden)]
pub type SendSerialConsoleSshPublicKeyInputOperationRetryAlias =
    aws_http::retry::AwsErrorRetryPolicy;
impl SendSerialConsoleSshPublicKeyInput {
    /// Consumes the builder and constructs an Operation<[`SendSerialConsoleSSHPublicKey`](crate::operation::SendSerialConsoleSSHPublicKey)>
    #[allow(unused_mut)]
    #[allow(clippy::let_and_return)]
    #[allow(clippy::needless_borrow)]
    pub async fn make_operation(
        &self,
        _config: &crate::config::Config,
    ) -> std::result::Result<
        aws_smithy_http::operation::Operation<
            crate::operation::SendSerialConsoleSSHPublicKey,
            aws_http::retry::AwsErrorRetryPolicy,
        >,
        aws_smithy_http::operation::BuildError,
    > {
        let mut request = {
            fn uri_base(
                _input: &crate::input::SendSerialConsoleSshPublicKeyInput,
                output: &mut String,
            ) -> Result<(), aws_smithy_http::operation::BuildError> {
                write!(output, "/").expect("formatting should succeed");
                Ok(())
            }
            #[allow(clippy::unnecessary_wraps)]
            fn update_http_builder(
                input: &crate::input::SendSerialConsoleSshPublicKeyInput,
                builder: http::request::Builder,
            ) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError>
            {
                let mut uri = String::new();
                uri_base(input, &mut uri)?;
                Ok(builder.method("POST").uri(uri))
            }
            let mut builder = update_http_builder(&self, http::request::Builder::new())?;
            builder = aws_smithy_http::header::set_request_header_if_absent(
                builder,
                http::header::CONTENT_TYPE,
                "application/x-amz-json-1.1",
            );
            builder = aws_smithy_http::header::set_request_header_if_absent(
                builder,
                http::header::HeaderName::from_static("x-amz-target"),
                "AWSEC2InstanceConnectService.SendSerialConsoleSSHPublicKey",
            );
            builder
        };
        let mut properties = aws_smithy_http::property_bag::SharedPropertyBag::new();
        #[allow(clippy::useless_conversion)]let body = aws_smithy_http::body::SdkBody::from(
            crate::operation_ser::serialize_operation_crate_operation_send_serial_console_ssh_public_key(&self)?
        );
        if let Some(content_length) = body.content_length() {
            request = aws_smithy_http::header::set_request_header_if_absent(
                request,
                http::header::CONTENT_LENGTH,
                content_length,
            );
        }
        let request = request.body(body).expect("should be valid request");
        let mut request = aws_smithy_http::operation::Request::from_parts(request, properties);
        request
            .properties_mut()
            .insert(aws_smithy_http::http_versions::DEFAULT_HTTP_VERSION_LIST.clone());
        let mut user_agent = aws_http::user_agent::AwsUserAgent::new_from_environment(
            aws_types::os_shim_internal::Env::real(),
            crate::API_METADATA.clone(),
        );
        if let Some(app_name) = _config.app_name() {
            user_agent = user_agent.with_app_name(app_name.clone());
        }
        request.properties_mut().insert(user_agent);
        let mut signing_config = aws_sig_auth::signer::OperationSigningConfig::default_config();
        request.properties_mut().insert(signing_config);
        request
            .properties_mut()
            .insert(aws_types::SigningService::from_static(
                _config.signing_service(),
            ));
        if let Some(region) = &_config.region {
            request
                .properties_mut()
                .insert(aws_types::region::SigningRegion::from(region.clone()));
        }
        let endpoint_params = aws_endpoint::Params::new(_config.region.clone());
        request
            .properties_mut()
            .insert::<aws_smithy_http::endpoint::Result>(
                _config.endpoint_resolver.resolve_endpoint(&endpoint_params),
            );
        if let Some(region) = &_config.region {
            request.properties_mut().insert(region.clone());
        }
        aws_http::auth::set_provider(
            &mut request.properties_mut(),
            _config.credentials_provider.clone(),
        );
        let op = aws_smithy_http::operation::Operation::new(
            request,
            crate::operation::SendSerialConsoleSSHPublicKey::new(),
        )
        .with_metadata(aws_smithy_http::operation::Metadata::new(
            "SendSerialConsoleSSHPublicKey",
            "ec2instanceconnect",
        ));
        let op = op.with_retry_policy(aws_http::retry::AwsErrorRetryPolicy::new());
        Ok(op)
    }
    /// Creates a new builder-style object to manufacture [`SendSerialConsoleSshPublicKeyInput`](crate::input::SendSerialConsoleSshPublicKeyInput).
    pub fn builder() -> crate::input::send_serial_console_ssh_public_key_input::Builder {
        crate::input::send_serial_console_ssh_public_key_input::Builder::default()
    }
}

/// See [`SendSshPublicKeyInput`](crate::input::SendSshPublicKeyInput).
pub mod send_ssh_public_key_input {

    /// A builder for [`SendSshPublicKeyInput`](crate::input::SendSshPublicKeyInput).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) instance_id: std::option::Option<std::string::String>,
        pub(crate) instance_os_user: std::option::Option<std::string::String>,
        pub(crate) ssh_public_key: std::option::Option<std::string::String>,
        pub(crate) availability_zone: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The ID of the EC2 instance.</p>
        pub fn instance_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.instance_id = Some(input.into());
            self
        }
        /// <p>The ID of the EC2 instance.</p>
        pub fn set_instance_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.instance_id = input;
            self
        }
        /// <p>The OS user on the EC2 instance for whom the key can be used to authenticate.</p>
        pub fn instance_os_user(mut self, input: impl Into<std::string::String>) -> Self {
            self.instance_os_user = Some(input.into());
            self
        }
        /// <p>The OS user on the EC2 instance for whom the key can be used to authenticate.</p>
        pub fn set_instance_os_user(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.instance_os_user = input;
            self
        }
        /// <p>The public key material. To use the public key, you must have the matching private key.</p>
        pub fn ssh_public_key(mut self, input: impl Into<std::string::String>) -> Self {
            self.ssh_public_key = Some(input.into());
            self
        }
        /// <p>The public key material. To use the public key, you must have the matching private key.</p>
        pub fn set_ssh_public_key(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.ssh_public_key = input;
            self
        }
        /// <p>The Availability Zone in which the EC2 instance was launched.</p>
        pub fn availability_zone(mut self, input: impl Into<std::string::String>) -> Self {
            self.availability_zone = Some(input.into());
            self
        }
        /// <p>The Availability Zone in which the EC2 instance was launched.</p>
        pub fn set_availability_zone(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.availability_zone = input;
            self
        }
        /// Consumes the builder and constructs a [`SendSshPublicKeyInput`](crate::input::SendSshPublicKeyInput).
        pub fn build(
            self,
        ) -> Result<crate::input::SendSshPublicKeyInput, aws_smithy_http::operation::BuildError>
        {
            Ok(crate::input::SendSshPublicKeyInput {
                instance_id: self.instance_id,
                instance_os_user: self.instance_os_user,
                ssh_public_key: self.ssh_public_key,
                availability_zone: self.availability_zone,
            })
        }
    }
}
#[doc(hidden)]
pub type SendSshPublicKeyInputOperationOutputAlias = crate::operation::SendSSHPublicKey;
#[doc(hidden)]
pub type SendSshPublicKeyInputOperationRetryAlias = aws_http::retry::AwsErrorRetryPolicy;
impl SendSshPublicKeyInput {
    /// Consumes the builder and constructs an Operation<[`SendSSHPublicKey`](crate::operation::SendSSHPublicKey)>
    #[allow(unused_mut)]
    #[allow(clippy::let_and_return)]
    #[allow(clippy::needless_borrow)]
    pub async fn make_operation(
        &self,
        _config: &crate::config::Config,
    ) -> std::result::Result<
        aws_smithy_http::operation::Operation<
            crate::operation::SendSSHPublicKey,
            aws_http::retry::AwsErrorRetryPolicy,
        >,
        aws_smithy_http::operation::BuildError,
    > {
        let mut request = {
            fn uri_base(
                _input: &crate::input::SendSshPublicKeyInput,
                output: &mut String,
            ) -> Result<(), aws_smithy_http::operation::BuildError> {
                write!(output, "/").expect("formatting should succeed");
                Ok(())
            }
            #[allow(clippy::unnecessary_wraps)]
            fn update_http_builder(
                input: &crate::input::SendSshPublicKeyInput,
                builder: http::request::Builder,
            ) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError>
            {
                let mut uri = String::new();
                uri_base(input, &mut uri)?;
                Ok(builder.method("POST").uri(uri))
            }
            let mut builder = update_http_builder(&self, http::request::Builder::new())?;
            builder = aws_smithy_http::header::set_request_header_if_absent(
                builder,
                http::header::CONTENT_TYPE,
                "application/x-amz-json-1.1",
            );
            builder = aws_smithy_http::header::set_request_header_if_absent(
                builder,
                http::header::HeaderName::from_static("x-amz-target"),
                "AWSEC2InstanceConnectService.SendSSHPublicKey",
            );
            builder
        };
        let mut properties = aws_smithy_http::property_bag::SharedPropertyBag::new();
        #[allow(clippy::useless_conversion)]
        let body = aws_smithy_http::body::SdkBody::from(
            crate::operation_ser::serialize_operation_crate_operation_send_ssh_public_key(&self)?,
        );
        if let Some(content_length) = body.content_length() {
            request = aws_smithy_http::header::set_request_header_if_absent(
                request,
                http::header::CONTENT_LENGTH,
                content_length,
            );
        }
        let request = request.body(body).expect("should be valid request");
        let mut request = aws_smithy_http::operation::Request::from_parts(request, properties);
        request
            .properties_mut()
            .insert(aws_smithy_http::http_versions::DEFAULT_HTTP_VERSION_LIST.clone());
        let mut user_agent = aws_http::user_agent::AwsUserAgent::new_from_environment(
            aws_types::os_shim_internal::Env::real(),
            crate::API_METADATA.clone(),
        );
        if let Some(app_name) = _config.app_name() {
            user_agent = user_agent.with_app_name(app_name.clone());
        }
        request.properties_mut().insert(user_agent);
        let mut signing_config = aws_sig_auth::signer::OperationSigningConfig::default_config();
        request.properties_mut().insert(signing_config);
        request
            .properties_mut()
            .insert(aws_types::SigningService::from_static(
                _config.signing_service(),
            ));
        if let Some(region) = &_config.region {
            request
                .properties_mut()
                .insert(aws_types::region::SigningRegion::from(region.clone()));
        }
        let endpoint_params = aws_endpoint::Params::new(_config.region.clone());
        request
            .properties_mut()
            .insert::<aws_smithy_http::endpoint::Result>(
                _config.endpoint_resolver.resolve_endpoint(&endpoint_params),
            );
        if let Some(region) = &_config.region {
            request.properties_mut().insert(region.clone());
        }
        aws_http::auth::set_provider(
            &mut request.properties_mut(),
            _config.credentials_provider.clone(),
        );
        let op = aws_smithy_http::operation::Operation::new(
            request,
            crate::operation::SendSSHPublicKey::new(),
        )
        .with_metadata(aws_smithy_http::operation::Metadata::new(
            "SendSSHPublicKey",
            "ec2instanceconnect",
        ));
        let op = op.with_retry_policy(aws_http::retry::AwsErrorRetryPolicy::new());
        Ok(op)
    }
    /// Creates a new builder-style object to manufacture [`SendSshPublicKeyInput`](crate::input::SendSshPublicKeyInput).
    pub fn builder() -> crate::input::send_ssh_public_key_input::Builder {
        crate::input::send_ssh_public_key_input::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct SendSshPublicKeyInput {
    /// <p>The ID of the EC2 instance.</p>
    #[doc(hidden)]
    pub instance_id: std::option::Option<std::string::String>,
    /// <p>The OS user on the EC2 instance for whom the key can be used to authenticate.</p>
    #[doc(hidden)]
    pub instance_os_user: std::option::Option<std::string::String>,
    /// <p>The public key material. To use the public key, you must have the matching private key.</p>
    #[doc(hidden)]
    pub ssh_public_key: std::option::Option<std::string::String>,
    /// <p>The Availability Zone in which the EC2 instance was launched.</p>
    #[doc(hidden)]
    pub availability_zone: std::option::Option<std::string::String>,
}
impl SendSshPublicKeyInput {
    /// <p>The ID of the EC2 instance.</p>
    pub fn instance_id(&self) -> std::option::Option<&str> {
        self.instance_id.as_deref()
    }
    /// <p>The OS user on the EC2 instance for whom the key can be used to authenticate.</p>
    pub fn instance_os_user(&self) -> std::option::Option<&str> {
        self.instance_os_user.as_deref()
    }
    /// <p>The public key material. To use the public key, you must have the matching private key.</p>
    pub fn ssh_public_key(&self) -> std::option::Option<&str> {
        self.ssh_public_key.as_deref()
    }
    /// <p>The Availability Zone in which the EC2 instance was launched.</p>
    pub fn availability_zone(&self) -> std::option::Option<&str> {
        self.availability_zone.as_deref()
    }
}
impl std::fmt::Debug for SendSshPublicKeyInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("SendSshPublicKeyInput");
        formatter.field("instance_id", &self.instance_id);
        formatter.field("instance_os_user", &self.instance_os_user);
        formatter.field("ssh_public_key", &self.ssh_public_key);
        formatter.field("availability_zone", &self.availability_zone);
        formatter.finish()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct SendSerialConsoleSshPublicKeyInput {
    /// <p>The ID of the EC2 instance.</p>
    #[doc(hidden)]
    pub instance_id: std::option::Option<std::string::String>,
    /// <p>The serial port of the EC2 instance. Currently only port 0 is supported.</p>
    /// <p>Default: 0</p>
    #[doc(hidden)]
    pub serial_port: i32,
    /// <p>The public key material. To use the public key, you must have the matching private key. For information about the supported key formats and lengths, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-key-pairs.html#how-to-generate-your-own-key-and-import-it-to-aws">Requirements for key pairs</a> in the <i>Amazon EC2 User Guide</i>.</p>
    #[doc(hidden)]
    pub ssh_public_key: std::option::Option<std::string::String>,
}
impl SendSerialConsoleSshPublicKeyInput {
    /// <p>The ID of the EC2 instance.</p>
    pub fn instance_id(&self) -> std::option::Option<&str> {
        self.instance_id.as_deref()
    }
    /// <p>The serial port of the EC2 instance. Currently only port 0 is supported.</p>
    /// <p>Default: 0</p>
    pub fn serial_port(&self) -> i32 {
        self.serial_port
    }
    /// <p>The public key material. To use the public key, you must have the matching private key. For information about the supported key formats and lengths, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-key-pairs.html#how-to-generate-your-own-key-and-import-it-to-aws">Requirements for key pairs</a> in the <i>Amazon EC2 User Guide</i>.</p>
    pub fn ssh_public_key(&self) -> std::option::Option<&str> {
        self.ssh_public_key.as_deref()
    }
}
impl std::fmt::Debug for SendSerialConsoleSshPublicKeyInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("SendSerialConsoleSshPublicKeyInput");
        formatter.field("instance_id", &self.instance_id);
        formatter.field("serial_port", &self.serial_port);
        formatter.field("ssh_public_key", &self.ssh_public_key);
        formatter.finish()
    }
}
