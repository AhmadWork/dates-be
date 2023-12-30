use dotenvy_macro::dotenv;
use dates_backend::config::Config;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let jwt_secret = dotenv!("JWT_SECRET").to_owned();
    let port = dotenv!("PORT")
        .parse()
        .expect("unable to parse port into a number");
    let database_uri = dotenv!("DATABASE_URL");
    let config = Config::new(jwt_secret, port, database_uri);

    match dates_backend::run(config).await {
        Ok(_) => println!("app running on port 3000"),
        Err(error) => eprintln!("Error: {:?}", error),

    }
}
