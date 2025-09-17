// Memory pools for high-performance entity management
use std::collections::VecDeque;

pub struct ObjectPool<T> {
    objects: VecDeque<T>,
    create_fn: Box<dyn Fn() -> T>,
}

impl<T> ObjectPool<T> {
    pub fn new<F>(capacity: usize, create_fn: F) -> Self
    where
        F: Fn() -> T + 'static
    {
        let mut objects = VecDeque::with_capacity(capacity);
        for _ in 0..capacity {
            objects.push_back(create_fn());
        }

        Self {
            objects,
            create_fn: Box::new(create_fn),
        }
    }

    pub fn acquire(&mut self) -> T {
        self.objects.pop_front()
            .unwrap_or_else(|| (self.create_fn)())
    }

    pub fn release(&mut self, object: T) {
        if self.objects.len() < self.objects.capacity() {
            self.objects.push_back(object);
        }
        // Otherwise drop it to prevent unbounded growth
    }

    pub fn len(&self) -> usize {
        self.objects.len()
    }
}

// Fast allocator for temporary mathematics
pub struct ScratchBuffer {
    data: Vec<f32>,
    position: usize,
}

impl ScratchBuffer {
    pub fn new(capacity: usize) -> Self {
        Self {
            data: vec![0.0; capacity],
            position: 0,
        }
    }

    pub fn allocate(&mut self, size: usize) -> &mut [f32] {
        let start = self.position;
        let end = start + size;

        if end > self.data.len() {
            // Reset and start from beginning (simple ring buffer)
            self.position = 0;
            let end = size;
            if end > self.data.len() {
                self.data.resize(end, 0.0);
            }
        }

        self.position = end;
        &mut self.data[start..end]
    }

    pub fn reset(&mut self) {
        self.position = 0;
    }
}