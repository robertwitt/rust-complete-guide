use crate::links::{LinkId, Links};
use actix::Actor;
use actix::Addr;
use actix::Handler;
use actix::Message;
use actix::SyncArbiter;
use actix::SyncContext;
use actix_web::{error, Error};
use serde_derive::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

const DB_THREADS: usize = 3;

#[derive(Clone)]
pub struct Db {
    pub inner: Arc<Mutex<Links>>,
}

impl Db {
    pub fn new(s: Arc<Mutex<Links>>) -> Db {
        Db { inner: s }
    }
}

impl Actor for Db {
    type Context = SyncContext<Self>;
}

#[derive(Clone)]
pub struct State {
    pub inner: Addr<Db>,
}

impl State {
    pub fn init() -> Self {
        let state = Arc::new(Mutex::new(Links::new()));
        let state = SyncArbiter::start(DB_THREADS, move || Db::new(state.clone()));
        let state = State { inner: state };
        state
    }

    pub fn get(&self) -> &Addr<Db> {
        &&self.inner
    }
}

pub struct GetLinks;

impl Message for GetLinks {
    type Result = Result<String, Error>;
}

impl Handler<GetLinks> for Db {
    type Result = Result<String, Error>;

    fn handle(&mut self, _mssg: GetLinks, _: &mut Self::Context) -> Self::Result {
        Ok(self.inner.lock().unwrap().links())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddLink {
    pub title: String,
    pub url: String,
}

impl Message for AddLink {
    type Result = Result<(), Error>;
}

impl Handler<AddLink> for Db {
    type Result = Result<(), Error>;

    fn handle(&mut self, new_link: AddLink, _: &mut Self::Context) -> Self::Result {
        let mut db_ref = self.inner.lock().unwrap();
        db_ref.add_link(new_link);
        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
pub struct RmLink {
    pub id: LinkId,
}

impl Message for RmLink {
    type Result = Result<LinkId, Error>;
}

impl Handler<RmLink> for Db {
    type Result = Result<LinkId, Error>;

    fn handle(&mut self, link: RmLink, _: &mut Self::Context) -> Self::Result {
        let mut db_ref = self.inner.lock().unwrap();
        db_ref
            .rm_link(link.id)
            .ok_or(error::ErrorInternalServerError("Failed to remove links"))
    }
}
