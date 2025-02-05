// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct BatchDeleteClusterSnapshotsInput {
    /// <p>A list of identifiers for the snapshots that you want to delete.</p>
    pub identifiers: ::std::option::Option<::std::vec::Vec<crate::types::DeleteClusterSnapshotMessage>>,
}
impl BatchDeleteClusterSnapshotsInput {
    /// <p>A list of identifiers for the snapshots that you want to delete.</p>
    pub fn identifiers(&self) -> ::std::option::Option<&[crate::types::DeleteClusterSnapshotMessage]> {
        self.identifiers.as_deref()
    }
}
impl BatchDeleteClusterSnapshotsInput {
    /// Creates a new builder-style object to manufacture [`BatchDeleteClusterSnapshotsInput`](crate::operation::batch_delete_cluster_snapshots::BatchDeleteClusterSnapshotsInput).
    pub fn builder() -> crate::operation::batch_delete_cluster_snapshots::builders::BatchDeleteClusterSnapshotsInputBuilder {
        crate::operation::batch_delete_cluster_snapshots::builders::BatchDeleteClusterSnapshotsInputBuilder::default()
    }
}

/// A builder for [`BatchDeleteClusterSnapshotsInput`](crate::operation::batch_delete_cluster_snapshots::BatchDeleteClusterSnapshotsInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct BatchDeleteClusterSnapshotsInputBuilder {
    pub(crate) identifiers: ::std::option::Option<::std::vec::Vec<crate::types::DeleteClusterSnapshotMessage>>,
}
impl BatchDeleteClusterSnapshotsInputBuilder {
    /// Appends an item to `identifiers`.
    ///
    /// To override the contents of this collection use [`set_identifiers`](Self::set_identifiers).
    ///
    /// <p>A list of identifiers for the snapshots that you want to delete.</p>
    pub fn identifiers(mut self, input: crate::types::DeleteClusterSnapshotMessage) -> Self {
        let mut v = self.identifiers.unwrap_or_default();
        v.push(input);
        self.identifiers = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of identifiers for the snapshots that you want to delete.</p>
    pub fn set_identifiers(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::DeleteClusterSnapshotMessage>>) -> Self {
        self.identifiers = input;
        self
    }
    /// <p>A list of identifiers for the snapshots that you want to delete.</p>
    pub fn get_identifiers(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::DeleteClusterSnapshotMessage>> {
        &self.identifiers
    }
    /// Consumes the builder and constructs a [`BatchDeleteClusterSnapshotsInput`](crate::operation::batch_delete_cluster_snapshots::BatchDeleteClusterSnapshotsInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::batch_delete_cluster_snapshots::BatchDeleteClusterSnapshotsInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::batch_delete_cluster_snapshots::BatchDeleteClusterSnapshotsInput {
            identifiers: self.identifiers,
        })
    }
}
