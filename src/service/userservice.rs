use spring::plugin::service::Service;
// use spring_web::extractor::{Component};
use crate::{dao::userdao::UserDao, dto::userdto::UserDto};
use spring::tracing;

#[derive(Clone, Service)]
pub struct UserService {
    #[inject(component)]
    pub user_dao: UserDao, //    #[inject(component)]
                           //    test: TestService
}

impl UserService {
    pub async fn get_user(&self, id: i64) -> UserDto {
        tracing::debug!("Get user by id: {}", id);
        let u = self.user_dao.get_user(id).await;
        let dto = UserDto {
            id: u.id,
            name: u.name,
            firstname: u.firstname,
            age: u.age,
        };
        dto
        /*        UserDto {
            id: 123,
            name: "John".to_string(),
            firstname: "Doe".to_string(),
            age: Some(30),
        } */
    }
}
