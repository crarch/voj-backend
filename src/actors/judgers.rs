use actix::{Actor};
use uuid::Uuid;
use std::collections::HashMap;
use actix::prelude::{Context, Handler, Recipient};

use super::{Connect,WsJob,Disconnect};

type Socket=Recipient<WsJob>;


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


impl Handler<WsJob> for Judgers{
    type Result=();
    
    fn handle(&mut self,msg:WsJob,_ctx:&mut Context<Self>)->Self::Result{
        let WsJob(msg)=msg;
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
        
    }
    
}

impl Handler<Disconnect> for Judgers{
    type Result=();
    
    fn handle(&mut self,msg:Disconnect,_:&mut Context<Self>){
        self.sessions.remove(&msg.id);
    }
    
}
        

impl Judgers {
    pub fn send_message_to_all(&self, message: &str) {
        self.sessions.iter().for_each(|(_,socket_recipient)| socket_recipient.do_send(WsJob(message.to_owned())).unwrap());
    }
    
    fn send_message(&self, message: &str, id_to: &Uuid) {
        if let Some(socket_recipient) = self.sessions.get(id_to) {
            let _ = socket_recipient
                .do_send(WsJob(message.to_owned()));
        } else {
            println!("no judger available");
        }
    }

}
