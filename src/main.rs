use http_body_util::{BodyExt, Empty};
use hyper::Request;
use hyper::body::Bytes;
use hyper_util::rt::TokioIo;
use rustls_pki_types::ServerName;
use std::sync::Arc;
use tokio::io::{self, AsyncWriteExt as _};
use tokio::net::TcpStream;
use tokio_rustls::TlsConnector;
use tokio_rustls::rustls::{ClientConfig, RootCertStore};

// A simple type alias so as to DRY.
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() -> Result<()> {
  let url = "https://example.com".parse::<hyper::Uri>()?;

  fetch_url(url).await
}

async fn fetch_url(url: hyper::Uri) -> Result<()> {
  let mut root_cert_store = RootCertStore::empty();
  root_cert_store.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());
  let config = ClientConfig::builder()
    .with_root_certificates(root_cert_store)
    .with_no_client_auth();

  let connector = TlsConnector::from(Arc::new(config));
  // let dnsname = ServerName::try_from("www.rust-lang.org").unwrap();

  let host = url.host().expect("uri has no host");
  let port = 443;
  let addr = format!("{host}:{port}");
  let stream = TcpStream::connect(addr).await?;

  let domain = ServerName::try_from(host)?.to_owned();
  let stream = connector.connect(domain, stream).await?;
  let io = TokioIo::new(stream);

  let (mut sender, conn) = hyper::client::conn::http1::handshake(io).await?;
  tokio::task::spawn(async move {
    if let Err(err) = conn.await {
      println!("Connection failed: {err:?}");
    }
  });

  let authority = url.authority().unwrap().clone();

  let path = url.path();
  let req = Request::builder()
    .uri(path)
    .header(hyper::header::HOST, authority.as_str())
    .body(Empty::<Bytes>::new())?;

  let mut response = sender.send_request(req).await?;

  println!("Response: {}", response.status());
  println!("Headers: {:#?}\n", response.headers());

  // Stream the body, writing each chunk to stdout as we get it
  // (instead of buffering and printing at the end).
  while let Some(next) = response.frame().await {
    let frame = next?;
    if let Some(chunk) = frame.data_ref() {
      io::stdout().write_all(chunk).await?;
    }
  }

  println!("\n\nDone!");

  Ok(())
}
