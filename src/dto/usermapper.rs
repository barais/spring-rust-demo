use crate::{domain::model::User, dto::userdto::{UserDto, UserInput}};

impl From<User> for UserDto {
    fn from(item: User) -> UserDto {
        UserDto {
            id: Some(item.id),
            name: item.name,
            firstname: item.firstname,
            age: item.age,
        }
    }
}

impl From<UserDto> for User {
    fn from(item: UserDto) -> User {
        User {
            id: item.id.unwrap_or(0),
            name: item.name,
            firstname: item.firstname,
            age: item.age,
        }
    }
}
impl From<UserInput> for User {
    fn from(item: UserInput) -> User {
        User {
            id:0,
            name: item.name,
            firstname: item.firstname,
            age: item.age,
        }
    }
}