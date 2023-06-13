#[cfg(any(
    feature = "rt-async-std",
    feature = "rt-tokio",
    feature = "rt-tokio-current-thread"
))]
use crate::exporter::addrs_and_family;
use async_trait::async_trait;
use ts_opentelemetry::{runtime::RuntimeChannel, sdk::trace::BatchMessage};
use std::net::ToSocketAddrs;

/// Jaeger Trace Runtime is an extension to [`RuntimeChannel`].
///
/// [`RuntimeChannel`]: ts_opentelemetry::sdk::runtime::RuntimeChannel
#[async_trait]
pub trait JaegerTraceRuntime: RuntimeChannel<BatchMessage> + std::fmt::Debug {
    /// A communication socket between Jaeger client and agent.
    type Socket: std::fmt::Debug + Send + Sync;

    /// Create a new communication socket.
    fn create_socket<T: ToSocketAddrs>(&self, endpoint: T) -> thrift::Result<Self::Socket>;

    /// Send payload over the socket.
    async fn write_to_socket(&self, socket: &Self::Socket, payload: Vec<u8>) -> thrift::Result<()>;
}

#[cfg(feature = "rt-tokio")]
#[async_trait]
impl JaegerTraceRuntime for ts_opentelemetry::runtime::Tokio {
    type Socket = tokio::net::UdpSocket;

    fn create_socket<T: ToSocketAddrs>(&self, endpoint: T) -> thrift::Result<Self::Socket> {
        let (addrs, family) = addrs_and_family(&endpoint)?;
        let conn = std::net::UdpSocket::bind(family)?;
        conn.connect(addrs.as_slice())?;
        Ok(tokio::net::UdpSocket::from_std(conn)?)
    }

    async fn write_to_socket(&self, socket: &Self::Socket, payload: Vec<u8>) -> thrift::Result<()> {
        socket.send(&payload).await?;

        Ok(())
    }
}

#[cfg(feature = "rt-tokio-current-thread")]
#[async_trait]
impl JaegerTraceRuntime for ts_opentelemetry::runtime::TokioCurrentThread {
    type Socket = tokio::net::UdpSocket;

    fn create_socket<T: ToSocketAddrs>(&self, endpoint: T) -> thrift::Result<Self::Socket> {
        let (addrs, family) = addrs_and_family(&endpoint)?;
        let conn = std::net::UdpSocket::bind(family)?;
        conn.connect(addrs.as_slice())?;
        Ok(tokio::net::UdpSocket::from_std(conn)?)
    }

    async fn write_to_socket(&self, socket: &Self::Socket, payload: Vec<u8>) -> thrift::Result<()> {
        socket.send(&payload).await?;

        Ok(())
    }
}

#[cfg(feature = "rt-async-std")]
#[async_trait]
impl JaegerTraceRuntime for ts_opentelemetry::runtime::AsyncStd {
    type Socket = async_std::net::UdpSocket;

    fn create_socket<T: ToSocketAddrs>(&self, endpoint: T) -> thrift::Result<Self::Socket> {
        let (addrs, family) = addrs_and_family(&endpoint)?;
        let conn = std::net::UdpSocket::bind(family)?;
        conn.connect(addrs.as_slice())?;
        Ok(async_std::net::UdpSocket::from(conn))
    }

    async fn write_to_socket(&self, socket: &Self::Socket, payload: Vec<u8>) -> thrift::Result<()> {
        socket.send(&payload).await?;

        Ok(())
    }
}
