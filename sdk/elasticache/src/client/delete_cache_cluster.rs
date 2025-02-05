// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteCacheCluster`](crate::operation::delete_cache_cluster::builders::DeleteCacheClusterFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`cache_cluster_id(impl ::std::convert::Into<String>)`](crate::operation::delete_cache_cluster::builders::DeleteCacheClusterFluentBuilder::cache_cluster_id) / [`set_cache_cluster_id(Option<String>)`](crate::operation::delete_cache_cluster::builders::DeleteCacheClusterFluentBuilder::set_cache_cluster_id): <p>The cluster identifier for the cluster to be deleted. This parameter is not case sensitive.</p>
    ///   - [`final_snapshot_identifier(impl ::std::convert::Into<String>)`](crate::operation::delete_cache_cluster::builders::DeleteCacheClusterFluentBuilder::final_snapshot_identifier) / [`set_final_snapshot_identifier(Option<String>)`](crate::operation::delete_cache_cluster::builders::DeleteCacheClusterFluentBuilder::set_final_snapshot_identifier): <p>The user-supplied name of a final cluster snapshot. This is the unique name that identifies the snapshot. ElastiCache creates the snapshot, and then deletes the cluster immediately afterward.</p>
    /// - On success, responds with [`DeleteCacheClusterOutput`](crate::operation::delete_cache_cluster::DeleteCacheClusterOutput) with field(s):
    ///   - [`cache_cluster(Option<CacheCluster>)`](crate::operation::delete_cache_cluster::DeleteCacheClusterOutput::cache_cluster): <p>Contains all of the attributes of a specific cluster.</p>
    /// - On failure, responds with [`SdkError<DeleteCacheClusterError>`](crate::operation::delete_cache_cluster::DeleteCacheClusterError)
    pub fn delete_cache_cluster(&self) -> crate::operation::delete_cache_cluster::builders::DeleteCacheClusterFluentBuilder {
        crate::operation::delete_cache_cluster::builders::DeleteCacheClusterFluentBuilder::new(self.handle.clone())
    }
}
