use actix::prelude::Handler;
use actix::Addr;
use actix::AsyncContext;
use actix::{fut, ActorFutureExt, ContextFutureSpawner, WrapFuture};
use actix::{Actor, ActorContext, Running, StreamHandler};
use actix_web_actors::ws;
use log::warn;
use std::time::{Duration, Instant};
use uuid::Uuid;

use super::Connect;
use super::Disconnect;
use super::Job;
use super::Queue;
use super::WsJob;
use super::WsJudgeResult;

impl Actor for JudgerWs {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);

        warn!("Judger {} Connection Established", &self.id);
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
        warn!("Judger {} Disconnecting", self.id);
        self.queue_addr.do_send(Disconnect { id: self.id });
        Running::Stop
    }
}

pub struct JudgerWs {
    hb: Instant,
    id: Uuid,
    queue_addr: Addr<Queue>,
}

impl JudgerWs {
    pub fn new(queue_addr: Addr<Queue>) -> JudgerWs {
        JudgerWs {
            hb: Instant::now(),
            id: Uuid::new_v4(),
            queue_addr: queue_addr,
        }
    }
}

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(2);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(5);

impl JudgerWs {
    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                act.queue_addr.do_send(Disconnect { id: act.id });
                ctx.stop();
                return;
            }

            ctx.ping(b"hihi");
            ctx.ping(b"hihi");
        });
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for JudgerWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            Ok(ws::Message::Text(text)) => {
                self.hb = Instant::now();
                let judge_result: Job = serde_json::from_str(&text).unwrap();
                self.queue_addr.do_send(WsJudgeResult(judge_result));
            }
            unexpected => {
                println!("{:?}", unexpected);
                panic!()
            }
        }
    }
}

impl Handler<WsJob> for JudgerWs {
    type Result = ();

    fn handle(&mut self, job: WsJob, ctx: &mut Self::Context) {
        let WsJob(job) = job;
        let job = serde_json::to_string(&job).unwrap();
        ctx.text(job);
    }
}
