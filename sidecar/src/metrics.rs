use prometheus::CounterVec;

use crate::events::{EventBundle, ProcessEvent_oneof_payload::*, ProcessTag};

lazy_static! {
    static ref ACTIVE_PROCESS_COUNTER: CounterVec = CounterVec::new(
        opts!(
            "active_process_count",
            "Number of process/routines running in the system"
        ),
        &["parent_id", "process_id", "tag"]
    )
    .unwrap();
    static ref RESTARTING_PROCESS_COUNTER: CounterVec = CounterVec::new(
        opts!(
            "active_process_count",
            "Number of process/routines running in the system"
        ),
        &["parent_id", "process_id", "tag"]
    )
    .unwrap();
}

fn tag_to_string(tag: &ProcessTag) -> &str {
    match tag {
        ProcessTag::SUPERVISOR => "supervisor",
        ProcessTag::WORKER => "worker",
    }
}

pub fn handle_event_bundle(bundle: &EventBundle) {
    let events = bundle.get_events();
    for event in events {
        match &event.payload {
            Some(process_started(ev)) => {
                println!("{:?}", event);
                let info = ev.get_info();
                if ev.has_first_restart_timestamp() {
                    RESTARTING_PROCESS_COUNTER
                        .with_label_values(&[
                            &info.parent_id,
                            &info.process_id,
                            tag_to_string(&ev.tag),
                        ])
                        .inc()
                } else {
                    ACTIVE_PROCESS_COUNTER
                        .with_label_values(&[
                            &info.parent_id,
                            &info.process_id,
                            tag_to_string(&ev.tag),
                        ])
                        .inc()
                };
            }
            _ => {
                println!("Some other stuff");
            }
        }
    }
}
