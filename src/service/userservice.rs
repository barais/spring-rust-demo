use spring::plugin::service::Service;
// use spring_web::extractor::{Component};
use crate::{dao::userdao::UserDao, dto::userdto::UserDto, web::{jwt::Claims, pagination::Pagination}};
use spring::tracing;

#[derive(Clone, Service)]
pub struct UserService {
    #[inject(component)]
    pub user_dao: UserDao, 
}

impl UserService {
    pub async fn get_user(&self, id: i64) -> Option<UserDto> {
        tracing::debug!("Get user by id: {}", id);
        let u = self.user_dao.get_user(id).await;
        let dto = u.and_then(|f|Some(f.into()));
        dto
    }
    pub async fn get_user_by_sub(&self, claims: Claims) -> UserDto {
        tracing::debug!("Get user by sub: {}", claims.sub);
        let u = self.user_dao.get_or_create_user_by_sub(claims).await;
        let dto = u.unwrap().into(); // u.and_then(|f|f.into());
        dto
    }


    pub async fn get_alluser(&self, pagination: Pagination) -> Vec<UserDto> {
        let users = self.user_dao.get_alluser(pagination).await;
        let mut usersdto = Vec::new();

        for u in users {
            let dto = u.into();
            usersdto.push(dto);
        }
        return usersdto;
    }

    pub async fn create_user(&self, mut user: UserDto) -> UserDto {
        let u = user.clone().into();        
        let id = self.user_dao.create_user(u).await;
        user.id = Some(id);
        user
    }
}
