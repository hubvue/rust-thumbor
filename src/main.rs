use axum::{extract::Path, routing::get, http::StatusCode, Router};
use percent_encoding::percent_decode_str;
use serde::Deserialize;
use std::convert::TryInto;

// 引入protobuf 生成的代码
mod pb;

use pb::*;

// 参数使用serde 做DEserialize， axum会自动识别并解析
#[derive(Deserialize)]
struct Params {
    spec: String,
    url: String,
}

#[tokio::main]
async fn main() {
    // 初始化tracing
    tracing_subscriber::fmt::init();

    // 构建路由
    let app = Router::new()
        .route("/image/:spec/:url", get(generate));

    // 运行web服务器
    let addr = "127.0.0.1:3000".parse().unwrap();
    tracing::debug!("listening on {}", addr);

    println!("server is listening {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    
    
    
}

// 目前我们就只把参数解析出来
async fn generate(Path( Params { spec, url}): Path<Params>) -> Result<String, StatusCode> {
    println!("spec: {}, url: {}", spec, url);
    let url = percent_decode_str(&url).decode_utf8_lossy();
    println!("url: {}", url);
    let spec: ImageSpec = spec
        .as_str()
        .try_into()
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    
    Ok(format!("url: {}\nspec: {:#?}", url, spec))
}
