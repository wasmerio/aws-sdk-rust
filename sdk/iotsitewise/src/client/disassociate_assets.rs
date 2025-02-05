// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DisassociateAssets`](crate::operation::disassociate_assets::builders::DisassociateAssetsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`asset_id(impl ::std::convert::Into<String>)`](crate::operation::disassociate_assets::builders::DisassociateAssetsFluentBuilder::asset_id) / [`set_asset_id(Option<String>)`](crate::operation::disassociate_assets::builders::DisassociateAssetsFluentBuilder::set_asset_id): <p>The ID of the parent asset from which to disassociate the child asset.</p>
    ///   - [`hierarchy_id(impl ::std::convert::Into<String>)`](crate::operation::disassociate_assets::builders::DisassociateAssetsFluentBuilder::hierarchy_id) / [`set_hierarchy_id(Option<String>)`](crate::operation::disassociate_assets::builders::DisassociateAssetsFluentBuilder::set_hierarchy_id): <p>The ID of a hierarchy in the parent asset's model. Hierarchies allow different groupings of assets to be formed that all come from the same asset model. You can use the hierarchy ID to identify the correct asset to disassociate. For more information, see <a href="https://docs.aws.amazon.com/iot-sitewise/latest/userguide/asset-hierarchies.html">Asset hierarchies</a> in the <i>IoT SiteWise User Guide</i>.</p>
    ///   - [`child_asset_id(impl ::std::convert::Into<String>)`](crate::operation::disassociate_assets::builders::DisassociateAssetsFluentBuilder::child_asset_id) / [`set_child_asset_id(Option<String>)`](crate::operation::disassociate_assets::builders::DisassociateAssetsFluentBuilder::set_child_asset_id): <p>The ID of the child asset to disassociate.</p>
    ///   - [`client_token(impl ::std::convert::Into<String>)`](crate::operation::disassociate_assets::builders::DisassociateAssetsFluentBuilder::client_token) / [`set_client_token(Option<String>)`](crate::operation::disassociate_assets::builders::DisassociateAssetsFluentBuilder::set_client_token): <p>A unique case-sensitive identifier that you can provide to ensure the idempotency of the request. Don't reuse this client token if a new idempotent request is required.</p>
    /// - On success, responds with [`DisassociateAssetsOutput`](crate::operation::disassociate_assets::DisassociateAssetsOutput)
    /// - On failure, responds with [`SdkError<DisassociateAssetsError>`](crate::operation::disassociate_assets::DisassociateAssetsError)
    pub fn disassociate_assets(&self) -> crate::operation::disassociate_assets::builders::DisassociateAssetsFluentBuilder {
        crate::operation::disassociate_assets::builders::DisassociateAssetsFluentBuilder::new(self.handle.clone())
    }
}
