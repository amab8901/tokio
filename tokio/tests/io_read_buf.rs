#![warn(rust_2018_idioms)]
#![cfg(feature = "full")]

use tokio::io::{AsyncRead, AsyncReadExt, ReadBuf};
use tokio_test::assert_ok;

use std::io;
use std::pin::Pin;
use std::task::{Context, Poll};

#[tokio::test]
async fn read_buf() {
    struct Rd {
        cnt: usize,
    }

    impl AsyncRead for Rd {
        fn poll_read(
            mut self: Pin<&mut Self>,
            _cx: &mut Context<'_>,
            buf: &mut ReadBuf<'_>,
        ) -> Poll<io::Result<()>> {
            self.cnt += 1;
            buf.put_slice(b"hello world");
            Poll::Ready(Ok(()))
        }
    }

    let mut buf = vec![];
    let mut rd = Rd { cnt: 0 };

    let n = assert_ok!(rd.read_buf(&mut buf).await);
    assert_eq!(1, rd.cnt);
    assert_eq!(n, 11);
    assert_eq!(buf[..], b"hello world"[..]);
}

#[tokio::test]
#[cfg(feature = "io-util")]
#[cfg_attr(docsrs, doc(cfg(feature = "io-util")))]
async fn issue_5588() {
    use bytes::BufMut;

    // steps to zero
    let mut buf = [0; 8];
    let mut read_buf = ReadBuf::new(&mut buf);
    assert_eq!(read_buf.remaining_mut(), 8);
    assert_eq!(read_buf.remaining_mut(), 8);
    unsafe {
        read_buf.advance_mut(1);
    }
    assert_eq!(read_buf.remaining_mut(), 7);
    assert_eq!(read_buf.remaining_mut(), 7);
    unsafe {
        read_buf.advance_mut(5);
    }
    assert_eq!(read_buf.remaining_mut(), 2);
    assert_eq!(read_buf.remaining_mut(), 2);
    unsafe {
        read_buf.advance_mut(2);
    }
    assert_eq!(read_buf.remaining_mut(), 0);
    assert_eq!(read_buf.remaining_mut(), 0);

    // directly to zero
    let mut buf = [0; 8];
    let mut read_buf = ReadBuf::new(&mut buf);
    assert_eq!(read_buf.remaining_mut(), 8);
    assert_eq!(read_buf.chunk_mut().len(), 8);
    unsafe {
        read_buf.advance_mut(8);
    }
    assert_eq!(read_buf.remaining_mut(), 0);
    assert_eq!(read_buf.chunk_mut().len(), 0);
}