use std::{
    net::{Ipv4Addr, SocketAddrV4},
    time::Duration,
};

use runtime::{
    Runtime,
    tcp::{Listener, Read, Stream, Tcp, Write},
    time::Time,
};

pub fn listener_test_when_bind_then_returns_tcp_listener<R: Runtime>() {
    let listener = R::new(1).block_on(async move {
        let addr = SocketAddrV4::new(Ipv4Addr::LOCALHOST, 8080);

        R::Tcp::listener(addr).await
    });

    assert!(matches!(listener, Ok(_)));
}
pub fn stream_test_when_connect_then_returns_tcp_stream<R: Runtime>() {
    let stream = R::new(1).block_on(async move {
        let addr = SocketAddrV4::new(Ipv4Addr::LOCALHOST, 8081);
        R::spawn_local(async move {
            let addr = SocketAddrV4::new(Ipv4Addr::LOCALHOST, 8081);
            let listener = R::Tcp::listener(addr).await.unwrap();

            while let Ok(_) = listener.accept().await {}
        });
        R::Time::sleep(Duration::from_millis(250)).await;
        R::Tcp::stream(addr).await
    });

    assert!(matches!(stream, Ok(_)))
}

pub fn stream_test_when_write_then_read_sucess<R: Runtime>() {
    R::new(1).block_on(async move {
        let addr = SocketAddrV4::new(Ipv4Addr::LOCALHOST, 8082);
        R::spawn_local(async move {
            let addr = SocketAddrV4::new(Ipv4Addr::LOCALHOST, 8082);
            let listener = R::Tcp::listener(addr).await.unwrap();
            while let Ok((stream, _)) = listener.accept().await {
                let (mut read, mut write) = stream.split();
                write.write_all("hello world!".as_bytes()).await.unwrap();

                let mut buf = [0; 12];
                read.read(&mut buf).await.unwrap();
                assert_eq!("hello there!", String::from_utf8_lossy(&buf));
            }
        });

        R::Time::sleep(Duration::from_millis(250)).await;

        let (mut read, mut write) = R::Tcp::stream(addr).await.unwrap().split();

        let mut buf = [0; 12];
        read.read(&mut buf).await.unwrap();
        assert_eq!("hello world!", String::from_utf8_lossy(&buf));

        write.write_all("hello there!".as_bytes()).await.unwrap();
    });
}
