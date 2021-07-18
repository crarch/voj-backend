use actix::{Actor,StreamHandler,ActorContext,Running};
use actix_web_actors::ws;
use actix::{fut,WrapFuture,ActorFutureExt,ContextFutureSpawner};
use uuid::Uuid;
use actix::Addr;
use actix::prelude::{Handler, Recipient};
use actix::{AsyncContext};
use std::time::{Duration, Instant};

use super::WsJob;
use super::WsJudgeResult;
use super::Connect;

use super::Queue;
use super::Disconnect;

type Socket=Recipient<WsJob>;

impl Actor for JudgerWs{
    type Context=ws::WebsocketContext<Self>;
    
    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
    
        println!("Judger {} Connection Established",&self.id);
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
    
    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        self.queue_addr.do_send(Disconnect { id: self.id });
        Running::Stop
    }
}

pub struct JudgerWs {
    hb:Instant,
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

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

impl JudgerWs{
    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                println!("Judger {} Disconnecting failed heartbeat",&act.id);
                act.queue_addr.do_send(Disconnect { id: act.id });
                ctx.stop();
                return;
            }
                
            ctx.ping(b"hi");
        });
    }
}


impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for JudgerWs {
    fn handle(
        &mut self,
        msg: Result<ws::Message, ws::ProtocolError>,
        ctx: &mut Self::Context,
    ) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            },
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            },
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
                println!("Judger {} Disconnecting",self.id);
            },
            Ok(ws::Message::Text(text)) =>{
                self.hb = Instant::now();
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
