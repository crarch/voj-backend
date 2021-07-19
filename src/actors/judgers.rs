use actix::{Actor};
use uuid::Uuid;
use std::collections::HashMap;
use actix::prelude::{Context, Handler, Recipient};

use super::{Connect,WsJob,Disconnect};

type Socket=Recipient<WsJob>;


pub struct Judgers{
    sessions:HashMap<Uuid,Socket>,
    judgers:Vec<(Uuid,Socket)>,
    iter:usize,
    judgers_count:usize
}

impl Default for Judgers{
    fn default()->Judgers{
        Judgers{
            sessions:HashMap::new(),
            judgers:Vec::new(),
            judgers_count:0,
            iter:0
        }
    }
}

impl Actor for Judgers {
    type Context = Context<Self>;
}


impl Handler<WsJob> for Judgers{
    type Result=();
    
    fn handle(&mut self,job:WsJob,_ctx:&mut Context<Self>)->Self::Result{
        self.send_job(job);
    }
}

impl Handler<Connect> for Judgers{
    type Result=();
    
    fn handle(&mut self,msg:Connect,_ctx:&mut Context<Self>)->Self::Result{
        self.sessions.insert(
            msg.self_id,
            msg.addr.clone(),
        );
        
        self.judgers.push(
            (msg.self_id,msg.addr)
        );
        
        self.judgers_count=self.judgers_count+1;
        
    }
    
}

impl Handler<Disconnect> for Judgers{
    type Result=();
    
    fn handle(&mut self,msg:Disconnect,_:&mut Context<Self>){
        self.sessions.remove(&msg.id);
        
        let index = self.judgers.iter().position(|&(addr,_)| addr == msg.id).unwrap();
        self.iter=0;
        
        self.judgers.remove(index);
        
        self.judgers_count=self.judgers_count-1;

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
    
    fn send_job(&mut self,job:WsJob){
        if(self.judgers_count==0){
            ()
        }else{
            if(self.iter==self.judgers_count){
                self.iter=0;
            }
            
            let (_,socket_recipient)=&self.judgers[self.iter];
            
            let _ = socket_recipient.do_send(job);
            
            self.iter=self.iter+1;
            
        }
        
    }
        

}
