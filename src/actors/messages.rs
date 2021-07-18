use actix_web::{HttpResponse,Error,get};
use actix::{Actor,StreamHandler};
use actix_web::{web,App,HttpRequest,HttpServer};
use actix_web_actors::ws;
use std::time::{Duration,Instant};
use actix::{fut,WrapFuture,ActorFutureExt,ContextFutureSpawner};
use uuid::Uuid;
use std::collections::HashMap;
use actix::Addr;
use tokio::time::{self};
use actix_web::web::Data;
use actix::prelude::{Context, Handler, Recipient,Message};
use actix::{AsyncContext};

type Socket=Recipient<WsMessage>;

#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
    pub addr: Recipient<WsMessage>,
    pub self_id: Uuid,
}

#[derive(Message)]
#[rtype(result="()")]
pub struct WsMessage(pub String);

