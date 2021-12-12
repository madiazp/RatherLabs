extern crate tokio;

mod connect;
mod config;
mod handlers;

#[tokio::main]
pub async fn main() {
    let forecast = handlers::forecast_retry();
    let response = match connect::api_call(forecast).await {
        Ok(r) => r,
        Err(e) => {
            println!("{}",e);
            return;
       }
    };
    handlers::weather_handler(response);
}
