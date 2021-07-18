use actix::{Actor};
use uuid::Uuid;
use std::collections::HashMap;
use actix::prelude::{Context, Handler, Recipient};

use super::{Connect,WsMessage};


type Socket=Recipient<WsMessage>;


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
        
        // self.send_message(&format!("your id is {}", msg.self_id), &msg.self_id);
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

    // pub fn send_job_to_all(&self, job: Bson) {
    //     self.sessions.iter().for_each(|(_,socket_recipient)| socket_recipient.do_send(WsMessage(job.to_string())).unwrap());
    // }
}

use bson::Bson;
use bson::document::Document;
use super::messages::Job;
