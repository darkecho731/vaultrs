use rustify_derive::Endpoint;

use super::responses::ListMountsResponse;

/// ## List Available Visible Mounts
/// This endpoint lists all enabled auth methods.
///
/// * Path: sys/internal/ui/mounts
/// * Method: GET
/// * Response: [ListMountResponse]
/// * Reference: https://www.vaultproject.io/api-docs/system/mounts#list-mounted-secrets-engines

#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "sys/internal/ui/mounts",
    response = "ListMountsResponse",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct ListMountsRequest {}
