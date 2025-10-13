use spring::plugin::service::Service;
use welds::{TransactStart, state::DbState};

use crate::{domain::model::User, web::pagination::Pagination, welds::welds::WeldsClient};

#[derive(Clone, Service)]
pub struct UserDao {
    #[inject(component)]
    pub db: WeldsClient,
}

impl UserDao {
    pub async fn get_user(&self, id: i64) -> User {
        self.check_schema().await;
        let u = User::find_by_id(&self.db, id).await.unwrap();

        if u.is_none() {
            return User {
                id: 0,
                name: "".to_string(),
                firstname: "".to_string(),
                age: None,
            };
        } else {
            return u.unwrap().into_inner();
        }
    }

    pub async fn create_user(&self, user: User) -> i64 {
        let client: &(dyn TransactStart) = &self.db;
        let transaction = client.begin().await.unwrap();
        tracing::info!("Transaction started {}", user.name);
        //  let t = welds::query::insert::insert_one(&mut user, &self.db).await;
        let mut u1 = DbState::new_uncreated(user);
        u1.save(&transaction).await;
        transaction.commit().await.unwrap();
        u1.id
    }

    pub async fn get_alluser(&self, pagination: Pagination) -> Vec<User> {
        let users = User::all()
            .order_by_desc(|user| user.id)
            .limit(pagination.limit() as i64)
            .offset(pagination.offset() as i64)
            .run(&self.db)
            .await;
        let mut result = Vec::new();
        for u in users.unwrap() {
            result.push(u.into_inner());
        }
        result
    }

    pub async fn check_schema(&self) {
        // Get all the things that are different from the Order struct and the order table in the DB
        let diff = welds::check::schema::<User>(&self.db).await.unwrap();
        for d in &diff {
            println!("{}", d);
        }
    }
}
