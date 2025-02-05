// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct PutInboundDmarcSettingsInput {
    /// <p>The ID of the organization that you are applying the DMARC policy to.</p>
    pub organization_id: ::std::option::Option<::std::string::String>,
    /// <p>Enforces or suspends a policy after it's applied.</p>
    pub enforced: ::std::option::Option<bool>,
}
impl PutInboundDmarcSettingsInput {
    /// <p>The ID of the organization that you are applying the DMARC policy to.</p>
    pub fn organization_id(&self) -> ::std::option::Option<&str> {
        self.organization_id.as_deref()
    }
    /// <p>Enforces or suspends a policy after it's applied.</p>
    pub fn enforced(&self) -> ::std::option::Option<bool> {
        self.enforced
    }
}
impl PutInboundDmarcSettingsInput {
    /// Creates a new builder-style object to manufacture [`PutInboundDmarcSettingsInput`](crate::operation::put_inbound_dmarc_settings::PutInboundDmarcSettingsInput).
    pub fn builder() -> crate::operation::put_inbound_dmarc_settings::builders::PutInboundDmarcSettingsInputBuilder {
        crate::operation::put_inbound_dmarc_settings::builders::PutInboundDmarcSettingsInputBuilder::default()
    }
}

/// A builder for [`PutInboundDmarcSettingsInput`](crate::operation::put_inbound_dmarc_settings::PutInboundDmarcSettingsInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct PutInboundDmarcSettingsInputBuilder {
    pub(crate) organization_id: ::std::option::Option<::std::string::String>,
    pub(crate) enforced: ::std::option::Option<bool>,
}
impl PutInboundDmarcSettingsInputBuilder {
    /// <p>The ID of the organization that you are applying the DMARC policy to.</p>
    pub fn organization_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.organization_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the organization that you are applying the DMARC policy to.</p>
    pub fn set_organization_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.organization_id = input;
        self
    }
    /// <p>The ID of the organization that you are applying the DMARC policy to.</p>
    pub fn get_organization_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.organization_id
    }
    /// <p>Enforces or suspends a policy after it's applied.</p>
    pub fn enforced(mut self, input: bool) -> Self {
        self.enforced = ::std::option::Option::Some(input);
        self
    }
    /// <p>Enforces or suspends a policy after it's applied.</p>
    pub fn set_enforced(mut self, input: ::std::option::Option<bool>) -> Self {
        self.enforced = input;
        self
    }
    /// <p>Enforces or suspends a policy after it's applied.</p>
    pub fn get_enforced(&self) -> &::std::option::Option<bool> {
        &self.enforced
    }
    /// Consumes the builder and constructs a [`PutInboundDmarcSettingsInput`](crate::operation::put_inbound_dmarc_settings::PutInboundDmarcSettingsInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::put_inbound_dmarc_settings::PutInboundDmarcSettingsInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::put_inbound_dmarc_settings::PutInboundDmarcSettingsInput {
            organization_id: self.organization_id,
            enforced: self.enforced,
        })
    }
}
