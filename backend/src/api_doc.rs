use utoipa::OpenApi;

use crate::endpoints;

#[derive(OpenApi)]
#[openapi(
    paths(endpoints::powerlifters),
    tags((name = "Powerlifters"))
)]
pub struct ApiDoc;
