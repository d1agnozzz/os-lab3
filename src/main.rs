mod utils;

use std::error::Error;
use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};
use colored::Colorize;
use std::sync::Barrier;
use signal_hook::{consts::SIGINT, iterator::Signals};
fn main() -> Result<(), Box<dyn Error>>{
    let mut signals = Signals::new(&[SIGINT])?;

    thread::spawn(move || {
        for sig in signals.forever() {
            println!("\nПринят сигнал: {:?}", sig);
            if sig == SIGINT {
                println!("Принят сигнал прерывания, выход из программы...");
                std::process::exit(0);
            }
        }
    });

    task_mutex_array();
    
    task_barrier();

    println!("Бесконечный цикл ∞\n");
    loop{};

    Ok(())
}



fn task_mutex_array() {
    println!("\nМьютекс\n");
    println!("Массив из трех элементов. Потоки будут увеличивать первый элемент на 1, второй на 2, третий на 3\n");

    let arc = Arc::new(vec![Mutex::new(0), Mutex::new(0), Mutex::new(0)]);
    let mut handles = vec![];

    for t in 0..15 {
        let cloned_arc = arc.clone();
        let handle = thread::spawn(move || {
            let cell_ind = t % 3;
            let mut cell = cloned_arc[cell_ind].lock().unwrap();
            *cell += (t % 3) + 1;
            println!("Поток {t} увеличил vec[{}] на {}\t текущий  vec[{}]: {}", cell_ind, cell_ind + 1, cell_ind, *cell);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("{:?}", arc);
}



fn task_barrier() {
    const THRESHOLD: usize = 5;
    const THREADS_QUANTITY: usize = 10;
    println!("\nБарьер\n");
    println!("Барьер будет {} {} потоков", "ожидать".red(), THRESHOLD);
    println!(
        "Всего будет {} {} потоков",
        "запущено".green(),
        THREADS_QUANTITY
    );
    println!();

    let mut handles = vec![];
    let barrier = Arc::new(Barrier::new(THRESHOLD));
    for t in 1..=THREADS_QUANTITY {
        let c = Arc::clone(&barrier);
        // The same messages will be printed together.
        // You will NOT see any interleaving.
        handles.push(thread::spawn(move || {
            thread::sleep(Duration::from_millis(1000 * t as u64));
            println!("Вывод потока {t} до блокировки");
            // thread::sleep(Duration::from_millis(1000));

            c.wait();
            thread::sleep(Duration::from_nanos(1));
            println!("\tВывод потока {t} после блокировки");
        }));
    }
    // Wait for other threads to finish.
    for handle in handles {
        handle.join().unwrap();
    }
}
