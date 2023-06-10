use itertools::Itertools;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

fn count_chars(text: &[&str]) -> HashMap<char, usize> {
    return text
        .iter()
        .flat_map(|&line| line.chars())
        .filter(|ch| ch.is_alphabetic())
        .filter_map(|ch| ch.to_lowercase().next())
        .counts();
}

#[inline]
pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let chunk_size = input.len() / worker_count + 1;
    let result: Arc<Mutex<HashMap<char, usize>>> = Arc::new(Mutex::new(HashMap::new()));
    let mut map: HashMap<char, usize> = HashMap::new();

    thread::scope(|s| {
        let mut threads = Vec::with_capacity(worker_count);

        for chunk in input.chunks(chunk_size) {
            threads.push(s.spawn(|| {
                let counting: HashMap<char, usize> = count_chars(chunk);
                counting.iter().for_each(|(&ch, &count)| {
                    *(result.lock().unwrap().entry(ch).or_insert(0)) += count;
                });
            }));
        }
        for thread in threads {
            _ = thread.join();
        }
        for item in result.lock().unwrap().iter() {
            map.insert(*(item.0), *(item.1));
        }
        map
    })
}

#[inline]
pub fn frequency_no_mutex(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let chunk_size = input.len() / worker_count + 1;
    thread::scope(|s| {
        let mut map = HashMap::new();
        let mut threads = Vec::with_capacity(worker_count);
        for chunk in input.chunks(chunk_size) {
            threads.push(s.spawn(|| count_chars(chunk)));
        }
        for thread in threads {
            thread.join().unwrap().iter().for_each(|(&ch, &count)| {
                map.entry(ch).and_modify(|c| *c += count).or_insert(count);
            });
        }
        map
    })
}