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

use super::WsMessage;
use super::Connect;
use super::Judgers;

type Socket=Recipient<WsMessage>;

impl Actor for JudgerWs{
    type Context=ws::WebsocketContext<Self>;
    
    fn started(&mut self, ctx: &mut Self::Context) {
    
        let addr = ctx.address(); 
        self.addr
            .send(Connect {
                addr: addr.recipient(),
                self_id: self.id,
            })
            .into_actor(self)
            .then(|res, _, ctx| {
                match res {
                    Ok(_res) => (),
                    _ => (),
                }
                fut::ready(())
            })
            .wait(ctx);
    }
}

pub struct JudgerWs {
    hb: Instant,
    id:Uuid,
    addr:Addr<Judgers>
}

impl JudgerWs{
    pub fn new(addr:Addr<Judgers>)->JudgerWs{
        JudgerWs{
            hb:Instant::now(),
            id:Uuid::new_v4(),
            addr:addr
        }
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for JudgerWs {
    fn handle(
        &mut self,
        msg: Result<ws::Message, ws::ProtocolError>,
        ctx: &mut Self::Context,
    ) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}

impl Handler<WsMessage> for JudgerWs {
    type Result = ();

    fn handle(&mut self, msg: WsMessage, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}
