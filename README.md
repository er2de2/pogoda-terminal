Polska lokalizacja weather-cli. Podziękowania dla autora projektu.

# pogoda-terminal
Use simple commands to check the weather in your city. Easily search your city and select it.


## Setup
You can install the crate with `cargo install` command.

cargo install pogoda-terminal

Once installed, create an API key on [OpenWeather](https://openweathermap.org). You can register your key using the `pogoda-terminal setup-api` command.

pogoda-terminal setup-api --key "EXAMPLE_KEY"


## Commands

| command      | description                            |
| ------------ | -------------------------------------- |
| check        | Check weather information in your city |
| set-location | Search and set your city               |
| setup-api    | Setup the OpenWeather API Key          |
| about        | View information about the program     |
| help         | View the list of commands              |


## Przykład

1. City Search

pogoda-terminal set-location --query Dobra,PL

City list:
1) Dobra, PL (lat: 51.9160472, lon: 18.6151806)
2) Dobra, PL (lat: 51.23583, lon: 17.30418)
3) Dobra, PL (lat: 53.584659, lon: 15.3073048)
4) Dobra, PL (lat: 49.7181214, lon: 20.2507691)
5) Dobra, PL (lat: 53.4872318, lon: 14.385464)

Please select your city.
3

Do you use Celsius or Fahrenheit?
1) Celsius
2) Fahrenheit
1

Do you want to display emoji? (y/n)
y

Dobra is now your city!
I'll use metric for you.

2. Weather Check

$ pogoda-terminal check                

Dobra (PL)
22.4° / ☁️ Clouds (scattered clouds)
Max: 23.03°, Min: 20.84°

- Wiatr: 3.18 m/s,
- Wilgotność: 56 %,
- Ciśnienie: 1022 hPa
- Zachód: 18:49
  (Wschód: 06:50)
