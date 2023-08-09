use async_graphql::*;
use chrono::{DateTime, NaiveDateTime, Utc};
use graphql_core::{
    simple_generic_errors::{DatabaseError, InternalError},
    standard_graphql_error::{validate_auth, StandardGraphqlError},
    ContextExt,
};
use service::{
    auth::{Resource, ResourceAccessRequest},
    login::LoginError,
    sync::sync_user::SyncUser,
};

use crate::queries::InvalidCredentials;

pub struct UpdateUserNode {
    pub last_successful_sync: NaiveDateTime,
}

#[Object]
impl UpdateUserNode {
    pub async fn last_successful_sync(&self) -> DateTime<Utc> {
        DateTime::<Utc>::from_utc(self.last_successful_sync, Utc)
    }
}

#[derive(Interface)]
#[graphql(field(name = "description", type = "&str"))]
pub enum UpdateUserErrorInterface {
    InvalidCredentials(InvalidCredentials),
    DatabaseError(DatabaseError),
    InternalError(InternalError),
}

#[derive(SimpleObject)]
pub struct UpdateUserError {
    pub error: UpdateUserErrorInterface,
}

#[derive(Union)]
pub enum UpdateUserResponse {
    Error(UpdateUserError),
    Response(UpdateUserNode),
}

pub async fn update_user(ctx: &Context<'_>) -> Result<UpdateUserResponse> {
    let user = validate_auth(
        ctx,
        &ResourceAccessRequest {
            resource: Resource::ManualSync,
            store_id: None,
        },
    )?;
    let service_provider = ctx.service_provider();
    let auth_data = ctx.get_auth_data();

    let user = match SyncUser::update_user(&service_provider, auth_data, &user.user_id).await {
        Ok(user) => user,
        Err(error) => {
            let formatted_error = format!("{:#?}", error);
            let graphql_error = match error {
                LoginError::LoginFailure => {
                    return Ok(UpdateUserResponse::Error(UpdateUserError {
                        error: UpdateUserErrorInterface::InvalidCredentials(InvalidCredentials {}),
                    }))
                }
                LoginError::FetchUserError(_)
                | LoginError::UpdateUserError(_)
                | LoginError::InternalError(_)
                | LoginError::DatabaseError(_)
                | LoginError::FailedToGenerateToken(_) => {
                    StandardGraphqlError::InternalError(formatted_error)
                }
            };
            return Err(graphql_error.extend());
        }
    };

    Ok(UpdateUserResponse::Response(UpdateUserNode {
        last_successful_sync: user.last_successful_sync,
    }))
}
