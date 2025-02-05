// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateBGPPeer`](crate::operation::create_bgp_peer::builders::CreateBGPPeerFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`virtual_interface_id(impl ::std::convert::Into<String>)`](crate::operation::create_bgp_peer::builders::CreateBGPPeerFluentBuilder::virtual_interface_id) / [`set_virtual_interface_id(Option<String>)`](crate::operation::create_bgp_peer::builders::CreateBGPPeerFluentBuilder::set_virtual_interface_id): <p>The ID of the virtual interface.</p>
    ///   - [`new_bgp_peer(NewBgpPeer)`](crate::operation::create_bgp_peer::builders::CreateBGPPeerFluentBuilder::new_bgp_peer) / [`set_new_bgp_peer(Option<NewBgpPeer>)`](crate::operation::create_bgp_peer::builders::CreateBGPPeerFluentBuilder::set_new_bgp_peer): <p>Information about the BGP peer.</p>
    /// - On success, responds with [`CreateBgpPeerOutput`](crate::operation::create_bgp_peer::CreateBgpPeerOutput) with field(s):
    ///   - [`virtual_interface(Option<VirtualInterface>)`](crate::operation::create_bgp_peer::CreateBgpPeerOutput::virtual_interface): <p>The virtual interface.</p>
    /// - On failure, responds with [`SdkError<CreateBGPPeerError>`](crate::operation::create_bgp_peer::CreateBGPPeerError)
    pub fn create_bgp_peer(&self) -> crate::operation::create_bgp_peer::builders::CreateBGPPeerFluentBuilder {
        crate::operation::create_bgp_peer::builders::CreateBGPPeerFluentBuilder::new(self.handle.clone())
    }
}
