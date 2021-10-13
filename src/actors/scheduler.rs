use actix::{Actor};
use uuid::Uuid;
use actix::prelude::{Context, Handler, Recipient};

use super::{Connect,WsJob,Disconnect};

type Socket=Recipient<WsJob>;


pub struct Scheduler{
    judgers:Vec<(Uuid,Socket)>,
    iter:usize,
    judgers_count:usize
}

impl Default for Scheduler{
    fn default()->Scheduler{
        Scheduler{
            judgers:Vec::new(),
            judgers_count:0,
            iter:0
        }
    }
}

impl Actor for Scheduler {
    type Context = Context<Self>;
}


impl Handler<WsJob> for Scheduler{
    type Result=();
    
    fn handle(&mut self,job:WsJob,_ctx:&mut Context<Self>)->Self::Result{
        self.send_job(job);
    }
}

impl Handler<Connect> for Scheduler{
    type Result=();
    
    fn handle(&mut self,msg:Connect,_ctx:&mut Context<Self>)->Self::Result{
        
        self.judgers.push(
            (msg.self_id,msg.addr)
        );
        
        self.judgers_count=self.judgers_count+1;
        
    }
    
}

impl Handler<Disconnect> for Scheduler{
    type Result=();
    
    fn handle(&mut self,msg:Disconnect,_:&mut Context<Self>){
        if let Some(index) = self.judgers.iter().position(|&(judger_id,_)| judger_id == msg.id){
            self.iter=0;
            
            self.judgers.remove(index);
            
            self.judgers_count=self.judgers_count-1;
        }

    }
    
}
        

impl Scheduler {
    
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
