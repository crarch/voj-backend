use actix::prelude::*;

use super::WsJudgeResult;
use super::Connect;
use super::WsJob;
use super::Judgers;
use super::Disconnect;

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

    fn handle(&mut self,msg:WsJudgeResult,ctx:&mut Context<Self>){
        let WsJudgeResult(result)=msg;
        let judge_result:JudgeResultJson=serde_json::from_str(&result).unwrap();
        let mongo=self.mongo.clone();
        
        let fut=async move{
            if(judge_result.success){
                
                let _result=update_pass_by_id(
                    mongo.clone(),
                    judge_result.question_id,
                    judge_result.user_id,
                ).await.unwrap();
                
            }
                
            let _result=update_judge_result(
                mongo,
                judge_result,
            ).await.unwrap();
        };
        
        let fut = actix::fut::wrap_future::<_, Self>(fut);
        ctx.spawn(fut);
        
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
    pub user_id:u32,
    pub code:String,
    pub submit_time:u32,
}


async fn update_judge_result(
    mongo:Database,
    result:JudgeResultJson
)->Result<(),()>{
    let collection=mongo.collection::<Document>("records");
    
    let doc=doc!{
        "_id":result._id,
        "success":result.success,
        "test_bench":result.test_bench,
        "question_id":result.question_id,
        "user_id":result.user_id,
        "code":result.code,
        "submit_time":result.submit_time
    };
    
    if let Ok(_result)=collection.insert_one(
        doc,
        None
    ).await{
        return Ok(());
    }
    Err(())
}


async fn update_pass_by_id(
    mongo:Database,
    user_id:u32,
    pass:u32
)->Result<(),()>{
    let collection=mongo.collection::<Document>("users");
    
    let result=collection.update_one(
        doc!{"_id":user_id},
        doc!{"$addToSet":{"pass":pass}},
        None,
    ).await;
    
    match result{
        Ok(_)=>Ok(()),
        Err(_)=>Err(()),
    }
}













