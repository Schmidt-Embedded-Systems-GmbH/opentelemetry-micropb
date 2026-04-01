#![cfg(all(feature = "use_std", feature = "use_area", feature = "use_bumpalo"))]
#![feature(test)]

extern crate test;

use micropb::{MessageDecode, MessageEncode, PbEncoder};
use opentelemetry_micropb as otlp;
use test::{black_box, Bencher};

const BATCH_SIZE: usize = 64;

fn encode_message<M: MessageEncode>(message: &M) -> Vec<u8> {
    let mut out = Vec::new();
    let mut encoder = PbEncoder::new(&mut out);
    message.encode(&mut encoder).unwrap();
    out
}

fn make_std_span() -> otlp::std::trace_::v1_::Span {
    use otlp::std::common_::v1_::{AnyValue, AnyValue_::Value, KeyValue};
    use otlp::std::trace_::v1_::{
        Span, Span_::Event, Span_::Link, Span_::SpanKind, Status, Status_::StatusCode,
    };

    let mut attr_short = KeyValue::default();
    attr_short
        .set_key("service.name".to_owned())
        .set_value(AnyValue {
            r#value: Some(Value::StringValue("checkout".to_owned())),
        });

    let mut attr_long = KeyValue::default();
    attr_long
        .set_key("payload".to_owned())
        .set_value(AnyValue {
            r#value: Some(Value::BytesValue(
                b"this-bytes-payload-is-long-enough-to-grow".to_vec(),
            )),
        });

    let mut event_attr = KeyValue::default();
    event_attr
        .set_key("db.statement".to_owned())
        .set_value(AnyValue {
            r#value: Some(Value::StringValue(
                "SELECT * FROM orders WHERE user_id = 42".to_owned(),
            )),
        });

    let event = Event {
        r#time_unix_nano: 1_718_000_000,
        r#name: "db.query".to_owned(),
        r#attributes: vec![event_attr],
        r#dropped_attributes_count: 0,
    };

    let link = Link {
        r#trace_id: vec![0x11; 16],
        r#span_id: vec![0x22; 8],
        r#trace_state: "vendor=something".to_owned(),
        r#attributes: vec![],
        r#dropped_attributes_count: 0,
        r#flags: 1,
    };

    let mut span = Span::default();
    span
        .set_trace_id((0u8..16).collect())
        .set_span_id((16u8..24).collect())
        .set_trace_state("rojo=00f067aa0ba902b7".to_owned())
        .set_parent_span_id((24u8..32).collect())
        .set_flags(0x01)
        .set_name("GET /v1/checkout/complete/with/a/long/path".to_owned())
        .set_kind(SpanKind::Server)
        .set_start_time_unix_nano(1_717_171_717)
        .set_end_time_unix_nano(1_717_171_999)
        .set_dropped_attributes_count(3)
        .set_dropped_events_count(2)
        .set_dropped_links_count(1)
        .set_status(Status {
            r#message: "status message long enough to spill".to_owned(),
            r#code: StatusCode::Error,
        });
    span.r#attributes = vec![attr_short, attr_long];
    span.r#events = vec![event];
    span.r#links = vec![link];
    span
}

fn make_std_span_bytes() -> Vec<u8> {
    let span = make_std_span();
    encode_message(&span)
}

