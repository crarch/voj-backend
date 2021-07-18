use actix::{Actor,StreamHandler};
use actix_web_actors::ws;
use std::time::{Instant};
use actix::{fut,WrapFuture,ActorFutureExt,ContextFutureSpawner};
use uuid::Uuid;
use actix::Addr;
use actix::prelude::{Handler, Recipient};
use actix::{AsyncContext};

use super::WsJob;
use super::WsJudgeResult;
use super::Connect;
use super::Judgers;
use super::Queue;

type Socket=Recipient<WsJob>;

impl Actor for JudgerWs{
    type Context=ws::WebsocketContext<Self>;
    
    fn started(&mut self, ctx: &mut Self::Context) {
    
        let addr = ctx.address(); 
        self.queue_addr
            .send(Connect {
                addr: addr.recipient(),
                self_id: self.id,
            })
            .into_actor(self)
            .then(|res, _, _ctx| {
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
    queue_addr:Addr<Queue>,
}

impl JudgerWs{
    pub fn new(
        queue_addr:Addr<Queue>,
    )->JudgerWs{
        JudgerWs{
            hb:Instant::now(),
            id:Uuid::new_v4(),
            queue_addr:queue_addr
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
            Ok(ws::Message::Text(text)) =>{
                //todo send to queue handle actor
                self.queue_addr.send(WsJudgeResult(text.to_string()))
                .into_actor(self)
                .then(|res, _, _ctx| {
                    match res {
                        Ok(_res) => (),
                        _ => (),
                    }
                    fut::ready(())
                })
                .wait(ctx);
            }, 
            // Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}

impl Handler<WsJob> for JudgerWs {
    type Result = ();

    fn handle(&mut self, msg: WsJob, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}
