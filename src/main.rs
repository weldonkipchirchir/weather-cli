mod weather_info;
mod weather_data;

use colored::*;
use std::io;
use weather_info::display_weather_info;
use weather_data::get_weather_data;
use dotenv::dotenv;

fn main() {
    dotenv().ok(); // Load environment variables from `.env` file
    println!("{}", "Welcome to the Weather Station!".bright_yellow());
    loop {
        println!("{}", "Please enter the name of the city:".bright_green());
        let mut city = String::new();
        io::stdin()
            .read_line(&mut city)
            .expect("failed to read city");
        let city = city.trim();

        println!("{}", "Please enter the name of the country:".bright_green());
        let mut country = String::new();
        io::stdin()
            .read_line(&mut country)
            .expect("failed to read country");
        let country = country.trim();

        let api_key = std::env::var("API_KEY").expect("API_KEY not found");

        //calling fetch weather
        match get_weather_data(&city, &country, &api_key) {
            Ok(response) => {
                display_weather_info(&response);
            }
            Err(err) => {
                eprint!("Err : {}", err)
            }
        }

        println!(
            "{}",
            "Do you want to search for weather in another city? (yes/no):".bright_green()
        );
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read input");
        let input = input.trim().to_lowercase();

        if input != "yes" {
            println!("{}", "Thank you for using our program!".bright_green());
            break;
        }
    }
}
