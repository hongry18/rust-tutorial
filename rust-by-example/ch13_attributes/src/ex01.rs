fn used_function() {}

#[allow(dead_code)]
fn unused_function() {}

fn noisy_unused_function() {}

pub fn run() {
    used_function();
}