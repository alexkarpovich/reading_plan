use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{HeaderMap, request::Parts},
    response::Response,
};


#[derive(Debug)]
pub struct AuthenticatedUser {
    // ...
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        // You can either call them directly...
        let headers = HeaderMap::from_request_parts(parts, state)
            .await
            .map_err(|err| match err {})?;

        // use axum::RequestPartsExt;
        // let Extension(state) = parts.extract::<Extension<AppState>>()
        //     .await
        //     .map_err(|err| err.into_response())?;

        tracing::debug!("{:?}", headers);

        Ok(AuthenticatedUser{})
    }
}