use std::{collections::HashMap, time::Duration};
use tokio::{
    sync::{
        broadcast, mpsc,
        watch::{self},
    },
    time::Sleep,
};

pub struct RegistrationTracker {
    registration_timers: HashMap<String, RegistrationTimer>,
    deregistration_listener: watch::Receiver<String>,
    deregistration_sender: watch::Sender<String>,
    registration_listener: mpsc::Receiver<(String, Duration)>,
}

impl RegistrationTracker {
    pub fn new(receiver: mpsc::Receiver<(String, Duration)>) -> Self {
        let (tx, rx) = watch::channel::<String>("".to_string());
        RegistrationTracker {
            registration_timers: HashMap::new(),
            deregistration_listener: rx,
            deregistration_sender: tx,
            registration_listener: receiver,
        }
    }

    pub fn get_deregistration_listener(&self) -> watch::Receiver<String> {
        self.deregistration_listener.clone()
    }
}

pub struct RegistrationTimer {
    server_endpoint: String,
}

impl RegistrationTimer {
    pub fn new(server_endpoint: String) {}
}
