use std::{fs::OpenOptions,os::fd::AsRawFd};
use anyhow::anyhow;
use coordinator::pool::automatic::Submitter;
use nix::sys::sendfile;
use tokio::net::TcpStream;
use tracing::error;
use crate::network::{DirectStreamWriter, UnsafeFD};
use crate::network::server::ThreadPoolResult;

pub(super) struct Download<'a> {
    filepath:String,
    submitter:Submitter<ThreadPoolResult>,
    write_stream:DirectStreamWriter<'a>
}

impl <'a> Download<'a> {
    pub(super) fn new(
        filepath:String,
        submitter: Submitter<ThreadPoolResult>,
        write_stream:&'a TcpStream
    ) -> Self {
        Self{
            filepath,
            submitter,
            write_stream:DirectStreamWriter{
                stream:write_stream
            }
        }
    }

    pub(super) async fn run(&mut self) -> anyhow::Result<()> {
        let file = OpenOptions::new().read(true).open(self.filepath.as_str())?;
        let size:usize = file.metadata()?.len() as usize;
        let file_fd = UnsafeFD{
            fd:file.as_raw_fd(),
        };
        let mut idx = 0;
        while idx < size {
            let socket_fd = self.write_stream.await?;
            match self.submitter.submit(move || {
                #[cfg(target_os = "macos")]
                loop {
                    let (res,n) = sendfile::sendfile(
                        file_fd,
                        socket_fd,
                        idx as i64,
                        None,
                        None,
                        None
                    );
                    return if let Err(e) = res {
                        if e == nix::errno::Errno::EAGAIN {
                            if n == 0 {
                                ThreadPoolResult::Usize(0)
                            } else {
                                ThreadPoolResult::Usize(n as usize)
                            }
                        } else {
                            error!("failed to sendfile: {}", e);
                            ThreadPoolResult::Err(anyhow!("failed to sendfile: {}", e))
                        }
                    } else {
                        ThreadPoolResult::Usize(n as usize)
                    }
                }
            }).await{
                ThreadPoolResult::Usize(n) => {
                    idx += n;
                }
                ThreadPoolResult::Err(e) => {
                    return Err(e)
                }
                ThreadPoolResult::None => {
                    break
                }
            }
        }
        Ok(())
    }
}