use arrayvec::ArrayVec;
use std::ptr::NonNull;

type Arity = usize;

const ARITY: Arity = 4;

struct Pointer<Value> {
    chunk: NonNull<Chunk<Value>>,
    offset: usize,
}

enum Elem<Value> {
    Sentinel,
    Value(Value),
}

struct ChunkedArrayQueue<Value> {
    front: Pointer<Value>,
    back: Pointer<Value>,
}

struct Chunk<Value> {
    values: ArrayVec<[Value; ARITY]>,
    next: Pointer<Value>,
}

impl<Value> Pointer<Value> {
    fn new(chunk: &Chunk<Value>) -> Self {
        Self {
            chunk: NonNull::from(chunk),
            offset: 0,
        }
    }
    fn set(self, other: Pointer<Value>) -> Pointer<Value> {
        todo!()
    }
    fn cmp(self, other: Pointer<Value>) -> Pointer<Value> {
        todo!()
    }
}

impl<Value> Chunk<Value> {
    fn new(next: NonNull<Chunk<Value>>) -> Self {
        todo!()
        //         Self {
        //             values: ArrayVec::new(),
        //             next,
        //         }
    }
}

impl<Value> ChunkedArrayQueue<Value> {
    fn new() -> Self {
        //         let chunk = Box::new(Chunk::new());
        //         let front = Pointer::new(&chunk);
        //         let back = Pointer::new(&chunk);
        //         Self { front, back }
        todo!()
    }
    fn push_back(&mut self, v: Value) {}
    fn pop_front(&mut self) -> Value {
        todo!()
    }
    fn next(&self, p: Pointer<Value>) {
        todo!()
    }
    fn prev(&self, p: Pointer<Value>) {
        todo!()
    }
    fn read(&self, p: Pointer<Value>) -> Value {
        todo!()
    }
    fn write(&self, p: Pointer<Value>) -> Value {
        todo!()
    }
}
