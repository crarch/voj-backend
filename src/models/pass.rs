use mongodb::bson::doc;
use bson::document::Document;
use crate::MongoDB;

pub fn get_pass_by_id(mongo:MongoDB,user_id:u32)->Result<Document,()>{
    let collection=mongo.collection::<Document>("users");

    let cursor=collection.find_one(
        doc!{"_id":user_id},
        mongodb::options::FindOneOptions::builder()
            .projection(Some(doc!{"pass":1,"_id":0}))
            .build()
    ).unwrap();

    let result=cursor.unwrap();
    
    Ok(result)
}

pub fn add_pass_by_id(mongo:MongoDB,user_id:u32,pass:u32)->Result<(),()>{
    let collection=mongo.collection::<Document>("users");
    
    let result=collection.update_one(
        doc!{"_id":user_id},
        doc!{"$addToSet":{"pass":pass}},
        None,
    );
    
    match result{
        Ok(_)=>Ok(()),
        Err(_)=>Err(()),
    }
}
