# GitHubPushNotificationLineBotRust
A line bot which can send Github push webhook message to line.  
[[繁體中文（建立中）](#)]
## Run Directly
1. Clone the project
2. Please prepare a .env file and put it in root of project directory
```env
ACCESSTOKEN=<LINE_BOT_ACCESS_TOKEN>
```
3. Open a terminal, cd into project directory, and run the project with command below
```sh
cargo run
```
## Docker Image Support Architecture
* `linux/arm64`
* `linux/amd64`
## Run in Docker Container
```sh
docker run --name notificationlinebot -p 3000:3000 -e ACCESSTOKEN=<LINE_BOT_ACCESS_TOKEN> -d ghcr.io/kayxue/githubpushnotificationlinebotrust:latest
```
## Contribution
Contributions are welcome, and please follow [Code of conduct](https://www.rust-lang.org/policies/code-of-conduct)
