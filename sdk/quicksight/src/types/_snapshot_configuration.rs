// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes the configuration of the dashboard snapshot.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct SnapshotConfiguration {
    /// <p>A list of <code>SnapshotJobResultFileGroup</code> objects that contain information about the snapshot that is generated. This list can hold a maximum of 6 <code>FileGroup</code> configurations.</p>
    pub file_groups: ::std::option::Option<::std::vec::Vec<crate::types::SnapshotFileGroup>>,
    /// <p>A structure that contains information on the Amazon S3 bucket that the generated snapshot is stored in.</p>
    pub destination_configuration: ::std::option::Option<crate::types::SnapshotDestinationConfiguration>,
    /// <p>A list of Amazon QuickSight parameters and the list's override values.</p>
    pub parameters: ::std::option::Option<crate::types::Parameters>,
}
impl SnapshotConfiguration {
    /// <p>A list of <code>SnapshotJobResultFileGroup</code> objects that contain information about the snapshot that is generated. This list can hold a maximum of 6 <code>FileGroup</code> configurations.</p>
    pub fn file_groups(&self) -> ::std::option::Option<&[crate::types::SnapshotFileGroup]> {
        self.file_groups.as_deref()
    }
    /// <p>A structure that contains information on the Amazon S3 bucket that the generated snapshot is stored in.</p>
    pub fn destination_configuration(&self) -> ::std::option::Option<&crate::types::SnapshotDestinationConfiguration> {
        self.destination_configuration.as_ref()
    }
    /// <p>A list of Amazon QuickSight parameters and the list's override values.</p>
    pub fn parameters(&self) -> ::std::option::Option<&crate::types::Parameters> {
        self.parameters.as_ref()
    }
}
impl SnapshotConfiguration {
    /// Creates a new builder-style object to manufacture [`SnapshotConfiguration`](crate::types::SnapshotConfiguration).
    pub fn builder() -> crate::types::builders::SnapshotConfigurationBuilder {
        crate::types::builders::SnapshotConfigurationBuilder::default()
    }
}

/// A builder for [`SnapshotConfiguration`](crate::types::SnapshotConfiguration).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct SnapshotConfigurationBuilder {
    pub(crate) file_groups: ::std::option::Option<::std::vec::Vec<crate::types::SnapshotFileGroup>>,
    pub(crate) destination_configuration: ::std::option::Option<crate::types::SnapshotDestinationConfiguration>,
    pub(crate) parameters: ::std::option::Option<crate::types::Parameters>,
}
impl SnapshotConfigurationBuilder {
    /// Appends an item to `file_groups`.
    ///
    /// To override the contents of this collection use [`set_file_groups`](Self::set_file_groups).
    ///
    /// <p>A list of <code>SnapshotJobResultFileGroup</code> objects that contain information about the snapshot that is generated. This list can hold a maximum of 6 <code>FileGroup</code> configurations.</p>
    pub fn file_groups(mut self, input: crate::types::SnapshotFileGroup) -> Self {
        let mut v = self.file_groups.unwrap_or_default();
        v.push(input);
        self.file_groups = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of <code>SnapshotJobResultFileGroup</code> objects that contain information about the snapshot that is generated. This list can hold a maximum of 6 <code>FileGroup</code> configurations.</p>
    pub fn set_file_groups(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::SnapshotFileGroup>>) -> Self {
        self.file_groups = input;
        self
    }
    /// <p>A list of <code>SnapshotJobResultFileGroup</code> objects that contain information about the snapshot that is generated. This list can hold a maximum of 6 <code>FileGroup</code> configurations.</p>
    pub fn get_file_groups(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::SnapshotFileGroup>> {
        &self.file_groups
    }
    /// <p>A structure that contains information on the Amazon S3 bucket that the generated snapshot is stored in.</p>
    pub fn destination_configuration(mut self, input: crate::types::SnapshotDestinationConfiguration) -> Self {
        self.destination_configuration = ::std::option::Option::Some(input);
        self
    }
    /// <p>A structure that contains information on the Amazon S3 bucket that the generated snapshot is stored in.</p>
    pub fn set_destination_configuration(mut self, input: ::std::option::Option<crate::types::SnapshotDestinationConfiguration>) -> Self {
        self.destination_configuration = input;
        self
    }
    /// <p>A structure that contains information on the Amazon S3 bucket that the generated snapshot is stored in.</p>
    pub fn get_destination_configuration(&self) -> &::std::option::Option<crate::types::SnapshotDestinationConfiguration> {
        &self.destination_configuration
    }
    /// <p>A list of Amazon QuickSight parameters and the list's override values.</p>
    pub fn parameters(mut self, input: crate::types::Parameters) -> Self {
        self.parameters = ::std::option::Option::Some(input);
        self
    }
    /// <p>A list of Amazon QuickSight parameters and the list's override values.</p>
    pub fn set_parameters(mut self, input: ::std::option::Option<crate::types::Parameters>) -> Self {
        self.parameters = input;
        self
    }
    /// <p>A list of Amazon QuickSight parameters and the list's override values.</p>
    pub fn get_parameters(&self) -> &::std::option::Option<crate::types::Parameters> {
        &self.parameters
    }
    /// Consumes the builder and constructs a [`SnapshotConfiguration`](crate::types::SnapshotConfiguration).
    pub fn build(self) -> crate::types::SnapshotConfiguration {
        crate::types::SnapshotConfiguration {
            file_groups: self.file_groups,
            destination_configuration: self.destination_configuration,
            parameters: self.parameters,
        }
    }
}
