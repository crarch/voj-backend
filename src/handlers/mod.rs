pub mod session;
pub mod profile;
pub mod testbench;
pub mod judge;
pub mod queue;

use crate::env;

use actix_web::{HttpResponse,Error,get};
use anyhow::Result;

#[get("/version")]
pub async fn get_version()->Result<HttpResponse,Error>{
    Ok(HttpResponse::Ok().body(format!("{{\"version\":\"{}\"}}",env::VERSION)))
}


use actix::{Actor,StreamHandler};
use actix_web::{web,App,HttpRequest,HttpServer};
use actix_web_actors::ws;
use std::time::{Duration,Instant};
use actix::{fut,WrapFuture,ActorFutureExt,ContextFutureSpawner};


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

#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
    pub addr: Recipient<WsMessage>,
    pub self_id: Uuid,
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



#[get("/websocket")]
pub async fn get_websocket(
    req:HttpRequest,
    stream:web::Payload,
    judgers:Data<Addr<Judgers>>
)->Result<HttpResponse,Error>{
    // panic!("");
    let ws=JudgerWs::new(
        judgers.get_ref().clone()
    );
    
    let resp=ws::start(ws,&req,stream);
    resp
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

use actix::prelude::{Context, Handler, Recipient,Message};
use actix::{AsyncContext};

type Socket=Recipient<WsMessage>;

#[derive(Message)]
#[rtype(result="()")]
pub struct WsMessage(pub String);

pub struct Judgers{
    sessions:HashMap<Uuid,Socket>,
}


impl Default for Judgers{
    fn default()->Judgers{
        Judgers{
            sessions:HashMap::new(),
        }
    }
}

use uuid::Uuid;
use std::collections::HashMap;
use actix::Addr;
use tokio::time::{self};
use actix_web::web::Data;

pub async fn call_back(
    srv: Data<Addr<Judgers>>
){
    srv.do_send(WsMessage("hihi".to_string()));
    // actix_rt::spawn(async move {
    //     let mut interval = time::interval(Duration::from_secs(2));
    //     loop {
    //         interval.tick().await;
    //         srv.do_send(WsMessage("hihi".to_string()));
    //     }
    // });
}


impl Actor for Judgers {
    type Context = Context<Self>;
}


impl Handler<WsMessage> for Judgers{
    type Result=();
    
    fn handle(&mut self,msg:WsMessage,_ctx:&mut Context<Self>)->Self::Result{
        let WsMessage(msg)=msg;
        self.send_message_to_all(&msg);
    }
}

impl Handler<Connect> for Judgers{
    type Result=();
    
    fn handle(&mut self,msg:Connect,_ctx:&mut Context<Self>)->Self::Result{
        self.sessions.insert(
            msg.self_id,
            msg.addr,
        );
        
        self.send_message(&format!("your id is {}", msg.self_id), &msg.self_id);
    }
    
}

impl Judgers {
    pub fn send_message_to_all(&self, message: &str) {
        self.sessions.iter().for_each(|(_,socket_recipient)| socket_recipient.do_send(WsMessage(message.to_owned())).unwrap());
    }
    
    fn send_message(&self, message: &str, id_to: &Uuid) {
        if let Some(socket_recipient) = self.sessions.get(id_to) {
            let _ = socket_recipient
                .do_send(WsMessage(message.to_owned()));
        } else {
            println!("attempting to send message but couldn't find user id.");
        }
    }

}

impl Handler<WsMessage> for JudgerWs {
    type Result = ();

    fn handle(&mut self, msg: WsMessage, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}