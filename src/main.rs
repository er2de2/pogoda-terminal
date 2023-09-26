use pogoda_terminal::cmd_line;

#[tokio::main]
async fn main() {
    cmd_line::init().await;
}
