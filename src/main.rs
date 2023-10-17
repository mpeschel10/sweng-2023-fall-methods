use std::collections::HashMap;
use std::convert::Infallible;
use std::net::SocketAddr;

use hyper::{Request, Response, Body, Server};
use hyper::{Method, StatusCode};
use hyper::service::{service_fn, make_service_fn};

use mysql::prelude::Queryable;

use json::JsonValue;

use sea_query::{Query, Iden,};
use sea_query::*;
use sea_query::types::Order;

#[derive(Debug, PartialEq, Eq)]
struct MethodEntry {
    id: i32,
    name: i32,
    description: Option<String>,
    image: Option<String>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum MethodsColumns {
    Table,
    Id,
    Name,
    Description,
    Image,
}

impl Iden for MethodsColumns {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        write!(s, "{}", match self {
            Self::Table => "methods",
            Self::Id => "id",
            Self::Name => "name",
            Self::Description => "description",
            Self::Image => "image",
        }).unwrap();
    }
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

fn get_params(request : &Request<Body>) -> HashMap<String, Vec<String>> {
    let query_string = request.uri().query().unwrap_or("");
    let query_pairs = form_urlencoded::parse(query_string.as_bytes()).into_owned();

    let mut params: HashMap<String, Vec<String>> = HashMap::new();
    for (key, value) in query_pairs {
        let my_key = key.clone(); // TODO There's got to be a better way
        if !params.contains_key(&key) {
            params.insert(key, Vec::new());
        }
        let values : &mut Vec<String> = params.get_mut(&my_key).unwrap();
        values.push(value);
    }
    println!("Parsed request params: {params:#?}");
    return params;
}

#[derive(Debug)]
struct RowsParams {
    order_by: MethodsColumns, // TODO this allows a nonexistent column, "Table = methods"
    order: Order,
}

fn normalize_rows_params(params : HashMap<String, Vec<String>>) -> RowsParams {
    let mut normal_params : RowsParams = RowsParams {
        order_by: MethodsColumns::Id,
        order: Order::Asc,
    };

    if let Some(v) = params.get("order-by") {
        if let Some(v) = v.get(0) {
            match v.as_str() {
                "id" => { normal_params.order_by = MethodsColumns::Id; },
                "name" => { normal_params.order_by = MethodsColumns::Name; },
                "description" => { normal_params.order_by = MethodsColumns::Description; },
                "image" => { normal_params.order_by = MethodsColumns::Image; },
                &_ => { },
            }
        }
    }

    if let Some(v) = params.get("desc") {
        if let Some(v) = v.get(0) {
            match v.as_str() {
                "on" | "true" => {
                    normal_params.order = Order::Desc;
                },
                &_ => { },
            }
        }
    }

    return normal_params;
}

fn handle_rows(req : &Request<Body>) -> Result<Response<Body>, Infallible> {
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

    let params = normalize_rows_params(get_params(&req));

    let query = Query::select()
        .column(MethodsColumns::Id)
        .column(MethodsColumns::Name)
        .column(MethodsColumns::Description)
        .column(MethodsColumns::Image)
        .from(MethodsColumns::Table)
        .order_by(params.order_by, params.order)
        // .conditions(
        //     false,
        //     |q| {
        //         q.and_where(Expr::col(MethodsColumns::Id).eq(1));
        //     },
        //     |_q| {},
        // )
        .to_string(MysqlQueryBuilder);
    
    // println!("Params dict: {:#?}", params);
    
    let selected_rows = conn.query_map(
        query,
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
