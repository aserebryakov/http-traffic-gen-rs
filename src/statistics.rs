use std::collections::HashMap;
use std::fmt;

pub struct Statistics {
    pub connection_attempts: i32,
    pub connections_failed: i32,
    pub requests_sent: i32,
    pub response_codes: HashMap<String, i32>,
}

impl Statistics {
    pub fn new() -> Statistics {
        Statistics{
            connection_attempts: 0,
            connections_failed: 0,
            requests_sent: 0,
            response_codes: HashMap::new(),
            }
    }

    pub fn connection_attempt(&mut self) {
        self.connection_attempts += 1;
    }

    pub fn connection_failed(&mut self) {
        self.connections_failed += 1;
    }

    pub fn count_response(&mut self, response_line: String) {
        let count = match self.response_codes.get(&response_line) {
            Some(count) => count + 1,
            None => 1,
        };

        self.response_codes.insert(response_line, count);
        self.requests_sent += 1;
    }
}

impl fmt::Display for Statistics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Connections attempts {}\n", &self.connection_attempts)?;
        write!(f, "Connections failed {}\n", &self.connections_failed)?;
        write!(f, "Requests sent {}\n", &self.requests_sent)?;

        for (code, count) in &self.response_codes {
            let percent = 100 * count / self.requests_sent;
            write!(f, "{} {} ({}%)", code, count, percent)?;
        }

        Ok(())
    }
}
