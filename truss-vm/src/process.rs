use obj::HeapObject;

enum HeapEntry {
    Object(HeapObject)
}

pub struct Process {
    heap: Vec<HeapEntry>
}
