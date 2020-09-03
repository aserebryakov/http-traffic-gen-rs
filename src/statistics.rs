use std::collections::HashMap;
use std::fmt;

#[derive(Clone)]
pub enum StatisticsUpdate {
    ConnectionAttempt,
    Sent,
    Received(String),
    Error(String),
    Empty,
}

struct StatisticsSummary {
    connection_attempts: i32,
    connections_failed: i32,
    requests_sent: i32,
    responses: HashMap<String, i32>,
}

impl StatisticsSummary {
    fn new() -> StatisticsSummary {
        StatisticsSummary {
            connection_attempts: 0,
            connections_failed: 0,
            requests_sent: 0,
            responses: HashMap::new(),
        }
    }

    pub fn update(&mut self, update: StatisticsUpdate) {
        match update {
            StatisticsUpdate::ConnectionAttempt => self.connection_attempt(),
            StatisticsUpdate::Sent => self.request_sent(),
            StatisticsUpdate::Received(response) => self.count_response(response),
            StatisticsUpdate::Error(cause) => self.connection_failed(cause),
            StatisticsUpdate::Empty => (),
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
}

impl fmt::Display for StatisticsSummary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Connections attempts {}\n", &self.connection_attempts)?;
        write!(f, "Connections failed {}\n", &self.connections_failed)?;
        write!(f, "Requests sent {}\n", &self.requests_sent)?;
        write!(f, "\n")?;

        for (code, count) in &self.responses {
            let percent = 100 * count / self.connection_attempts;
            write!(f, "{:50} {:8} ({:3}%) {:*<width$}\n", code, count, percent, "", width = (percent/10 + 1) as usize)?;
        }

        Ok(())
    }
}

pub struct Statistics {
    run_summary: StatisticsSummary,
    last_updates: Vec::<StatisticsUpdate>,
    sliding_window_size: usize,
    sliding_window_cursor: usize,
}

impl Statistics {
    pub fn new() -> Statistics {
        Statistics {
            run_summary: StatisticsSummary::new(),
            last_updates: Vec::new(),
            sliding_window_size: 4096,
            sliding_window_cursor: 0,
        }
    }

    pub fn update(&mut self, update: StatisticsUpdate) {
        self.run_summary.update(update.clone());
        self.update_sliding_window(update);
    }

    fn update_sliding_window(&mut self, update: StatisticsUpdate) {
        if self.last_updates.len() < self.sliding_window_size {
            self.last_updates.resize(self.sliding_window_size, StatisticsUpdate::Empty);
        }

        self.last_updates[self.sliding_window_cursor] = update;
        self.sliding_window_cursor += 1;
        self.sliding_window_cursor %= self.sliding_window_size;
    }

    fn get_sliding_window_stats(&self) -> StatisticsSummary {
        let mut summary = StatisticsSummary::new();
        for update in &self.last_updates {
            summary.update(update.clone());
        }

        summary
    }
}

impl fmt::Display for Statistics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Run Summary\n")?;
        write!(f, "-----------\n")?;
        write!(f, "{}\n", self.run_summary)?;

        write!(f, "Last {} updates\n", self.sliding_window_size)?;
        write!(f, "-----------\n")?;
        write!(f, "{}\n", self.get_sliding_window_stats())?;

        Ok(())
    }
}