fn make_area_span(area: &otlp::area::Area) -> otlp::area_proto::trace_::v1_::Span {
    use otlp::area::{AreaBytes, AreaString};
    use otlp::area_proto::common_::v1_::{AnyValue, AnyValue_::Value, KeyValue};
    use otlp::area_proto::trace_::v1_::{
        Span, Span_::Event, Span_::Link, Span_::SpanKind, Status, Status_::StatusCode,
    };

    otlp::area::with_area_in_scope(area, || {
        let mut attr_short = KeyValue::default();
        attr_short
            .set_key(AreaString::from_str("service.name").unwrap())
            .set_value(AnyValue {
                r#value: Some(Value::StringValue(
                    AreaString::from_str("checkout").unwrap(),
                )),
            });

        let mut attr_long = KeyValue::default();
        attr_long
            .set_key(AreaString::from_str("payload").unwrap())
            .set_value(AnyValue {
                r#value: Some(Value::BytesValue(
                    AreaBytes::from_slice(
                        b"this-bytes-payload-is-long-enough-to-grow",
                    )
                    .unwrap(),
                )),
            });

        let mut event_attr = KeyValue::default();
        event_attr
            .set_key(AreaString::from_str("db.statement").unwrap())
            .set_value(AnyValue {
                r#value: Some(Value::StringValue(
                    AreaString::from_str(
                        "SELECT * FROM orders WHERE user_id = 42",
                    )
                    .unwrap(),
                )),
            });

        let mut event = Event::default();
        event.set_time_unix_nano(1_718_000_000);
        event.set_name(AreaString::from_str("db.query").unwrap());
        event.r#attributes.push(event_attr).unwrap();

        let mut link = Link::default();
        link.set_trace_id(AreaBytes::from_slice(&[0x11; 16]).unwrap());
        link.set_span_id(AreaBytes::from_slice(&[0x22; 8]).unwrap());
        link.set_trace_state(AreaString::from_str("vendor=something").unwrap());
        link.set_flags(1);

        let trace_id: Vec<u8> = (0u8..16).collect();
        let span_id: Vec<u8> = (16u8..24).collect();
        let parent_span_id: Vec<u8> = (24u8..32).collect();

        let mut span = Span::default();
        span
            .set_trace_id(AreaBytes::from_slice(&trace_id).unwrap())
            .set_span_id(AreaBytes::from_slice(&span_id).unwrap())
            .set_trace_state(AreaString::from_str("rojo=00f067aa0ba902b7").unwrap())
            .set_parent_span_id(AreaBytes::from_slice(&parent_span_id).unwrap())
            .set_flags(0x01)
            .set_name(
                AreaString::from_str("GET /v1/checkout/complete/with/a/long/path").unwrap(),
            )
            .set_kind(SpanKind::Server)
            .set_start_time_unix_nano(1_717_171_717)
            .set_end_time_unix_nano(1_717_171_999)
            .set_dropped_attributes_count(3)
            .set_dropped_events_count(2)
            .set_dropped_links_count(1)
            .set_status(Status {
                r#message: AreaString::from_str("status message long enough to spill").unwrap(),
                r#code: StatusCode::Error,
            });
        span.r#attributes.push(attr_short).unwrap();
        span.r#attributes.push(attr_long).unwrap();
        span.r#events.push(event).unwrap();
        span.r#links.push(link).unwrap();
        span
    })
}

