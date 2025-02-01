### Weather API
Project inspiration taken from [roadmap.sh](https://roadmap.sh/projects/weather-api-wrapper-service). Built with Rust 1.84.0. 

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

#### Configuring `.env`
Create a `.env` file in the project directory. This will contain environment variables that are read when the program starts up. The file should contain the following entries:
```
API_KEY='X123456789'
REDIS_HOST='127.0.0.1'
``` 
where `API_KEY` is the Visual Crossing Weather API key; replace the placeholder value with your key.
