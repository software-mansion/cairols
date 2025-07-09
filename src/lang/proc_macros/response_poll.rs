use crate::server::schedule::thread::{self, JoinHandle, ThreadPriority};
use crossbeam::channel::{Receiver, RecvTimeoutError, Sender};
use std::time::Duration;

pub struct ResponsePollThread {
    generate_code_complete_receiver: Receiver<()>,
    poll_response_sender: Sender<()>,
}

impl ResponsePollThread {
    pub fn spawn(
        generate_code_complete_receiver: Receiver<()>,
        poll_response_sender: Sender<()>,
    ) -> JoinHandle<()> {
        let this = Self { generate_code_complete_receiver, poll_response_sender };

        thread::Builder::new(ThreadPriority::Worker)
            .name("cairo-ls:pms-response-poll".into())
            .spawn(move || this.event_loop())
            .expect("failed to spawn pms response poll thread")
    }

    fn event_loop(self) {
        loop {
            // Responses can be received after `generate_code_complete_receiver` event.
            // In this case we want to probe every 3 seconds to include these.
            let _ = match self.generate_code_complete_receiver.recv_timeout(Duration::from_secs(3))
            {
                Ok(()) => self.poll_response_sender.try_send(()),
                Err(RecvTimeoutError::Disconnected) => break,
                Err(RecvTimeoutError::Timeout) => self.poll_response_sender.try_send(()),
            };
        }
    }
}
