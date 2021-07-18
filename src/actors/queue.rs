use actix::prelude::*;



use super::WsJudgeResult;
use super::Connect;
use super::WsJob;
use super::Judgers;
use super::Disconnect;

type Socket=Recipient<WsJob>;

use serde::{Deserialize,Serialize};
use bson::document::Document;
use bson::oid::ObjectId;
use mongodb::bson::doc;
use mongodb::Database;

//Actor definition
pub struct Queue{
    mongo:Database,
    judgers_addr:Addr<Judgers>,
    // queue:VecDeque

}

impl Actor for Queue{
    type Context=Context<Queue>;
}

impl Queue{
    pub fn new(
        mongo:Database
    )->Queue{
        Queue{
            mongo:mongo,
            judgers_addr:Judgers::default().start(),
        }
    }
    
    fn get_judgers(&self)->Addr<Judgers>{
        self.judgers_addr.clone()
    }
}

impl Handler<WsJudgeResult> for Queue{
    type Result=();

    fn handle(&mut self,msg:WsJudgeResult,_ctx:&mut Context<Self>){
        let WsJudgeResult(result)=msg;
        let judge_result:JudgeResultJson=serde_json::from_str(&result).unwrap();
        
        
        
        let fut=async move{
            let _result=update_judge_result(
                self.mongo.clone(),
                judge_result._id.clone(),
                judge_result.success,
                &judge_result.test_bench
            ).await.unwrap();
        };
        
        let _fut = actix::fut::wrap_future::<_, Self>(fut);
        //todo update pass
        
    }
}

impl Handler<Connect> for Queue{
    type Result=();

    fn handle(&mut self,msg:Connect,_ctx:&mut Context<Self>){
        self.judgers_addr.do_send(msg);
    }
    
}

impl Handler<Disconnect> for Queue{
    type Result=();

    fn handle(&mut self,msg:Disconnect,_ctx:&mut Context<Self>){
        self.judgers_addr.do_send(msg);
    }
    
}

impl Handler<WsJob> for Queue{
    type Result=();

    fn handle(&mut self,job:WsJob,_ctx:&mut Context<Self>){
        self.judgers_addr.do_send(job);
    }
    
}

#[derive(Debug,Serialize,Deserialize)]
pub struct JudgeResultJson{
    pub _id:ObjectId,
    pub success:bool,
    pub test_bench:Document,
    pub question_id:u32,
    pub user_id:u32
}


async fn update_judge_result(
    mongo:Database,
    object_id:ObjectId,
    is_success:bool,
    test_bench:&Document,
)->Result<(),()>{
    let collection=mongo.collection::<Document>("records");
    
    println!("{:?}",test_bench);
    
    if let Ok(_result)=collection.update_one(
        doc!{"_id":object_id,"success":doc!{"$exists":false}},
        doc!{
            "$set":{
                "success":is_success,
                "test_bench":test_bench
            }
        },
        None

    ).await{
        return Ok(());
    }
    Err(())
}