fn make_bumpalo_span() -> Vec<u8> {
    use otlp::bumpalo::{UnsafeString, UnsafeVec};
    use otlp::bumpalo::common_::v1_::{AnyValue, AnyValue_::Value, KeyValue};
    use otlp::bumpalo::trace_::v1_::{
        Span, Span_::Event, Span_::Link, Span_::SpanKind, Status, Status_::StatusCode,
    };

    fn bump_string(text: &str) -> UnsafeString {
        let mut out = UnsafeString::default();
        out.0.push_str(text);
        out
    }

    fn bump_bytes(data: &[u8]) -> UnsafeVec<u8> {
        let mut out = UnsafeVec::default();
        out.0.extend_from_slice_copy(data);
        out
    }

    let bump = bumpalo::Bump::with_capacity(1024);
    otlp::bumpalo::with_pool_in_scope(&bump, || {
        let mut attr_short = KeyValue::default();
        attr_short
            .set_key(bump_string("service.name"))
            .set_value(AnyValue {
                r#value: Some(Value::StringValue(bump_string("checkout"))),
            });

        let mut attr_long = KeyValue::default();
        attr_long
            .set_key(bump_string("payload"))
            .set_value(AnyValue {
                r#value: Some(Value::BytesValue(bump_bytes(
                    b"this-bytes-payload-is-long-enough-to-grow",
                ))),
            });

        let mut event_attr = KeyValue::default();
        event_attr
            .set_key(bump_string("db.statement"))
            .set_value(AnyValue {
                r#value: Some(Value::StringValue(bump_string(
                    "SELECT * FROM orders WHERE user_id = 42",
                ))),
            });

        let mut event = Event::default();
        event.set_time_unix_nano(1_718_000_000);
        event.set_name(bump_string("db.query"));
        micropb::PbVec::pb_push(&mut event.r#attributes, event_attr).unwrap();

        let mut link = Link::default();
        link.set_trace_id(bump_bytes(&[0x11; 16]));
        link.set_span_id(bump_bytes(&[0x22; 8]));
        link.set_trace_state(bump_string("vendor=something"));
        link.set_flags(1);

        let trace_id: Vec<u8> = (0u8..16).collect();
        let span_id: Vec<u8> = (16u8..24).collect();
        let parent_span_id: Vec<u8> = (24u8..32).collect();

        let mut span = Span::default();
        span
            .set_trace_id(bump_bytes(&trace_id))
            .set_span_id(bump_bytes(&span_id))
            .set_trace_state(bump_string("rojo=00f067aa0ba902b7"))
            .set_parent_span_id(bump_bytes(&parent_span_id))
            .set_flags(0x01)
            .set_name(bump_string(
                "GET /v1/checkout/complete/with/a/long/path",
            ))
            .set_kind(SpanKind::Server)
            .set_start_time_unix_nano(1_717_171_717)
            .set_end_time_unix_nano(1_717_171_999)
            .set_dropped_attributes_count(3)
            .set_dropped_events_count(2)
            .set_dropped_links_count(1)
            .set_status(Status {
                r#message: bump_string("status message long enough to spill"),
                r#code: StatusCode::Error,
            });
        micropb::PbVec::pb_push(&mut span.r#attributes, attr_short).unwrap();
        micropb::PbVec::pb_push(&mut span.r#attributes, attr_long).unwrap();
        micropb::PbVec::pb_push(&mut span.r#events, event).unwrap();
        micropb::PbVec::pb_push(&mut span.r#links, link).unwrap();

        encode_message(&span)
    })
}

#[bench]
fn encode_std(b: &mut Bencher) {
    b.iter(|| {
        let mut total = 0usize;
        for _ in 0..BATCH_SIZE {
            let span = make_std_span();
            let bytes = encode_message(black_box(&span));
            total += bytes.len();
        }
        black_box(total);
    });
}

#[bench]
fn encode_area(b: &mut Bencher) {
    b.iter(|| {
        let pool = Box::new(otlp::area::AreaPool::new(4096, 128));
        let area = pool.checkout().unwrap();
        let mut total = 0usize;
        for _ in 0..BATCH_SIZE {
            let span = make_area_span(&area);
            let bytes =
                otlp::area::with_area_in_scope(&area, || encode_message(black_box(&span)));
            total += bytes.len();
        }
        black_box(total);
    });
}

#[bench]
fn encode_bumpalo(b: &mut Bencher) {
    b.iter(|| {
        let mut total = 0usize;
        for _ in 0..BATCH_SIZE {
            let bytes = make_bumpalo_span();
            total += bytes.len();
        }
        black_box(total);
    });
}

#[bench]
fn decode_encode_std(b: &mut Bencher) {
    let bytes = make_std_span_bytes();
    b.iter(|| {
        let mut total = 0usize;
        for _ in 0..BATCH_SIZE {
            let mut span = otlp::std::trace_::v1_::Span::default();
            span.decode_from_bytes(black_box(&bytes)).unwrap();
            let out = encode_message(&span);
            total += out.len();
        }
        black_box(total);
    });
}

#[bench]
fn decode_encode_area(b: &mut Bencher) {
    let bytes = make_std_span_bytes();
    b.iter(|| {
        let pool = Box::new(otlp::area::AreaPool::new(4096, 128));
        let area = pool.checkout().unwrap();
        let mut total = 0usize;
        for _ in 0..BATCH_SIZE {
            let out = otlp::area::with_area_in_scope(&area, || {
                let mut span = otlp::area_proto::trace_::v1_::Span::default();
                span.decode_from_bytes(black_box(&bytes)).unwrap();
                encode_message(&span)
            });
            total += out.len();
        }
        black_box(total);
    });
}

#[bench]
fn decode_encode_bumpalo(b: &mut Bencher) {
    let bytes = make_std_span_bytes();
    b.iter(|| {
        let mut total = 0usize;
        for _ in 0..BATCH_SIZE {
            let bump = bumpalo::Bump::with_capacity(2048);
            let out = otlp::bumpalo::with_pool_in_scope(&bump, || {
                let mut span = otlp::bumpalo::trace_::v1_::Span::default();
                span.decode_from_bytes(black_box(&bytes)).unwrap();
                encode_message(&span)
            });
            total += out.len();
        }
        black_box(total);
    });
}
