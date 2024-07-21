use std::{collections::HashMap, io, net::SocketAddr, sync::Arc};

use crate::{
    tcp::{parse_tcp_config, TcpConfig, TcpConnector},
    tls::{parse_tls_config, TlsConfig, TlsConnector},
};
use anyhow::bail;
use async_trait::async_trait;
use clap::{arg, value_parser, ArgMatches, Command};
use http_body_util::Full;
use hyper::{
    body::{Body, Bytes},
    client::conn::{
        http1::{Builder, SendRequest},
        http2::handshake,
    },
};
use hyper_util::rt::TokioIo;
use rk::{basic_command, parse_http_header, ProtocolConnector, Stream, IO, NAME};
use tokio::net::TcpStream;
use url::Url;

pub fn root_subcommand() -> Command {
    return basic_command(NAME)
        .subcommand_negates_reqs(true)
        .subcommand_required(false)
        .about(
            "By default we benchmark url using HTTP/1.1(works like wrk).\nOr you can use Commands to benchmark other protocol.",
        )
        .arg(
            arg!(
                -c --connections <connections>  "Connections to keep open"
            ).required(true)
            .value_parser(value_parser!(usize)),
        )
        .arg(
            arg!(
                -t --threads <thread>  "Number of threads to use"
            ).required(true)
            .value_parser(value_parser!(usize)),
        )
        .arg(
            arg!(
                -d --duration <duration> "Duration of test"
            )
            .value_parser(value_parser!(String)),
        )
        .arg(
            arg!(
                headers: -H --header <headers> "Add header to request"
            )
            .value_parser(value_parser!(String)),
        )
        .arg(
            arg!(
                --latency "Print latancy statistics"
            )
            .value_parser(value_parser!(bool)),
        )
        .arg(
            arg!(
                --timeout <timeout> "Socket/request timeout"
            )
            .value_parser(value_parser!(String)),
        );
}
#[derive(Clone)]
pub struct H1Config {
    pub url: Url,
    pub headers: HashMap<String, String>,
    pub tls_config: TlsConfig,
    pub tcp_config: TcpConfig,
}
struct Http1Connector<T> {
    config: H1Config,
    inner: T,
    builder: Builder,
}

impl<T> Http1Connector<T> {}

impl<T> Http1Connector<T> {
    pub fn new(inner: T, c: &H1Config) -> Self {
        let mut b = hyper::client::conn::http1::Builder::new();
        Self {
            config: c.clone(),
            inner,
            builder: b,
        }
    }
}

#[async_trait]
impl<T> ProtocolConnector for Http1Connector<T>
where
    T: ProtocolConnector<Connection = Stream>,
{
    type Connection = Http1Connection;
    async fn connect(&self) -> anyhow::Result<Self::Connection> {
        let s = self.inner.connect().await?;
        let stream = TokioIo::new(s);
        let (sender, conn) = self.builder.handshake(stream).await?;
        tokio::spawn(async move {
            if let Err(err) = conn.await {
                log::debug!("err {err:?}");
            }
        });
        let stream = Http1Connection {
            url: self.config.url.clone(),
            sender,
        };
        Ok(stream)
    }
}

fn parse_h1_config(m: &ArgMatches) -> anyhow::Result<H1Config> {
    let tls_config = parse_tls_config(&m)?;
    let tcp_config = parse_tcp_config(&m)?;
    let target: &Url = m.get_one("url").unwrap();
    let headers = parse_http_header(m);
    let c = H1Config {
        url: target.clone(),
        tcp_config,
        tls_config,
        headers,
    };
    Ok(c)
}

pub fn do_h1(m: &ArgMatches) -> anyhow::Result<()> {
    let c = parse_h1_config(m)?;
    let connector: Arc<dyn ProtocolConnector<Connection = Http1Connection>> = match c.url.scheme() {
        "http" => {
            let tcp_connector = TcpConnector::new(&c.tcp_config);
            let h1 = Http1Connector::new(tcp_connector, &c);
            Arc::new(h1)
        }
        "https" => {
            let tcp = TcpConnector::new(&c.tcp_config);
            let tls = TlsConnector::new(tcp, &c.tls_config);
            let h1 = Http1Connector::new(tls, &c);
            Arc::new(h1)
        }
        scheme => bail!("unknown scheme for http1 connector {scheme}"),
    };
    Ok(())
}

struct Http1Stream {}
pub enum HttpProtocol {
    H1,
    H2,
    H3,
}
struct Http1Connection {
    url: Url,
    sender: SendRequest<Full<Bytes>>,
}

impl Http1Connection {
    pub fn new(url: Url, sender: SendRequest<Full<Bytes>>) -> Self {
        Self { url, sender }
    }
    pub async fn send_req(&mut self) -> anyhow::Result<()> {
        todo!()
        // b.handshake(&mut self.stream)
        // handshake(exec, io)
    }
}
