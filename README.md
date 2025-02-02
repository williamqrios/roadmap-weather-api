## Weather API
Project idea from [roadmap.sh](https://roadmap.sh/projects/weather-api-wrapper-service). This project consists of a simple wrapper around the Visual Crossing Weather API. Built with Rust 1.84.0. 

### Setting up the project locally
#### Install and start Redis Stack 
See [this tutorial](https://redis.io/docs/latest/operate/oss_and_stack/install/install-stack/) on how to install Redis Stack. 

On Linux, you can start Redis as a background process with: 
```
sudo systemctl start redis-stack-server
```
To stop, use:
```
sudo systemctl stop redis-stack-server
```
Verify the address where Redis is running by invoking the `redis-cli` command:
```
redis-cli
```
To use Redis in Rust, this project depends on the [redis-rs](https://github.com/redis-rs/redis-rs) crate. 

#### Getting an API key 
Sign up for a free API key on [Visual Crossing](https://www.visualcrossing.com/weather-api). 

#### Configuring `.env`
Create a `.env` file in the project directory. This will contain environment variables that are read when the program starts up. The file should contain the following entries:
```
API_KEY='X123456789'
REDIS_HOST='127.0.0.1'
``` 
where `API_KEY` is the Visual Crossing Weather API key; replace the placeholder value with your key. Replace `REDIS_HOST` if you are not running Redis locally. 

### Example 
Start the API by running: 
```
cargo run 
# Output 
# Listening for requests on 0.0.0.0:3000
```
There is only a single route, which takes in a location and optionally two query parameters, a start date and an end date, corresponding to `date1` and `date2` in the [Timeline Weather API](https://www.visualcrossing.com/resources/documentation/weather-api/timeline-weather-api/). The date should be in the format `yyyy-mm-dd`.
```
localhost:3000/{location}?start_date={date1}&end_date={date2}
```
You can test the response from a browser.  
- Location with no dates - returns a 15 day weather forecast. 
```
http://localhost:3000/London,UK 
```
- Location with a start date - returns the weather for the specified date. 
```
http://localhost:3000/Ithaca,NY?start_date=2025-01-01
```
- Location with start and end date - returns weather data between the specified dates. 
```
http://localhost:3000/Ithaca,NY?start_date=2025-01-01&end_date=2025-01-04 
```
