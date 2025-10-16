use welds::WeldsModel;

#[derive(Debug, WeldsModel)]
#[welds(schema = "public", table = "users")]
pub struct User {
    #[welds(rename = "id")]
    #[welds(primary_key)]
    pub id: i64,
    #[welds(rename = "name")]
    pub name: String,
    pub firstname: String,
    pub sub: String,
    pub email: String,
    pub age: Option<i32>,
}
