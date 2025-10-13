use spring::plugin::service::Service;

use crate::{domain::model::User, welds::welds::WeldsClient};


#[derive(Clone, Service)]
pub struct UserDao {
    #[inject(component)]
    pub db: WeldsClient,
}

impl UserDao {
    pub async fn get_user(&self, id: i64) -> User {
        self.check_schema().await;
        let u = User::find_by_id(&self.db, id).await.unwrap();
        //        let users = User::all()
        // .where_col(|product| product.price.equal(3.50))
        // .map_query(|product| product.seller )
        // .where_col(|seller| seller.name.ilike("%Nessie%") )
        //     .order_by_desc(|user| user.id )
        //     .limit( 10 )
        //     .run(&pool).await;

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


    pub async fn check_schema(&self) {
    // Get all the things that are different from the Order struct and the order table in the DB
    let diff = welds::check::schema::<User>(&self.db).await.unwrap();
    for d in &diff {
        println!("{}", d);
    }

    }
}
