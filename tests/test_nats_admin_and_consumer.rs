use crate::common::init_cfg::get_test_cfg;
use crate::common::nats_container::get_nats_container;
use async_channel::bounded;
use bytes::Bytes;
use ductaper::mq::connector::Connector;
use ductaper::mq::publisher::Publisher;
use ductaper::mq::redact_consumer::RedactConsumer;
use ductaper::mq::stream_admin::StreamAdmin;
use ductaper::util::logging::init_tracing;
use testcontainers::core::IntoContainerPort;

mod common;

#[tokio::test]
pub async fn test_nats_admin_and_consumer() {
    let container = get_nats_container().await;

    if let Ok(port) = container.get_host_port_ipv4(4222.tcp()).await {
        init_tracing();
        let cfg = get_test_cfg(port);
        let conn = Connector::new(&cfg, None).await;
        let admin = StreamAdmin::new(&conn);
        let subj = StreamAdmin::get_full_subject(&cfg, cfg.redact_subject.clone().as_str());
        for _ in 0..2 {
            // first it should create then just check
            if let Err(e) = admin
                .check_stream(
                    cfg.queue_stream.clone(), //
                    vec![subj.clone()],
                )
                .await
            {
                panic!("Cannot check stream: {e}");
            }
        }
        for _ in 0..2 {
            // first it should create then just check
            if let Err(e) = admin
                .check_stream(
                    cfg.queue_stream.clone(), //
                    vec!["asdf_subject2".to_string()],
                )
                .await
            {
                panic!("Cannot check stream: {e}");
            }
        }
        let (msg_sender, msg_receiver) = bounded(1);

        let consumer = RedactConsumer::new(&conn, msg_sender);

        let mut count = 0;
        match consumer.subscribe(&cfg).await {
            Ok(_) => {
                let stop = consumer.start(&cfg).await;

                //tokio::time::sleep(Duration::from_micros(42)).await;
                let publisher = Publisher::new(&conn);

                publisher
                    .publish(
                        subj,
                        b"asdf".to_vec(), //
                        None,
                    )
                    .await;

                match msg_receiver.recv().await {
                    Ok(m) => {
                        assert_eq!(m.payload, Bytes::from_static(b"asdf"));
                        count += 1;
                    }
                    Err(e) => {
                        panic!("Cannot read message: {e}");
                    }
                }
                consumer.stop(stop).await;
                //stop.stop_tx.send(()).ok();
            }
            Err(e) => {
                panic!("Cannot subscribe: {e}");
            }
        }
        assert_eq!(1, count)
    }
}
