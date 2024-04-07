pub(self) mod download;
pub(self) mod upload;
pub(crate) mod server;

use std::future::Future;
use std::os::fd::{AsFd, AsRawFd, BorrowedFd, RawFd};
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::net::TcpStream;

#[derive(Copy, Clone)]
pub(super) struct UnsafeFD{
    fd:RawFd,
}

impl AsFd for UnsafeFD {
    fn as_fd(&self) -> BorrowedFd<'_> {
        unsafe {
            BorrowedFd::borrow_raw(self.fd)
        }
    }
}

pub(super) struct DirectStreamWriter<'a> {
    pub(super) stream:&'a TcpStream
}

impl <'a> Future for DirectStreamWriter<'a> {
    type Output = anyhow::Result<UnsafeFD>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.stream.poll_write_ready(cx) {
            Poll::Ready(Ok(_)) => {
                let fd = self.stream.as_raw_fd();
                Poll::Ready(Ok(UnsafeFD{fd }))
            }
            Poll::Pending => {
                Poll::Pending
            }
            Poll::Ready(Err(e)) => {
                Poll::Ready(Err(e.into()))
            }
        }
    }
}