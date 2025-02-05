// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Specifies the set of parameters needed to perform the dynamic transform.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DynamicTransform {
    /// <p>Specifies the name of the dynamic transform.</p>
    pub name: ::std::option::Option<::std::string::String>,
    /// <p>Specifies the name of the dynamic transform as it appears in the Glue Studio visual editor.</p>
    pub transform_name: ::std::option::Option<::std::string::String>,
    /// <p>Specifies the inputs for the dynamic transform that are required.</p>
    pub inputs: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>Specifies the parameters of the dynamic transform.</p>
    pub parameters: ::std::option::Option<::std::vec::Vec<crate::types::TransformConfigParameter>>,
    /// <p>Specifies the name of the function of the dynamic transform.</p>
    pub function_name: ::std::option::Option<::std::string::String>,
    /// <p>Specifies the path of the dynamic transform source and config files.</p>
    pub path: ::std::option::Option<::std::string::String>,
    /// <p>This field is not used and will be deprecated in future release.</p>
    pub version: ::std::option::Option<::std::string::String>,
    /// <p>Specifies the data schema for the dynamic transform.</p>
    pub output_schemas: ::std::option::Option<::std::vec::Vec<crate::types::GlueSchema>>,
}
impl DynamicTransform {
    /// <p>Specifies the name of the dynamic transform.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>Specifies the name of the dynamic transform as it appears in the Glue Studio visual editor.</p>
    pub fn transform_name(&self) -> ::std::option::Option<&str> {
        self.transform_name.as_deref()
    }
    /// <p>Specifies the inputs for the dynamic transform that are required.</p>
    pub fn inputs(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.inputs.as_deref()
    }
    /// <p>Specifies the parameters of the dynamic transform.</p>
    pub fn parameters(&self) -> ::std::option::Option<&[crate::types::TransformConfigParameter]> {
        self.parameters.as_deref()
    }
    /// <p>Specifies the name of the function of the dynamic transform.</p>
    pub fn function_name(&self) -> ::std::option::Option<&str> {
        self.function_name.as_deref()
    }
    /// <p>Specifies the path of the dynamic transform source and config files.</p>
    pub fn path(&self) -> ::std::option::Option<&str> {
        self.path.as_deref()
    }
    /// <p>This field is not used and will be deprecated in future release.</p>
    pub fn version(&self) -> ::std::option::Option<&str> {
        self.version.as_deref()
    }
    /// <p>Specifies the data schema for the dynamic transform.</p>
    pub fn output_schemas(&self) -> ::std::option::Option<&[crate::types::GlueSchema]> {
        self.output_schemas.as_deref()
    }
}
impl DynamicTransform {
    /// Creates a new builder-style object to manufacture [`DynamicTransform`](crate::types::DynamicTransform).
    pub fn builder() -> crate::types::builders::DynamicTransformBuilder {
        crate::types::builders::DynamicTransformBuilder::default()
    }
}

