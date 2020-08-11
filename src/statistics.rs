use std::collections::HashMap;
use std::fmt;

pub struct Statistics {
    pub connection_attempts: i32,
    pub connections_failed: i32,
    pub requests_sent: i32,
    pub responses: HashMap<String, i32>,
}

pub enum StatisticsUpdate {
    ConnectionAttempt,
    Sent,
    Received(String),
    Error(String),
}

impl Statistics {
    pub fn new() -> Statistics {
        Statistics{
            connection_attempts: 0,
            connections_failed: 0,
            requests_sent: 0,
            responses: HashMap::new(),
            }
    }

    fn connection_attempt(&mut self) {
        self.connection_attempts += 1;
    }

    fn connection_failed(&mut self, cause: String) {
        self.connections_failed += 1;
        self.count_response(cause);
    }

    fn request_sent(&mut self) {
        self.requests_sent += 1;
    }

    fn count_response(&mut self, response_line: String) {
        let count = match self.responses.get(&response_line) {
            Some(count) => count + 1,
            None => 1,
        };

        self.responses.insert(response_line, count);
    }

    pub fn update(&mut self, update: StatisticsUpdate) {
        match update {
            StatisticsUpdate::ConnectionAttempt => self.connection_attempt(),
            StatisticsUpdate::Sent => self.request_sent(),
            StatisticsUpdate::Received(response) => self.count_response(response),
            StatisticsUpdate::Error(cause) => self.connection_failed(cause),
        }
    }
}

impl fmt::Display for Statistics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Connections attempts {}\n", &self.connection_attempts)?;
        write!(f, "Connections failed {}\n", &self.connections_failed)?;
        write!(f, "Requests sent {}\n", &self.requests_sent)?;
        write!(f, "\n")?;

        for (code, count) in &self.responses {
            let percent = 100 * count / self.requests_sent;
            write!(f, "{:50} {:8} ({:3}%) {:*<width$}\n", code, count, percent, "", width = (percent/10 + 1) as usize)?;
        }

        Ok(())
    }
}
