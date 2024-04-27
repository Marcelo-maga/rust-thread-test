fn fatorial(number: u128) -> u128 {

    let mut result: u128 = 1;

    for i in 1..=number {
        result = i * result;
    }
    
    result
}


fn soma(number: u128) -> u128 {

    let mut result: u128 = 1;

    for i in 1..=number {
        result = i + result;
    }
    
    result
}

fn main() {
    let initial_time = chrono::Local::now();
    let fatorial_30 = fatorial(30);
    let soma_30 = soma(30);
    let end_time = chrono::Local::now();

    let execute_time = end_time - initial_time;

    println!("Sem thread fatorial: {}", fatorial_30);
    println!("Sem thread soma: {}", soma_30);
    println!("Tempo sem thread {}", execute_time);

    println!("--------------------");

    let _ = std::thread::spawn(|| {
        let initial_time = chrono::Local::now();
        let fatorial_30 = fatorial(30);
        let soma_30 = soma(30);
        let end_time = chrono::Local::now();
    
        let execute_time = end_time - initial_time;
    
        println!("Com thread fatorial: {}", fatorial_30);
        println!("Com thread soma: {}", soma_30);
        println!("Tempo com thread {}", execute_time);
    
    }).join();

}
