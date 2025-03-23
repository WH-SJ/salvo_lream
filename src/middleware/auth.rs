// use salvo::prelude::*;
// use jsonwebtoken::{decode, Validation, Algorithm, DecodingKey};
// use crate::{errors::AppError, models::user::UserClaims};
// 
// #[derive(Clone)]
// pub struct JwtConfig {
//     pub secret: String,
//     pub validation: Validation,
// }
// 
// #[handler]
// pub async fn auth_middleware(
//     req: &mut Request,
//     depot: &mut Depot,
//     _res: &mut Response,
//     ctrl: &mut FlowCtrl,
// ) {
//     let config = depot.obtain::<JwtConfig>().unwrap();
// 
//     let token = req
//         .header("Authorization")
//         .and_then(|v| v.strip_prefix("Bearer "))
//         .ok_or_else(|| {
//             ctrl.skip_rest();
//             AppError::Unauthorized
//         })
//         .unwrap();
// 
//     match decode::<UserClaims>(
//         token,
//         &DecodingKey::from_secret(config.secret.as_ref()),
//         &config.validation,
//     ) {
//         Ok(token_data) => {
//             depot.insert("user_claims", token_data.claims);
//             ctrl.call_next(req, depot, res).await;
//         }
//         Err(e) => {
//             ctrl.skip_rest();
//             Err(AppError::Unauthorized).unwrap();
//         }
//     }
// }