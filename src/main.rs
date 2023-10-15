use std::convert::Infallible;
use std::net::SocketAddr;

use hyper::{Request, Response, Body, Server};
use hyper::{Method, StatusCode};
use hyper::service::{service_fn, make_service_fn};

use mysql::prelude::Queryable;

use json::JsonValue;

#[derive(Debug, PartialEq, Eq)]
struct MethodEntry {
    id: i32,
    name: i32,
    description: Option<String>,
    image: Option<String>,
}

impl From<MethodEntry> for JsonValue {
    fn from(method_entry : MethodEntry) -> JsonValue {
        let mut object = JsonValue::new_object();
        object["id"] = method_entry.id.into();
        object["name"] = method_entry.name.into();
        object["description"] = method_entry.description.into();
        object["image"] = method_entry.image.into();
        return object;
    }
}

fn handle_rows(_req : &Request<Body>) -> Result<Response<Body>, Infallible> {
    let mut response = Response::new(Body::empty());
    
    let pool = mysql::Pool::new("mysql://AzureDiamond:hunter2@localhost/sweng?socket=/run/mysqld/mysqld.sock&prefer_socket=true");
    let pool = match pool {
        Ok(pool) => pool,
        Err(e) => {
            eprintln!("{:#?}", e);
            *response.body_mut() = "{\"result\":\"err\", \"value\":\"could not connect to database\"}".into();
            *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
            return Ok(response);
        },
    };
    
    let conn = pool.get_conn();
    let mut conn = match conn {
        Ok(conn) => conn,
        Err(e) => {
            eprintln!("{:#?}", e);
            *response.body_mut() = "{\"result\":\"err\", \"value\":\"could not connect to database\"}".into();
            *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
            return Ok(response);
        },
    };

    let selected_rows = conn.query_map(
        "SELECT id, name, description, image FROM methods",
        |(id, name, description, image)| {
            MethodEntry {id, name, description, image}
        }
    );

    let Ok(selected_rows) = selected_rows else {
        *response.body_mut() = "{\"result\":\"err\", \"value\":\"database returned bad data I guess\"}".into();
        *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
        return Ok(response);
    };
    
    let body = json::stringify(selected_rows);
    *response.body_mut() = body.into();
    return Ok(response);
}

async fn handle_request(req : Request<Body>) -> Result<Response<Body>, Infallible> {
    
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/api/rows") => {
            return handle_rows(&req);
        },
        _ => {
            let mut response = Response::new(Body::empty());
            *response.status_mut() = StatusCode::NOT_FOUND;
            *response.body_mut() = Body::from("<!DOCTYPE html><html><meta charset=\"utf-8\"><head><title>Not found</title></head><body><p>The requested resource does not exist on this server.</p></body></html>");
            return Ok(response);
        }
    }

}

#[tokio::main]
async fn main() {
    let socket_addr = SocketAddr::from(([127, 0, 0, 1], 12181));

    let make_callback = make_service_fn(|_connection| async {
        Ok::<_, Infallible>(service_fn(handle_request))
    });

    let server = Server::bind(&socket_addr).serve(make_callback);

    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }
}