/// A builder for [`DynamicTransform`](crate::types::DynamicTransform).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct DynamicTransformBuilder {
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) transform_name: ::std::option::Option<::std::string::String>,
    pub(crate) inputs: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) parameters: ::std::option::Option<::std::vec::Vec<crate::types::TransformConfigParameter>>,
    pub(crate) function_name: ::std::option::Option<::std::string::String>,
    pub(crate) path: ::std::option::Option<::std::string::String>,
    pub(crate) version: ::std::option::Option<::std::string::String>,
    pub(crate) output_schemas: ::std::option::Option<::std::vec::Vec<crate::types::GlueSchema>>,
}
impl DynamicTransformBuilder {
    /// <p>Specifies the name of the dynamic transform.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Specifies the name of the dynamic transform.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>Specifies the name of the dynamic transform.</p>
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.name
    }
    /// <p>Specifies the name of the dynamic transform as it appears in the Glue Studio visual editor.</p>
    pub fn transform_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.transform_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Specifies the name of the dynamic transform as it appears in the Glue Studio visual editor.</p>
    pub fn set_transform_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.transform_name = input;
        self
    }
    /// <p>Specifies the name of the dynamic transform as it appears in the Glue Studio visual editor.</p>
    pub fn get_transform_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.transform_name
    }
    /// Appends an item to `inputs`.
    ///
    /// To override the contents of this collection use [`set_inputs`](Self::set_inputs).
    ///
    /// <p>Specifies the inputs for the dynamic transform that are required.</p>
    pub fn inputs(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.inputs.unwrap_or_default();
        v.push(input.into());
        self.inputs = ::std::option::Option::Some(v);
        self
    }
    /// <p>Specifies the inputs for the dynamic transform that are required.</p>
    pub fn set_inputs(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inputs = input;
        self
    }
    /// <p>Specifies the inputs for the dynamic transform that are required.</p>
    pub fn get_inputs(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        &self.inputs
    }
    /// Appends an item to `parameters`.
    ///
    /// To override the contents of this collection use [`set_parameters`](Self::set_parameters).
    ///
    /// <p>Specifies the parameters of the dynamic transform.</p>
    pub fn parameters(mut self, input: crate::types::TransformConfigParameter) -> Self {
        let mut v = self.parameters.unwrap_or_default();
        v.push(input);
        self.parameters = ::std::option::Option::Some(v);
        self
    }
    /// <p>Specifies the parameters of the dynamic transform.</p>
    pub fn set_parameters(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::TransformConfigParameter>>) -> Self {
        self.parameters = input;
        self
    }
    /// <p>Specifies the parameters of the dynamic transform.</p>
    pub fn get_parameters(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::TransformConfigParameter>> {
        &self.parameters
    }
    /// <p>Specifies the name of the function of the dynamic transform.</p>
    pub fn function_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.function_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Specifies the name of the function of the dynamic transform.</p>
    pub fn set_function_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.function_name = input;
        self
    }
    /// <p>Specifies the name of the function of the dynamic transform.</p>
    pub fn get_function_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.function_name
    }
    /// <p>Specifies the path of the dynamic transform source and config files.</p>
    pub fn path(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.path = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Specifies the path of the dynamic transform source and config files.</p>
    pub fn set_path(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.path = input;
        self
    }
    /// <p>Specifies the path of the dynamic transform source and config files.</p>
    pub fn get_path(&self) -> &::std::option::Option<::std::string::String> {
        &self.path
    }
    /// <p>This field is not used and will be deprecated in future release.</p>
    pub fn version(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.version = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>This field is not used and will be deprecated in future release.</p>
    pub fn set_version(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.version = input;
        self
    }
    /// <p>This field is not used and will be deprecated in future release.</p>
    pub fn get_version(&self) -> &::std::option::Option<::std::string::String> {
        &self.version
    }
    /// Appends an item to `output_schemas`.
    ///
    /// To override the contents of this collection use [`set_output_schemas`](Self::set_output_schemas).
    ///
    /// <p>Specifies the data schema for the dynamic transform.</p>
    pub fn output_schemas(mut self, input: crate::types::GlueSchema) -> Self {
        let mut v = self.output_schemas.unwrap_or_default();
        v.push(input);
        self.output_schemas = ::std::option::Option::Some(v);
        self
    }
    /// <p>Specifies the data schema for the dynamic transform.</p>
    pub fn set_output_schemas(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::GlueSchema>>) -> Self {
        self.output_schemas = input;
        self
    }
    /// <p>Specifies the data schema for the dynamic transform.</p>
    pub fn get_output_schemas(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::GlueSchema>> {
        &self.output_schemas
    }
    /// Consumes the builder and constructs a [`DynamicTransform`](crate::types::DynamicTransform).
    pub fn build(self) -> crate::types::DynamicTransform {
        crate::types::DynamicTransform {
            name: self.name,
            transform_name: self.transform_name,
            inputs: self.inputs,
            parameters: self.parameters,
            function_name: self.function_name,
            path: self.path,
            version: self.version,
            output_schemas: self.output_schemas,
        }
    }
}
