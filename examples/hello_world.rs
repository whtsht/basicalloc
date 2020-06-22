use basic_allocator::UnixAllocator;

#[global_allocator]
static ALLOCATOR: UnixAllocator = UnixAllocator::new();

fn main() {
    env_logger::init();
    println!("Hello, World!");

    let s: String = "abc".to_owned();
    println!("Got a string {}", s);

    let mut v = vec![];
    for n in 0..(1024 * 1024) {
        log::debug!("Pushing {}", n);
        v.push(n);
    }

    let (validity, stats) = ALLOCATOR.stats();
    println!("Valid: {:?}", validity);
    println!("Stats: {:?}", stats);
}
