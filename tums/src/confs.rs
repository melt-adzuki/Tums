use dotenv::dotenv;
use once_cell::sync::Lazy;
use std::env::var;

#[derive(Debug)]
pub(crate) struct Confs {
    pub(crate) db_host: String,
    pub(crate) mk_endpnt: String,
    pub(crate) mk_token: String,
    pub(crate) mk_tlcat: String,
}

impl Default for Confs {
    fn default() -> Self {
        Self {
            db_host: "localhost:27017".to_string(),
            mk_endpnt: "submarin.online".to_string(),
            mk_token: "".to_string(),
            mk_tlcat: "localTimeLine".to_string(),
        }
    }
}

pub(crate) static CONFS: Lazy<Confs> = Lazy::new(|| {
    dotenv().ok();

    Confs {
        db_host: var("DB_HOST").unwrap_or(Confs::default().db_host),
        mk_endpnt: var("INSTANCE").unwrap_or(Confs::default().mk_endpnt),
        mk_token: var("TOKEN").unwrap(),
        mk_tlcat: var("TIMELINE").unwrap_or(Confs::default().mk_tlcat),
    }
});
