use std::time::{Instant};

struct UI {
    backend: (),
    target_refresh_rate: u64,
    
}

enum Message {
    Tick(Instant)
}