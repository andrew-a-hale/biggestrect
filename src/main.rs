use std::env;

fn main() {
    let mut bars: Vec<String> = env::args().into_iter().collect();
    bars.remove(0);

    let bars: Vec<i32> = bars.into_iter().map(|x| {
        match x.parse::<i32>() {
            Ok(x) => x,
            Err(_) => panic!("Unable to parse input")
        }
    }).collect();

    println!("{}", naive(&bars));
    println!("{}", less_naive(&bars));
    println!("{}", stack(&bars));
}

// O(n^3)
fn naive(bars: &Vec<i32>) -> i32 {
    let mut bars = bars.clone();
    let mut max_area = 0;
    loop {
        // calculate biggest rect
        let mut height = bars[0];
        for j in 1..bars.len() {
            if bars[0] > bars[j] {
                break;
            }
            height += bars[0]
        }

        if height > max_area {
            max_area = height
        }

        // consume bar!
        bars[0] -= 1;
        if bars[0] == 0 || bars.len() == 1 {
            bars.remove(0);
        }
        if bars.len() == 0 {
            break;
        }
    }

    max_area
}

// O(n^2)
fn less_naive(bars: &Vec<i32>) -> i32 {
    let mut max_area = 0;
    for i in 0..bars.len() {
        let mut lo = i;
        let mut hi = i;
        while hi < bars.len() - 1 && bars[i] <= bars[hi+1] {
            hi += 1
        }
        while lo > 0 && bars[i] <= bars[lo-1] {
            lo -= 1
        }
        let width = (hi - lo + 1) as i32;
        if bars[i] * width > max_area {
            max_area = bars[i] * width;
        }
    }

    max_area
}

struct Stack<T> {
    data: Vec<T>,
    length: usize
}

impl<T: Copy> Stack<T> {
    fn new() -> Self {
        Stack { data: Vec::<T>::new(), length: 0 }
    }

    fn append(&mut self, x: T) {
        self.data.push(x);
        self.length += 1;
    }

    fn pop(&mut self) -> T {
        self.length -= 1;
        self.data.pop().unwrap()
    }

    fn last(&self) -> T {
        self.data[self.length-1]
    }

    fn is_empty(&self) -> bool {
        self.length == 0
    }

}


fn stack(bars: &Vec<i32>) -> i32 {
    let mut stack = Stack::<usize>::new();
    let mut max_area = 0;
    let mut i = 0;

    while i < bars.len() {
        if stack.is_empty() || bars[i] >= bars[stack.last()] {
            stack.append(i);
            i += 1;
        } else {
            let top = stack.pop();
            let width = if stack.is_empty() {
                i
            } else {
                i - stack.last() - 1
            };
            let area = bars[top] * width as i32;
            if area > max_area {
                max_area = area;
            }
        }
    }

    while !stack.is_empty() {
        let top = stack.pop();
        let width = if stack.is_empty() {
            i
        } else {
            i - stack.last() - 1
        };
        let area = bars[top] * width as i32;
        if area > max_area {
            max_area = area;
        }
    }

    max_area
}