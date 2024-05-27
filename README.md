# Weather CLI

A command-line interface (CLI) application written in Rust that fetches and displays weather information for a given city and country.

## Features

- Get current weather details for any city and country
- Display temperature, humidity, pressure, wind speed, and weather description
- Colorized output based on weather conditions
- Interactive CLI with prompts for city and country input
- Support for repeated weather searches

## Prerequisites

- Rust (latest stable version)
- An API key from [OpenWeatherMap](https://openweathermap.org/api)

## Installation

1. Clone the repository: git clone https://github.com/weldonkipchirchir/weather-cli.git

2. Navigate to the project directory: cd weather-cli

3. Create a `.env` file in the project root directory and add your OpenWeatherMap API key: API_KEY=your_api_key_here

4. Build and run the project: cargo run

## Usage

1. When the application starts, you'll be greeted with a welcome message.
2. Enter the name of the city when prompted.
3. Enter the code of the country when prompted.
4. The application will fetch and display the current weather information for the specified city and country.
5. You'll be asked if you want to search for weather in another city. Enter 'yes' or 'no'.
6. If you enter 'yes', the application will prompt you for a new city and country.
7. If you enter 'no', the application will exit.

## Dependencies

- [reqwest](https://crates.io/crates/reqwest) - For making HTTP requests
- [serde](https://crates.io/crates/serde) - For deserializing JSON responses
- [colored](https://crates.io/crates/colored) - For colorizing terminal output
- [dotenv](https://crates.io/crates/dotenv) - For loading environment variables from a `.env` file

## Contributing

Contributions are welcome! If you find any issues or have suggestions for improvements, please open an issue or submit a pull request.

## License

This project is licensed under the [MIT License](LICENSE).