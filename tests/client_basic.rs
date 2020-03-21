use franz::Client;

#[tokio::test]
async fn client_simple() {
    let mut client = Client::connect("127.0.0.1:9092").await.unwrap();

    client.send_api_version_request().await.unwrap();
    client.wip_recv().await.unwrap();
}
