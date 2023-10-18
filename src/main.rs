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
    name: String,
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
        let values : &mut Vec<String> = params.get_mut(&my_key).expect("Inserted vector into dictionary and that vector was immediately unreachable. Are we multithreaded now???");
        values.push(value);
    }
    // println!("Parsed request params: {params:#?}");
    return params;
}

#[derive(Debug)]
struct RowsParams {
    order_by: MethodsColumns, // TODO this allows a nonexistent column, "Table = methods"
    order: Order,
    where_id: bool,
    id: i32,
    where_keyword: bool,
    keyword: String,
}

// TODO this should probably be a ::from interface on RowsParams from HashMap<>
fn normalize_rows_params(params : HashMap<String, Vec<String>>) -> RowsParams {
    let mut normal_params : RowsParams = RowsParams {
        order_by: MethodsColumns::Id,
        order: Order::Asc,
        where_id: false,
        id: 1,
        where_keyword: false,
        keyword: String::new(),
    };

    if let Some(v) = params.get("order_by") {
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

    if let Some(v) = params.get("where_id") {
        if let Some(v) = v.get(0) {
            match v.parse::<i32>() {
                Ok(id) => {
                    normal_params.where_id = true;
                    normal_params.id = id;
                },
                _ => {
                    normal_params.where_id = true;
                    normal_params.id = 0; // Never valid for AUTO_INCREMENT table
                 },
            }
        }
    }

    if let Some(v) = params.get("where_keyword") {
        if let Some(v) = v.get(0) {
            normal_params.where_keyword = true;
            normal_params.keyword = v.clone();
        }
    }

    // println!("Normalized params to {normal_params:#?}");

    return normal_params;
}

fn insert(method : MethodEntry) {
    let pool = mysql::Pool::new("mysql://AzureDiamond:hunter2@localhost/sweng?socket=/run/mysqld/mysqld.sock&prefer_socket=true");
    let pool = match pool {
        Ok(pool) => pool,
        Err(e) => {
            return;
        },
    };
    
    let conn = pool.get_conn();
    let mut conn = match conn {
        Ok(conn) => conn,
        Err(e) => {
            return;
        },
    };

    let query = Query::insert()
        .into_table(MethodsColumns::Table)
        .columns([MethodsColumns::Name, MethodsColumns::Description, MethodsColumns::Image])
        .values_panic([method.name.into(), method.description.into(), method.image.into()])
        .to_string(MysqlQueryBuilder);
    
    println!("Running query: {query:#?}");
    let result = conn.exec_drop(query, ());
    println!("Insert result: {result:#?}");
}

fn update(method : MethodEntry) {
    let pool = mysql::Pool::new("mysql://AzureDiamond:hunter2@localhost/sweng?socket=/run/mysqld/mysqld.sock&prefer_socket=true");
    let pool = match pool {
        Ok(pool) => pool,
        Err(e) => {
            return;
        },
    };
    
    let conn = pool.get_conn();
    let mut conn = match conn {
        Ok(conn) => conn,
        Err(e) => {
            return;
        },
    };

    let query = Query::update()
        .table(MethodsColumns::Table)
        .values([(MethodsColumns::Name, method.name.into()), (MethodsColumns::Description, method.description.into()), (MethodsColumns::Image, method.image.into())])
        .and_where(Expr::col(MethodsColumns::Id).eq(method.id))
        .to_string(MysqlQueryBuilder);
    
    println!("Running query: {query:#?}");
    let result = conn.exec_drop(query, ());
    println!("Insert result: {result:#?}");
}

async fn handle_put_row(req : &mut Request<Body>) -> Result<Response<Body>, Infallible> {
    let arguments = get_params(req);
    println!("POST /api/row params: {arguments:#?}");
    let id = arguments.get("field-edit-id").unwrap().get(0).unwrap().parse::<i32>().unwrap();
    let name = arguments.get("field-edit-name").unwrap().get(0).unwrap().to_string();
    let description = arguments.get("field-edit-description").unwrap().get(0).unwrap().to_string();
    let image = arguments.get("field-edit-image").unwrap().get(0).unwrap().to_string();
    
    let method = MethodEntry { id: id, name: name, description: Some(description), image: Some(image)};
    
    update(method);
    
    Ok(Response::new("Howdy!".into()))
}

fn delete(id : i32) {
    let pool = mysql::Pool::new("mysql://AzureDiamond:hunter2@localhost/sweng?socket=/run/mysqld/mysqld.sock&prefer_socket=true");
    let pool = match pool {
        Ok(pool) => pool,
        Err(_e) => {
            return;
        },
    };
    
    let conn = pool.get_conn();
    let mut conn = match conn {
        Ok(conn) => conn,
        Err(_e) => {
            return;
        },
    };

    let query = Query::delete()
        .from_table(MethodsColumns::Table)
        .cond_where(Expr::col(MethodsColumns::Id).eq(id))
        .to_string(MysqlQueryBuilder);
    
    println!("Running query: {query:#?}");
    let result = conn.exec_drop(query, ());
    println!("Insert result: {result:#?}");
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
        .conditions(
            params.where_id,
            |q| {
                q.and_where(Expr::col(MethodsColumns::Id).eq(params.id));
            },
            |_q| {},
        )
        .conditions(
            params.where_keyword,
            |q| {
                q.and_where(
                    Expr::col(MethodsColumns::Name).like(&params.keyword).or(
                        Expr::col(MethodsColumns::Description).like(&params.keyword)
                    )
                );
            },
            |_q| {},
        )
        .to_string(MysqlQueryBuilder);
    
    // println!("Params dict: {:#?}", params);
    println!("Query: {}", query);
    
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

async fn handle_post_row(req : &mut Request<Body>) -> Result<Response<Body>, Infallible> {
    let arguments = get_params(req);
    println!("POST /api/row params: {arguments:#?}");
    let name = arguments.get("field-name").unwrap().get(0).unwrap().to_string();
    let description = arguments.get("field-description").unwrap().get(0).unwrap().to_string();
    let image = arguments.get("field-image").unwrap().get(0).unwrap().to_string();
    
    let method = MethodEntry { id: 0, name: name, description: Some(description), image: Some(image)};
    
    insert(method);
    
    Ok(Response::new("Howdy!".into()))
}

async fn handle_delete_row(req : &mut Request<Body>) -> Result<Response<Body>, Infallible> {
    let arguments = get_params(req);
    println!("DELETE /api/row params: {arguments:#?}");
    let id = arguments.get("field-delete-id").unwrap().get(0).unwrap().parse::<i32>().unwrap();
    delete(id);
    
    Ok(Response::new("Howdy!".into()))
}

// Got request to handle_post_row Request {
//     method: POST,
//     uri: /api/row,
//     version: HTTP/1.0,
//     headers: {
//         "host": "127.0.0.1:12181",
//         "connection": "close",
//         "content-length": "419",
//         "user-agent": "Mozilla/5.0 (X11; Linux x86_64; rv:109.0) Gecko/20100101 Firefox/118.0",
//         "accept": "*/*",
//         "accept-language": "en-US,en;q=0.5",
//         "accept-encoding": "gzip, deflate, br",
//         "referer": "http://localhost/?",
//         "content-type": "multipart/form-data; boundary=---------------------------131870820826134894091747844280",
//         "origin": "http://localhost",
//         "cookie": "G_ENABLED_IDPS=google",
//         "sec-fetch-dest": "empty",
//         "sec-fetch-mode": "cors",
//         "sec-fetch-site": "same-origin",
//         "pragma": "no-cache",
//         "cache-control": "no-cache",
//     },
//     body: Body(
//         Streaming,
//     ),
// }

async fn handle_request(mut req : Request<Body>) -> Result<Response<Body>, Infallible> {
    
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/api/rows") => {
            return handle_rows(&req);
        },
        (&Method::POST, "/api/row") => {
            return handle_post_row(&mut req).await;
        },
        (&Method::DELETE, "/api/row") => {
            return handle_delete_row(&mut req).await;
        },
        (&Method::PUT, "/api/row") => {
            return handle_put_row(&mut req).await;
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
