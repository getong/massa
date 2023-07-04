// Copyright (c) 2023 MASSA LABS <info@massa.net>

use std::str::FromStr;

use crate::error::GrpcError;
use crate::server::MassaPrivateGrpc;
use massa_hash::Hash;
use massa_proto_rs::massa::api::v1 as grpc_api;
use massa_signature::KeyPair;
// use massa_proto_rs::massa::model::v1 "add_to_bootstrap_blacklist"as grpc_model;

/// Add IP addresses to node bootstrap blacklist
pub(crate) fn add_to_bootstrap_blacklist(
    _grpc: &MassaPrivateGrpc,
    _request: tonic::Request<grpc_api::AddToBootstrapBlacklistRequest>,
) -> Result<tonic::Response<grpc_api::AddToBootstrapBlacklistResponse>, GrpcError> {
    Err(GrpcError::Unimplemented(
        "add_to_bootstrap_blacklist".to_string(),
    ))
}
/// Add IP addresses to node bootstrap whitelist
pub(crate) fn add_to_bootstrap_whitelist(
    _grpc: &MassaPrivateGrpc,
    _request: tonic::Request<grpc_api::AddToBootstrapWhitelistRequest>,
) -> Result<tonic::Response<grpc_api::AddToBootstrapWhitelistResponse>, GrpcError> {
    Err(GrpcError::Unimplemented(
        "add_to_bootstrap_whitelist".to_string(),
    ))
}
/// Add IP addresses to node peers whitelist. No confirmation to expect.
/// Note: If the ip was unknown it adds it to the known peers, otherwise it updates the peer type
pub(crate) fn add_to_peers_whitelist(
    _grpc: &MassaPrivateGrpc,
    _request: tonic::Request<grpc_api::AddToPeersWhitelistRequest>,
) -> Result<tonic::Response<grpc_api::AddToPeersWhitelistResponse>, GrpcError> {
    Err(GrpcError::Unimplemented(
        "add_staking_secret_keys".to_string(),
    ))
}
/// Add staking secret keys to wallet
pub(crate) fn add_staking_secret_keys(
    grpc: &MassaPrivateGrpc,
    request: tonic::Request<grpc_api::AddStakingSecretKeysRequest>,
) -> Result<tonic::Response<grpc_api::AddStakingSecretKeysResponse>, GrpcError> {
    let secret_keys = request.into_inner().secret_keys;

    if secret_keys.is_empty() {
        return Err(GrpcError::InvalidArgument(
            "no secret key received".to_string(),
        ));
    }

    //TODO customize number of accepted parameters
    if secret_keys.len() as u32 > grpc.grpc_config.max_parameter_size {
        return Err(GrpcError::InvalidArgument(format!(
            "too many secret received. Only a maximum of {} secret keys are accepted per request",
            grpc.grpc_config.max_parameter_size
        )));
    }

    let keypairs = match secret_keys.iter().map(|x| KeyPair::from_str(x)).collect() {
        Ok(keypairs) => keypairs,
        Err(e) => return Err(GrpcError::InvalidArgument(e.to_string()).into()),
    };

    grpc.node_wallet.write().add_keypairs(keypairs)?;

    Ok(tonic::Response::new(
        grpc_api::AddStakingSecretKeysResponse {},
    ))
}

