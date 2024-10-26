use std::sync::Mutex;
use actix_web::web::{Json, Path, Data};
use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};
use rsocket_rust::prelude::*;
use rsocket_rust::Result;
use rsocket_rust_transport_tcp::TcpClientTransport;
use crate::configs;
use crate::lb_algotithms::{self, get_next_node_with_lb_strategy};


#[derive(Serialize, Deserialize,Debug)]
struct PostQuery {
    method: String,
    query: String
}

#[derive(Serialize, Deserialize,Debug)]
struct PostData {
    query: String
}

#[post("/execute_query")]
pub async fn execute_query(data: Json<PostData> ) -> HttpResponse {
    // TODO find tweet a tweet by ID and return it
    let text_result: &str = data.query.as_str();
    match text_result {
        "res" => HttpResponse::Ok()
            .content_type("application/json")
            .json(text_result),
        "" => HttpResponse::NoContent()
            .content_type("application/json")
            .await
            .unwrap(),
        _ => panic!("Error")
    }
}


// #[post("/execute_query_method")]
// pub async fn execute_query_method(data: Json<PostQuery> ) -> HttpResponse {
//     // TODO find tweet a tweet by ID and return it
//     print!("execute_query_method");
    
//     let result = executeInCluster(&data.method, &data.query);
    
//     print!("Got here!!!");

//     match result {
//         Ok(_) => {
//             HttpResponse::Ok()
//             .content_type("application/json")
//             .json(result.unwrap().as_str())
//         },
//         Err(err) => {
//             println!("input: {}", err);
//             HttpResponse::Ok()
//             .content_type("application/json")
//             .json(err.to_string())
//         }
//     }
// }

#[post("/execute_query_method")]
pub async fn execute_query_method(data: Json<PostQuery>, current_node: Data<Mutex<u8>> ) -> HttpResponse {
    // TODO find tweet a tweet by ID and return it
    print!("execute_query_method");
    let peers = configs::read_config_file().unwrap();
    let rt = tokio::runtime::Runtime::new().unwrap();
    let mut lock_result = current_node.lock().unwrap();
    let peer = get_next_node_with_lb_strategy(lb_algotithms::LbAlgotithms::RoundRobin, peers, &mut lock_result);
    let return_value: Result<String> = rt.block_on(execute_in_cluster(&data.method, &data.query, peer));
    
    match return_value {
        Err(err) => {
            println!("input: {}", err);
            HttpResponse::Ok()
            .content_type("application/json")
            .json(err.to_string())
        }
        ,
        _ => {
            HttpResponse::Ok()
            .content_type("application/json")
            // .json((*lock_result).to_string())
            .json(return_value.unwrap().as_str())
        }
        
    }
    
}

#[get("/request_methods/{id}")]
pub async fn get(path: Path<String>) -> HttpResponse {
    // TODO find tweet a tweet by ID and return it
    let id = path.into_inner();
    let text_result: &str = id.as_str();
    
    match text_result {
        "res" => HttpResponse::Ok()
            .content_type("application/json")
            .json(text_result),
        "" => HttpResponse::NoContent()
            .content_type("application/json")
            .await
            .unwrap(),
        _ => panic!("Error")
    }
}

//#[tokio::main]
async fn execute_in_cluster(name_method: &str, data_json: &str, peer: configs::Peer) -> Result<String>{
    
    let name_peer = peer.name;
    let port_peer = peer.port;
    let host_peer = peer.ip;
    let host_server = format!("{host_peer}:{port_peer}");
    
    let cli = RSocketFactory::connect()
        .transport(TcpClientTransport::from(host_server))
        .setup(Payload::from("READY!"))
        .mime_type("text/plain", "text/plain")
        .on_close(Box::new(|| println!("connection closed")))
        .start()
        .await?;
    
    let method = "{\"method\":\"execute_something\"}";
    let data = format!("{{\"method\":\"/{name_peer}/{name_method}\",\"payload\":{{\"query\":\"{data_json}\"}}}}");
    let req = Payload::builder()
        .set_data_utf8(&data)
        .set_metadata_utf8(method)
        .build();
    let res = cli.request_response(req).await?;
    //println!("got: {:?}", res);
    let result2 = res.unwrap();
    let result1 = result2.data_utf8().unwrap();
    

    // If you want to block until socket disconnected.
    // cli.wait_for_close().await;
    Ok(String::from(result1))
}