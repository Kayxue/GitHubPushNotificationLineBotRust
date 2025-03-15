# GitHubPushNotificationLineBotRust
A line bot which can send Github push webhook message to line.  
[[繁體中文](https://blog.kayxue.xyz/posts/githubpushnotificationlinebotrust/)]
> [!CAUTION]
> If you want to test the version on this branch, make sure you have updated your rust toolchain to nightly build and be careful for possible rust bug.
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
## Run the Docker Image
```sh
docker run --name notificationlinebot -p 3000:3000 -e ACCESSTOKEN=<LINE_BOT_ACCESS_TOKEN> -d ghcr.io/kayxue/githubpushwebhooklinebotrustnightly:xitcanightly
```
## Contribution
Contributions are welcome, and please follow [Code of conduct](https://www.rust-lang.org/policies/code-of-conduct)
