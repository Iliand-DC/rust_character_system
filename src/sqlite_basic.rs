use sqlx::{migrate::MigrateDatabase, Row, Sqlite, SqlitePool, FromRow, Pool, sqlite::SqliteQueryResult, sqlite::SqliteRow};
const DB_URL: &str = "sqlite://sqlite.db";
const DB_MAIN_STATS: &str = "db_main_stats";

#[derive(Clone, FromRow, Debug)]
struct UserMainInfo {
    chara_id: i64,
    name: String,
    class: String,
    race: String
}

#[derive(Clone, FromRow, Debug)]
struct PersonMainStats {
    chara_id: i64,
    damage: i64,
    hp: i64
}

#[allow(dead_code)]
impl PersonMainStats {
    pub fn get_chara_id(&self) -> i64 {
        self.chara_id
    }
    pub fn get_damage(&self) -> i64 {
        self.damage
    }
    pub fn get_hp(&self) -> i64 {
        self.hp
    }
    pub fn set_damage(&mut self, new_dmg:i64) {
        self.damage = new_dmg;
    }
    pub fn set_hp(&mut self, new_hp:i64) {
        self.hp = new_hp;
    }
    async fn get_info_by_id(&mut self, db: &Pool<Sqlite>, chara_id:i64) 
    {
        let input_text: String = format!("SELECT * FROM {DB_MAIN_STATS} WHERE chara_id = {chara_id}");
        let user_results: Vec<PersonMainStats> = sqlx::query_as::<_, PersonMainStats>(&input_text)
        .fetch_all(db).await.unwrap();
        let get_result = user_results[0].clone();
        *self = get_result.clone();

    }
}

#[allow(dead_code)]
async fn create_db()
{
    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        println!("Creating database {}", DB_URL);
        match Sqlite::create_database(DB_URL).await {
            Ok(_) => println!("Create db success"),
            Err(error) => panic!("error: {}", error),
        }
    } else {
        println!("Database already exists");
    }
}

#[allow(dead_code)]
async fn create_table(db: &Pool<Sqlite>, name_table: String, data_types: String)
{
    let input_text = format!("CREATE TABLE IF NOT EXISTS {name_table} ({data_types});");
    let result = sqlx::query(&input_text).execute(db).await.unwrap();
    println!("Create user table result: {:?}", result);
}

#[allow(dead_code)]
async fn print_all_shit(db: &Pool<Sqlite>, table_name:String)
{
    let cmd_sql = format!("SELECT * FROM {table_name}");
    let user_results = sqlx::query_as::<_, UserMainInfo>(&cmd_sql)
    .fetch_all(db)
    .await
    .unwrap();
    for user in user_results {
        println!("[{}] name: {}   class: {}   race:  {}", user.chara_id, &user.name, &user.class, &user.race);
    }
}

