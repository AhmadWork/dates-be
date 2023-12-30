use crate::models::palms::{self, Entity as Palms};
use crate::models::users::{self, Entity as Users, Model};
use crate::models::sea_orm_active_enums::Growth;
use crate::utilities::errors::AppError;
use crate::utilities::hash_password::{hash_password, verify};
use crate::{config::Config, utilities::jwt::create_token};
use axum::http::StatusCode;
use axum::{Extension, Json};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, Set,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize, Deserialize, Default)]
pub struct UserResponse {
    data: User,
}

#[derive(Serialize, Deserialize, Default)]
pub struct UserProfileResponse {
    id: i32,
    name: String,
    email: String,
    dates: i32,
    coins: i32,
}

impl Default for Growth {
    fn default() -> Self {
        Growth::Adult
    }
}


#[derive(Serialize, Deserialize)]
pub struct UserProfileResponseWithTree {
    id: i32,
    name: String,
    email: String,
    dates: i32,
    coins: i32,
    token: Option<String>,
    palms: Vec<Palm>,
}


#[derive(Serialize, Deserialize, Default)]
pub struct User {
    id: i32,
    name: String,
    email: String,
    dates: i32,
    coins: i32,
    token: String,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Palm {
    id: i32,
    dpm: i32,
    growth: Growth,
    user_id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct RequestUser {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct RequestRegisterUser {
    pub name: String,
    pub email: String,
    pub dates: i32,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct Coins {
    number: i32,
    
}

#[derive(Serialize, Deserialize)]
pub struct Dates {
    number: i32,
    
}


#[derive(Serialize, Deserialize)]
pub struct RequestUpdate {
    pub coins: Option<Coins>,
    pub dates: Option<Dates>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestTree {
    dpm: i32,
    growth: Growth,
    
}

pub async fn create_user(
    Extension(config): Extension<Arc<Config>>,
    Extension(db): Extension<DatabaseConnection>,
    Json(request_user): Json<RequestRegisterUser>,
) -> Result<Json<UserProfileResponseWithTree>, AppError> {
    let token = create_token(&config.jwt_secret, &request_user.email).unwrap();
    let hash = match hash_password(&request_user.password) {
        Ok(hash) => hash,
        Err(error) => return Err(AppError::new(StatusCode::INTERNAL_SERVER_ERROR, error)),
    };
    let new_user = users::ActiveModel {
        name: Set(request_user.name),
        email: Set(request_user.email),
        dates: Set(request_user.dates),
        password: Set(hash),
        token: Set(Some(token)),
        ..Default::default()
    };
    let user = match new_user.insert(&db).await {
        Ok(user) => user,
        Err(error) => {
            return Err(translate_error(error));
        }
    };
        let new_user_id = user.id;
        let new_palm = palms::ActiveModel {
            dpm: Set(3),
            growth: Set(Some(Growth::Adult)),
            user_id: Set(new_user_id),
            ..Default::default()
        };
        new_palm.insert(&db).await.unwrap();

        let palm_trees = match find_palms_by_user(new_user_id, &db).await{
            Ok(palm_trees) => palm_trees,
            Err(error) => {
                return Err(translate_error(error));
            }
        };
        
        let palms_json: Vec<Palm> = palm_trees.into_iter().map(|p| {
            Palm {
                id: p.id,
                dpm: p.dpm,
                growth: p.growth.unwrap(),
                user_id: p.user_id,
            }
        }).collect();

        Ok(Json(UserProfileResponseWithTree {
            id: user.id.clone(),
            name: user.name.clone(),
            email: user.email.clone(),
            dates: user.dates.clone(),
            coins: user.coins.clone(),
            token: Some(user.token.clone().unwrap_or_default()),
            palms: palms_json,
        }))
}

pub async fn sign_in(
    Extension(db): Extension<DatabaseConnection>,
    Extension(config): Extension<Arc<Config>>,
    Json(request_user): Json<RequestUser>,
) -> Result<Json<UserProfileResponseWithTree>, AppError> {
    let db_user = match Users::find()
        .filter(users::Column::Email.eq(request_user.email))
        .one(&db)
        .await
    {
        Ok(Some(user)) => user,
        Ok(None) => {
            return Err(AppError::new(
                StatusCode::BAD_REQUEST,
                eyre::eyre!("incorrect email and/or password"),
            ))
        }
        _ => {
            return Err(AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                eyre::eyre!("Internal Server error please try again later"),
            ))
        }
    };

    let is_password_correct = match verify(&request_user.password, &db_user.password) {
        Ok(result) => result,
        Err(error) => return Err(error),
    };

    if is_password_correct {
        let new_token = match create_token(&config.jwt_secret, &db_user.email) {
            Ok(token) => token,
            Err(error) => return Err(AppError::new(StatusCode::INTERNAL_SERVER_ERROR, error)),
        };
        update_user_token(&new_token, db_user.clone().into(), &db).await?;

        let palm_trees = match find_palms_by_user(db_user.id, &db).await{
            Ok(palm_trees) => palm_trees,
            Err(error) => {
                return Err(translate_error(error));
            }
        };
        
        let palms_json: Vec<Palm> = palm_trees.into_iter().map(|p| {
            Palm {
                id: p.id,
                dpm: p.dpm,
                growth: p.growth.unwrap(),
                user_id: p.user_id,
            }
        }).collect();

        Ok(Json(UserProfileResponseWithTree {
            id: db_user.id,
            name: db_user.name,
            email: db_user.email,
            dates: db_user.dates,
            coins: db_user.coins,
            token: Some(new_token.to_owned()),
            palms: palms_json,
        }))
    } else {
        Err(AppError::new(
            StatusCode::BAD_REQUEST,
            eyre::eyre!("incorrect email and/or password"),
        ))
    }
}

pub async fn user_detalis(
    Extension(db): Extension<DatabaseConnection>,
    Extension(user): Extension<Model>,
) -> Json<UserProfileResponse> {
            Json(UserProfileResponse {
                id: user.id,
                name: user.name,
                email: user.email,
                dates: user.dates,
                coins: user.coins,
        })
}
pub async fn logout(
    Extension(db): Extension<DatabaseConnection>,
    Extension(user): Extension<Model>,
) -> Result<(), AppError> {
    let mut user: users::ActiveModel = user.into();
    user.token = Set(None);
    match user.save(&db).await {
        Err(error) => Err(AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            eyre::eyre!(error),
        )),
        Ok(_) => Ok(()),
    }
}

pub async fn update_user(
    Extension(db): Extension<DatabaseConnection>,
    Extension(user): Extension<Model>,
    Json(request_update): Json<RequestUpdate>,
) -> Result<Json<UserProfileResponse>, AppError> {
    let mut user: users::ActiveModel = user.into();
    // Handle Coins if it's provided
    if let Some(coins) = &request_update.coins {
        // Assuming Coins has a field `number`
        user.coins = Set(coins.number);
    }

    // Handle Dates if it's provided
    if let Some(dates) = &request_update.dates {
        // Assuming Dates has a field `number`
        user.dates = Set(dates.number);
    }
    match user.save(&db).await {
        Err(error) => Err(AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            eyre::eyre!(error),
        )),
        Ok(saved_user) => {
            Ok(Json(UserProfileResponse {
                id: saved_user.id.unwrap(),
                name: saved_user.name.unwrap(),
                email: saved_user.email.unwrap(),
                dates: saved_user.dates.unwrap(),
                coins: saved_user.coins.unwrap(),
          }))
        },
    }
}

pub async fn add_tree(
    Extension(db): Extension<DatabaseConnection>,
    Extension(user): Extension<Model>,
    Json(request_update): Json<RequestTree>,
) -> Result<Json<UserProfileResponseWithTree>, AppError> {
    let mut user: users::ActiveModel = user.into();
    let user_id = user.id.clone().unwrap();

    let new_palm = palms::ActiveModel {
        dpm: Set(3),
        growth: Set(Some(Growth::Adult)),
        user_id:user.id.clone(),
        ..Default::default()
    };
    match new_palm.insert(&db).await {
        Err(error) => Err(AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            eyre::eyre!(error),
        )),
        Ok(_) => {

            let palm_trees = match find_palms_by_user(user_id.clone(), &db).await{
                Ok(palm_trees) => palm_trees,
                Err(error) => {
                    return Err(translate_error(error));
                }
            };
            
            let palms_json: Vec<Palm> = palm_trees.into_iter().map(|p| {
                Palm {
                    id: p.id,
                    dpm: p.dpm,
                    growth: p.growth.unwrap(),
                    user_id: p.user_id,
                }
            }).collect();

            Ok(Json(UserProfileResponseWithTree {
                id: user.id.unwrap(),
                name: user.name.unwrap(),
                email: user.email.unwrap(),
                dates: user.dates.unwrap(),
                coins: user.coins.unwrap(),
                token: None,
                palms: palms_json,
            }))
        }
    }
}

fn translate_error(error: DbErr) -> AppError {
    if let DbErr::Query(query_error) = error {

            dbg!("is a query error, but not exactly what we thought", &query_error);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, eyre::eyre!(query_error))
    } else {
        dbg!("not a query error");
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, eyre::eyre!(error))
    }
}


async fn find_palms_by_user(user_id: i32, db: &DatabaseConnection) -> Result<Vec<palms::Model>, DbErr> {
        palms::Entity::find()
        .filter(palms::Column::UserId.eq(user_id))
        .all(db)
        .await
}

async fn update_user_token(
    token: &str,
    mut db_user: users::ActiveModel,
    db: &DatabaseConnection,
) -> Result<(), AppError> {
    db_user.token = Set(Some(token.to_owned()));
    print!("{}", &token);
    if let Err(error) = db_user.save(db).await {
        Err(AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            eyre::eyre!(error),
        ))
    } else {
        Ok(())
    }
}
