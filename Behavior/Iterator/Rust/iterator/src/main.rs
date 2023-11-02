// 定义一个结构体来持有迭代器的状态
struct Counter {
    count: usize,
    max: usize,
}

// 为我们的结构体实现迭代器trait
impl Iterator for Counter {
    // 迭代器将产生usize类型的值
    type Item = usize;

    // next方法将被调用以获取序列的下一个值
    fn next(&mut self) -> Option<Self::Item> {
        // 如果当前计数小于最大值，返回当前计数并递增
        if self.count < self.max {
            let ret = Some(self.count);
            self.count += 1;
            ret
        } else {
            // 如果已经到达或超过最大值，返回None
            None
        }
    }
}

// 实现一个新函数来创建新的Counter实例
impl Counter {
    fn new(max: usize) -> Self {
        Counter { count: 0, max }
    }
}

fn main() {
    // 创建一个新的Counter实例，将最大值设置为5
    let mut counter = Counter::new(5);

    // 使用迭代器遍历序列
    while let Some(i) = counter.next() {
        println!("{}", i);
    }
}
