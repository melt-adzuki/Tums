use dotenv::dotenv;
use once_cell::sync::Lazy;
use std::env::var;

#[derive(Debug)]
pub(crate) struct Confs {
    pub(crate) db_host: String,
    pub(crate) db_username: String,
    pub(crate) db_password: String,
    pub(crate) mk_endpnt: String,
    pub(crate) mk_token: String,
    pub(crate) mk_tlcat: String,
}

impl Default for Confs {
    fn default() -> Self {
        Self {
            db_host: "localhost:27017".to_string(),
            db_username: "root".to_string(),
            db_password: "password".to_string(),
            mk_endpnt: "submarin.online".to_string(),
            mk_token: "".to_string(),
            mk_tlcat: "hybridTimeline".to_string(),
        }
    }
}

pub(crate) static CONFS: Lazy<Confs> = Lazy::new(|| {
    dotenv().ok();

    let default: Confs = Confs::default();

    Confs {
        db_host: var("DB_HOST").unwrap_or(default.db_host),
        db_username: var("DB_USERNAME").unwrap_or(default.db_username),
        db_password: var("DB_PASSWORD").unwrap_or(default.db_password),
        mk_endpnt: var("INSTANCE").unwrap_or(default.mk_endpnt),
        mk_token: var("TOKEN").unwrap(),
        mk_tlcat: var("TIMELINE").unwrap_or(default.mk_tlcat),
    }
});
