use calendar::greeter_client::GreeterClient;
use calendar::HelloRequest;

pub mod calendar {
    tonic::include_proto!("calendar");
}

pub async fn ping() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GreeterClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(HelloRequest {
        name: "Oliver".into(),
    });

    let response = client.say_hello(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}