/// Get node bootstrap blacklist IP addresses
pub(crate) fn get_bootstrap_blacklist(
    _grpc: &MassaPrivateGrpc,
    _request: tonic::Request<grpc_api::GetBootstrapBlacklistRequest>,
) -> Result<tonic::Response<grpc_api::GetBootstrapBlacklistResponse>, GrpcError> {
    Err(GrpcError::Unimplemented(
        "get_bootstrap_whitelist".to_string(),
    ))
}
/// Get node bootstrap whitelist IP addresses
pub(crate) fn get_bootstrap_whitelist(
    _grpc: &MassaPrivateGrpc,
    _request: tonic::Request<grpc_api::GetBootstrapWhitelistRequest>,
) -> Result<tonic::Response<grpc_api::GetBootstrapWhitelistResponse>, GrpcError> {
    Err(GrpcError::Unimplemented(
        "allow_everyone_to_bootstrap".to_string(),
    ))
}
/// Allow everyone to bootstrap from the node by removing bootstrap whitelist configuration file
pub(crate) fn allow_everyone_to_bootstrap(
    _grpc: &MassaPrivateGrpc,
    _request: tonic::Request<grpc_api::AllowEveryoneToBootstrapRequest>,
) -> Result<tonic::Response<grpc_api::AllowEveryoneToBootstrapResponse>, GrpcError> {
    Err(GrpcError::Unimplemented("get_node_status".to_string()))
}
/// Get node status
pub(crate) fn get_node_status(
    _grpc: &MassaPrivateGrpc,
    _request: tonic::Request<grpc_api::GetNodeStatusRequest>,
) -> Result<tonic::Response<grpc_api::GetNodeStatusResponse>, GrpcError> {
    Err(GrpcError::Unimplemented("get_peers_whitelist".to_string()))
}
/// Get node peers whitelist IP addresses
pub(crate) fn get_peers_whitelist(
    _grpc: &MassaPrivateGrpc,
    _request: tonic::Request<grpc_api::GetPeersWhitelistRequest>,
) -> Result<tonic::Response<grpc_api::GetPeersWhitelistResponse>, GrpcError> {
    Err(GrpcError::Unimplemented(
        "remove_from_bootstrap_blacklist".to_string(),
    ))
}
/// Remove from bootstrap blacklist given IP addresses
pub(crate) fn remove_from_bootstrap_blacklist(
    _grpc: &MassaPrivateGrpc,
    _request: tonic::Request<grpc_api::RemoveFromBootstrapBlacklistRequest>,
) -> Result<tonic::Response<grpc_api::RemoveFromBootstrapBlacklistResponse>, GrpcError> {
    Err(GrpcError::Unimplemented(
        "remove_from_bootstrap_whitelist".to_string(),
    ))
}
/// Remove from bootstrap whitelist given IP addresses
pub(crate) fn remove_from_bootstrap_whitelist(
    _grpc: &MassaPrivateGrpc,
    _request: tonic::Request<grpc_api::RemoveFromBootstrapWhitelistRequest>,
) -> Result<tonic::Response<grpc_api::RemoveFromBootstrapWhitelistResponse>, GrpcError> {
    Err(GrpcError::Unimplemented(
        "remove_from_peers_whitelist".to_string(),
    ))
}
/// Remove from peers whitelist given IP addresses
pub(crate) fn remove_from_peers_whitelist(
    _grpc: &MassaPrivateGrpc,
    _request: tonic::Request<grpc_api::RemoveFromPeersWhitelistRequest>,
) -> Result<tonic::Response<grpc_api::RemoveFromPeersWhitelistResponse>, GrpcError> {
    Err(GrpcError::Unimplemented(
        "remove_staking_addresses".to_string(),
    ))
}
/// Remove addresses from staking
pub(crate) fn remove_staking_addresses(
    _grpc: &MassaPrivateGrpc,
    _request: tonic::Request<grpc_api::RemoveStakingAddressesRequest>,
) -> Result<tonic::Response<grpc_api::RemoveStakingAddressesResponse>, GrpcError> {
    Err(GrpcError::Unimplemented("sign_messages".to_string()))
}
/// Sign messages with node's key
pub(crate) fn sign_messages(
    grpc: &MassaPrivateGrpc,
    request: tonic::Request<grpc_api::SignMessagesRequest>,
) -> Result<tonic::Response<grpc_api::SignMessagesResponse>, GrpcError> {
    let messages = request.into_inner().messages;

    if messages.is_empty() {
        return Err(GrpcError::InvalidArgument(
            "no message received".to_string(),
        ));
    }

    //TODO customize number of accepted parameters
    if messages.len() as u32 > grpc.grpc_config.max_parameter_size {
        return Err(GrpcError::InvalidArgument(format!(
            "too many messages received. Only a maximum of {} messages are accepted per request",
            grpc.grpc_config.max_parameter_size
        )));
    }

    let keypair = grpc.grpc_config.keypair.clone();
    let signatures = messages
        .into_iter()
        .map(|message| {
            keypair
                .sign(&Hash::compute_from(&message))
                .map(|signature| signature.to_string())
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(tonic::Response::new(grpc_api::SignMessagesResponse {
        public_key: keypair.get_public_key().to_string(),
        signatures,
    }))
}
/// Shutdown the node gracefully
pub(crate) fn shutdown_gracefully(
    _grpc: &MassaPrivateGrpc,
    _request: tonic::Request<grpc_api::ShutdownGracefullyRequest>,
) -> Result<tonic::Response<grpc_api::ShutdownGracefullyResponse>, GrpcError> {
    Err(GrpcError::Unimplemented("shutdown_gracefully".to_string()))
}