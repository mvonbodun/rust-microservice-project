use std::env;

use authentication::auth_client::AuthClient;
use authentication::{SignInRequest, SignOutRequest, SignUpRequest};
use tokio::time::{sleep, Duration};
use tonic::{Request, Response};
use uuid::Uuid;

use crate::authentication::{SignInResponse, SignOutResponse, SignUpResponse, StatusCode};

pub mod authentication {
    tonic::include_proto!("authentication");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // AUTH_SERVICE_HOST_NAME will be set to 'auth' when running the health check service in Docker
    // ::0 is required for Docker to work: https://stackoverflow.com/questions/59179831/docker-app-server-ip-address-127-0-0-1-difference-of-0-0-0-0-ip
    let auth_hostname = env::var("AUTH_SERVICE_HOST_NAME").unwrap_or("[::0]".to_owned());

    // Establish connection when auth service
    let mut client = AuthClient::connect(format!("http://{}:50051", auth_hostname)).await?;

    loop {
        // Create random username using new_v4()
        let username: String = Uuid::new_v4().to_string();
        // Create random password using new_v4()
        let password: String = Uuid::new_v4().to_string();
        // Create a new `SignUpRequest`.
        let request: Request<SignUpRequest> = Request::new(SignUpRequest {
            username: username.clone(),
            password: password.clone(),
        });
        // Make a sign up request. Propagate any errors.
        let response: Response<SignUpResponse> = client.sign_up(request).await?;

        // Log the response
        println!(
            "SIGN UP RESPONSE STATUS: {:?}",
            StatusCode::try_from(response.into_inner().status_code)
        );

        // ---------------------------------------------
        // Create a new `SignInRequest`.
        let request: Request<SignInRequest> = Request::new(SignInRequest {
            username: username.clone(),
            password: password.clone(),
        });

        // Make a sign in request. Propagate any errors. Convert Response<SignInResponse> into SignInResponse.
        println!("START SignInRequest");
        let response: SignInResponse = client.sign_in(request).await.unwrap().into_inner();
        println!("END SignInRequest");

        println!(
            "SIGN IN RESPONSE STATUS: {:?}",
            response.status_code // Log response status_code
        );

        // ---------------------------------------------
        // Create a new `SignOutRequest`.
        let request: Request<SignOutRequest> = Request::new(SignOutRequest {
            session_token: response.session_token.clone(), // Use session_token from sign in response
        });
        // Make a sign out request. Propagate any errors.
        let response: Response<SignOutResponse> = client.sign_out(request).await?;

        println!(
            "SIGN OUT RESPONSE STATUS: {:?}",
            response // Log response status_code
        );

        println!("--------------------------------------",);

        sleep(Duration::from_secs(3)).await;
    }
}
