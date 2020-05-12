use arrayvec::ArrayVec;
use std::ptr::NonNull;

type Arity = usize;

const ARITY: Arity = 4;

struct Pointer<T> {
    chunk: NonNull<Chunk<T>>,
    offset: usize,
}

enum Elem<T> {
    Sentinel,
    Value(T),
}

struct ChunkedArrayQueue<T> {
    front: Pointer<T>,
    back: Pointer<T>,
}

struct Chunk<T> {
    values: ArrayVec<[T; ARITY]>,
    next: Pointer<T>,
}

impl<T> Pointer<T> {
    fn new(chunk: &Chunk<T>) -> Self {
        Self {
            chunk: NonNull::from(chunk),
            offset: 0,
        }
    }
    fn set(self, other: Pointer<T>) -> Pointer<T> {
        todo!()
    }
    fn cmp(self, other: Pointer<T>) -> Pointer<T> {
        todo!()
    }
}

impl<T> Chunk<T> {
    fn new(next: NonNull<Chunk<T>>) -> Self {
        todo!()
        //         Self {
        //             values: ArrayVec::new(),
        //             next,
        //         }
    }
}

impl<T> ChunkedArrayQueue<T> {
    fn new() -> Self {
        //         let chunk = Box::new(Chunk::new());
        //         let front = Pointer::new(&chunk);
        //         let back = Pointer::new(&chunk);
        //         Self { front, back }
        todo!()
    }
    fn push_back(&mut self, v: T) {}
    fn pop_front(&mut self) -> T {
        todo!()
    }
    fn next(&self, p: Pointer<T>) {
        todo!()
    }
    fn prev(&self, p: Pointer<T>) {
        todo!()
    }
    fn read(&self, p: Pointer<T>) -> T {
        todo!()
    }
    fn write(&self, p: Pointer<T>) -> T {
        todo!()
    }
}
