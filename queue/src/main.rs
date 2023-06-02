trait Queue<T> {
    fn enqueue(&mut self, value: T);
    fn dequeue(&mut self) -> Option<T>;
}

//FIFO
#[derive(Debug)]
struct FIFO<T> {
    data: Vec<T>
}

impl<T> FIFO<T> {
    fn new() -> Self {
        FIFO { data: vec![] }
    }
}

impl<T> Queue<T> for FIFO<T> {
    fn enqueue(&mut self, value: T) {
        self.data.push(value)
    }

    fn dequeue(&mut self) -> Option<T> {
        if self.data.is_empty() {
            None
        } else {
            Some(self.data.remove(0))
        }
    }
}

//FILO
#[derive(Debug)]
struct FILO<T> {
    data: Vec<T>
}

impl<T> FILO<T> {
    fn new() -> Self {
        FILO { data: vec![] }
    }
}

impl<T> Queue<T> for FILO<T> {
    fn enqueue(&mut self, value: T) {
        self.data.push(value)
    }

    fn dequeue(&mut self) -> Option<T> {
        self.data.pop()
    }
}

fn main() {
    let mut fifo: FIFO<String> = FIFO::new();
    fifo.enqueue("a".to_string());
    fifo.enqueue("b".to_string());
    println!("{:?}", fifo);
    fifo.dequeue().unwrap();
    println!("{:?}", fifo);

    let mut filo: FILO<i64> = FILO::new();
    filo.enqueue(1);
    filo.enqueue(2);
    println!("{:?}", filo);
    filo.dequeue().unwrap();
    println!("{:?}", filo);
}

