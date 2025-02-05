// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains error details for each device that didn't return a position.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct BatchGetDevicePositionError {
    /// <p>The ID of the device that didn't return a position.</p>
    pub device_id: ::std::option::Option<::std::string::String>,
    /// <p>Contains details related to the error code.</p>
    pub error: ::std::option::Option<crate::types::BatchItemError>,
}
impl BatchGetDevicePositionError {
    /// <p>The ID of the device that didn't return a position.</p>
    pub fn device_id(&self) -> ::std::option::Option<&str> {
        self.device_id.as_deref()
    }
    /// <p>Contains details related to the error code.</p>
    pub fn error(&self) -> ::std::option::Option<&crate::types::BatchItemError> {
        self.error.as_ref()
    }
}
impl BatchGetDevicePositionError {
    /// Creates a new builder-style object to manufacture [`BatchGetDevicePositionError`](crate::types::BatchGetDevicePositionError).
    pub fn builder() -> crate::types::builders::BatchGetDevicePositionErrorBuilder {
        crate::types::builders::BatchGetDevicePositionErrorBuilder::default()
    }
}

/// A builder for [`BatchGetDevicePositionError`](crate::types::BatchGetDevicePositionError).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct BatchGetDevicePositionErrorBuilder {
    pub(crate) device_id: ::std::option::Option<::std::string::String>,
    pub(crate) error: ::std::option::Option<crate::types::BatchItemError>,
}
impl BatchGetDevicePositionErrorBuilder {
    /// <p>The ID of the device that didn't return a position.</p>
    pub fn device_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.device_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the device that didn't return a position.</p>
    pub fn set_device_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.device_id = input;
        self
    }
    /// <p>The ID of the device that didn't return a position.</p>
    pub fn get_device_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.device_id
    }
    /// <p>Contains details related to the error code.</p>
    pub fn error(mut self, input: crate::types::BatchItemError) -> Self {
        self.error = ::std::option::Option::Some(input);
        self
    }
    /// <p>Contains details related to the error code.</p>
    pub fn set_error(mut self, input: ::std::option::Option<crate::types::BatchItemError>) -> Self {
        self.error = input;
        self
    }
    /// <p>Contains details related to the error code.</p>
    pub fn get_error(&self) -> &::std::option::Option<crate::types::BatchItemError> {
        &self.error
    }
    /// Consumes the builder and constructs a [`BatchGetDevicePositionError`](crate::types::BatchGetDevicePositionError).
    pub fn build(self) -> crate::types::BatchGetDevicePositionError {
        crate::types::BatchGetDevicePositionError {
            device_id: self.device_id,
            error: self.error,
        }
    }
}
