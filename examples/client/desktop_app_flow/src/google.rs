/*
RUST_BACKTRACE=1 RUST_LOG=debug cargo run -p oauth2_client_desktop_app_flow_example --bin desktop_app_flow_google -- 'YOUR_CLIENT_ID' 'YOUR_CLIENT_SECRET'
*/

use std::{env, error, io, thread};

use http_api_isahc_client::IsahcClient;
use oauth2_client::{authorization_code_grant::Flow, oauth2_core::types::RedirectUri};
use oauth2_google::{GoogleProviderForDesktopApps, GoogleScope};
use wry::{
    application::{
        event::{Event, StartCause, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        platform::unix::EventLoopExtUnix as _,
        window::WindowBuilder,
    },
    webview::WebViewBuilder,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    pretty_env_logger::init();

    let client_id = env::args().nth(1).unwrap();
    let client_secret = env::args().nth(2).unwrap();

    run(client_id, client_secret).await
}

async fn run(client_id: String, client_secret: String) -> Result<(), Box<dyn error::Error>> {
    let scopes = vec![GoogleScope::Email];

    let flow = Flow::new(IsahcClient::new()?);
    let provider = GoogleProviderForDesktopApps::new(client_id, client_secret, RedirectUri::Oob)?;

    let authorization_url = flow.build_authorization_url(&provider, scopes, None)?;

    println!("authorization_url: {:?}", authorization_url.as_str());

    //
    //
    //
    thread::spawn(move || {
        let event_loop: EventLoop<()> = EventLoop::new_any_thread();
        let window = WindowBuilder::new()
            .with_title("OAuth2")
            .build(&event_loop)
            .unwrap();
        let _webview = WebViewBuilder::new(window)
            .unwrap()
            .with_url(authorization_url.as_str())
            .unwrap()
            .build()
            .unwrap();

        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;

            match event {
                Event::NewEvents(StartCause::Init) => {}
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => *control_flow = ControlFlow::Exit,
                _ => (),
            }
        });
    });

    //
    //
    //
    println!("Enter code: ");
    let mut code = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut code)?;
    code = code.trim_end().to_owned();
    println!("code: {:?}", code);

    let access_token_body = flow.handle_callback(&provider, code, None).await?;

    println!("access_token_body: {:?}", access_token_body);

    Ok(())
}
