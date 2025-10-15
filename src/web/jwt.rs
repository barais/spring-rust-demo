use axum_extra::headers::authorization::Bearer;
use axum_extra::headers::Authorization;
use axum_extra::TypedHeader;
use jsonwebtoken::{Algorithm, DecodingKey, Validation};
use lazy_static::lazy_static;
use schemars::json_schema;
use serde::{Deserialize, Serialize};
use spring_web::aide::generate::GenContext;
use spring_web::aide::openapi::{
    Operation, Parameter, ParameterData, ParameterSchemaOrContent, ReferenceOr, SchemaObject,
};
use spring_web::aide::OperationInput;
use spring_web::axum::http::request::Parts;
use spring_web::axum::RequestPartsExt;
use spring_web::error::{KnownWebError, Result, WebError};
use spring_web::extractor::FromRequestParts;

lazy_static! {
    static ref DECODE_KEY: DecodingKey =
        DecodingKey::from_rsa_pem(include_bytes!("../../keys/public.key"))
            .expect("public key parse failed");
/*    static ref ENCODE_KEY: EncodingKey =
        EncodingKey::from_rsa_pem(include_bytes!("../keys/private.key"))
            .expect("private key parse failed"); */
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub exp: u64,
    pub iat: Option<u64>,
    pub jti: Option<String>,
    pub iss: Option<String>,
    pub aud: Option<String>,
    pub sub: String,
    pub typ: Option<String>,
    pub azp: Option<String>,
    pub sid: Option<String>,
    pub acr: Option<String>,
    #[serde(rename = "allowed-origins")]
    pub allowed_origins: Option<Vec<String>>,
    pub realm_access: Option<RealmAccess>,
    pub resource_access: Option<ResourceAccess>,
    pub scope: Option<String>,
    pub email_verified: Option<bool>,
    pub name: Option<String>,
    pub preferred_username: Option<String>,
    pub given_name: Option<String>,
    pub family_name: Option<String>,
    pub email: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealmAccess {
    roles: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceAccess {
    account: Option<RealmAccess>,
}




impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = WebError;

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &S,
    ) -> std::result::Result<Self, Self::Rejection> {
        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| KnownWebError::bad_request("invalid token"))?;
        // Decode the user data
        let claims = decode(bearer.token())?;

        Ok(claims)
    }
}

/// define the OpenAPI doc for Claims
impl OperationInput for Claims {
    fn operation_input(_ctx: &mut GenContext, operation: &mut Operation) {
        let aide_schema = SchemaObject {
            json_schema: json_schema!({
                "description": "JWT Claims",
                "type": ["string"]
            }),
            external_docs: None,
            example: None,
        };
        operation
            .parameters
            .push(ReferenceOr::Item(Parameter::Header {
                parameter_data: ParameterData {
                    name: "Authorization".into(),
                    description: Some("Bearer token for authentication".into()),
                    required: true,
                    format: ParameterSchemaOrContent::Schema(aide_schema),
                    example: Some("Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...".into()),
                    examples: Default::default(),
                    explode: Default::default(),
                    extensions: Default::default(),
                    deprecated: Default::default(),
                },
                style: Default::default(),
            }));
    }
}

/// JWT encode
/* pub fn encode(claims: Claims) -> Result<String> {
    let header = Header::new(Algorithm::RS256);

    let token = jsonwebtoken::encode::<Claims>(&header, &claims, &ENCODE_KEY)
        .map_err(|_| KnownWebError::internal_server_error("Token created error"))?;

    Ok(token)
}*/

/// JWT decode
pub fn decode(token: &str) -> Result<Claims> {
    let mut validation = Validation::new(Algorithm::RS256);
    validation.set_audience(&["account"]);
    let token_data =
        jsonwebtoken::decode::<Claims>(&token, &DECODE_KEY, &validation).map_err(|e| {
            tracing::info!("{:?}", e);
            KnownWebError::bad_request("invalid token")
        })?;
    Ok(token_data.claims)
}