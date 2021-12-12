use std::collections::HashMap;
use std::io;
use crate::config;
use crate::connect;

// Stdin input error auxiliar
fn bad_input(e: Option<io::Error>) -> Result<String,io::Error> {
        let msg = match e {
            Some(e) => format!(": {}",e),
            None => "".to_string(),
        };
        let input_error = io::Error::new(io::ErrorKind::InvalidInput, format!("{}{}",config::INVALID_INPUT, msg));
        return Err(input_error);
}

// Forecast days functions
fn forecast_days_handler() -> Result<String, io::Error> {
    let mut forecast_input = String::new();
    println!("Buscamos el pronóstico a 1, 2 o 3 días?");
    if let Err(e) = io::stdin().read_line(&mut forecast_input) {
        return Err(e);
    }
    if !config::FORECAST_OPTIONS.contains(&&forecast_input.trim()) {
        return bad_input(None);
    }
    println!("Buscando el pronóstico a {} día/s", forecast_input.trim());
    Ok(forecast_input)
}

pub fn forecast_retry() -> String {
    return match forecast_days_handler(){
        Ok(f) => f,
        Err(e) => {
            println!("{}: {}", config::RETRY_ERROR, e);
            forecast_retry()   
        }
    };

}


// weather of cities functions
pub fn weather_handler(api_response: Vec<connect::ApiResponse>) {
    // create a hashmap of provinces with the indexes of its cities
    let mut province_cities_index_map: HashMap<String, Vec<usize>> = HashMap::new();
    api_response.iter()
        .enumerate()
        .for_each(|(i,forecast_info)|{
            if let Some(province) = province_cities_index_map.get_mut(&forecast_info.province) {
                province.push(i);
            } else {
                province_cities_index_map.insert(forecast_info.province.clone(), vec!(i));
            }
        });
    // promt the provinces and get the index of the province selected by the user
    let province_name = province_retry(&province_cities_index_map);
    // get the indexes of the cities of the selected province
    let province_city_indexes = province_cities_index_map.get(&province_name).unwrap();
    // promt the cities and get the index of one of the city selected by the user
    let city_index = city_retry(&api_response, province_city_indexes);
    // get the city info
    let city = api_response.get(city_index).unwrap();
    println!(
        "Ciudad Seleccionada: {} \n\n Mañana: \n Temperatura: {}\n Pronóstico: {}\n\n Tarde: \n Temperatura: {}\n Pronóstico: {}",
        city.name,
        city.weather.morning_temp,
        city.weather.morning_desc,
        city.weather.afternoon_temp,
        city.weather.afternoon_desc);
}

fn province_retry(province_cities_index_map: &HashMap<String, Vec<usize>>) -> String {

    return match province_selector(province_cities_index_map){
        Ok(province_name) => province_name,
        Err(e) => {
            println!("{}: {}", config::RETRY_ERROR, e);
            province_retry(province_cities_index_map)   
        }
    };
}

fn province_selector(province_cities_index_map: &HashMap<String, Vec<usize>>) -> Result<String, io::Error> {
    println!("Seleccione una provincia (ingrese número)");
    province_cities_index_map.into_iter()
        .enumerate()
        .for_each(|(i,(province_name,_))|{
            println!("[{}]: {}", i, province_name);
        });

    let mut api_response_province_index = String::new();
    if let Err(e) = io::stdin().read_line(&mut api_response_province_index) {
        return Err(e);
    }
    let parsed_province_index: usize = match api_response_province_index.trim().parse() {
        Ok(province_index) => province_index,
        Err (_) => {
            return bad_input(Some(io::Error::new(io::ErrorKind::InvalidInput, "No se puede convertir el input a número")));
        }
    };
    let province_name =  match province_cities_index_map.keys().nth(parsed_province_index){
        Some(province) => province,
        None => {
            return bad_input(None)
        }
    };
    println!("Provincia seleccionada: {}", province_name);
    Ok(province_name.to_string())

}

fn city_retry(api_response: &Vec<connect::ApiResponse>, province_cities: &Vec<usize>) -> usize {

    return match city_selector(api_response, province_cities){
        Ok(f) => f,
        Err(e) => {
            println!("{}: {}", config::RETRY_ERROR, e);
            city_retry(api_response, province_cities)   
        }
    };
}

fn city_selector(api_response: &Vec<connect::ApiResponse>, province_cities: &Vec<usize>) -> Result<usize, io::Error> {
    println!("Seleccione una ciudad (ingrese número)");
    province_cities.into_iter()
        .enumerate()
        .for_each(|(i, city_index)|{
            let city = api_response.get(*city_index).unwrap();
            println!("[{}]:{}",i,city.name);
        });
    let mut api_response_city_index = String::new();
    if let Err(e) = io::stdin().read_line(&mut api_response_city_index) {
        return Err(e);
    }
    let parsed_city_index: usize = match api_response_city_index.trim().parse() {
        Ok(u) => u,
        Err (_) => {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, format!("{}",config::PARSE_ERROR)));
        }
    };
    let city = match province_cities.get(parsed_city_index) {
        Some(c) => *c,
        None => {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, config::INVALID_INPUT));
        }
    };
    Ok(city)
}
