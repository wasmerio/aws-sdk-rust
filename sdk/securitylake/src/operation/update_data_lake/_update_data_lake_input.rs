// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UpdateDataLakeInput {
    /// <p>Specify the Region or Regions that will contribute data to the rollup region.</p>
    pub configurations: ::std::option::Option<::std::vec::Vec<crate::types::DataLakeConfiguration>>,
}
impl UpdateDataLakeInput {
    /// <p>Specify the Region or Regions that will contribute data to the rollup region.</p>
    pub fn configurations(&self) -> ::std::option::Option<&[crate::types::DataLakeConfiguration]> {
        self.configurations.as_deref()
    }
}
impl UpdateDataLakeInput {
    /// Creates a new builder-style object to manufacture [`UpdateDataLakeInput`](crate::operation::update_data_lake::UpdateDataLakeInput).
    pub fn builder() -> crate::operation::update_data_lake::builders::UpdateDataLakeInputBuilder {
        crate::operation::update_data_lake::builders::UpdateDataLakeInputBuilder::default()
    }
}

/// A builder for [`UpdateDataLakeInput`](crate::operation::update_data_lake::UpdateDataLakeInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct UpdateDataLakeInputBuilder {
    pub(crate) configurations: ::std::option::Option<::std::vec::Vec<crate::types::DataLakeConfiguration>>,
}
impl UpdateDataLakeInputBuilder {
    /// Appends an item to `configurations`.
    ///
    /// To override the contents of this collection use [`set_configurations`](Self::set_configurations).
    ///
    /// <p>Specify the Region or Regions that will contribute data to the rollup region.</p>
    pub fn configurations(mut self, input: crate::types::DataLakeConfiguration) -> Self {
        let mut v = self.configurations.unwrap_or_default();
        v.push(input);
        self.configurations = ::std::option::Option::Some(v);
        self
    }
    /// <p>Specify the Region or Regions that will contribute data to the rollup region.</p>
    pub fn set_configurations(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::DataLakeConfiguration>>) -> Self {
        self.configurations = input;
        self
    }
    /// <p>Specify the Region or Regions that will contribute data to the rollup region.</p>
    pub fn get_configurations(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::DataLakeConfiguration>> {
        &self.configurations
    }
    /// Consumes the builder and constructs a [`UpdateDataLakeInput`](crate::operation::update_data_lake::UpdateDataLakeInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::update_data_lake::UpdateDataLakeInput, ::aws_smithy_http::operation::error::BuildError> {
        ::std::result::Result::Ok(crate::operation::update_data_lake::UpdateDataLakeInput {
            configurations: self.configurations,
        })
    }
}
