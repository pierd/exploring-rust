use std::rc::{Rc, Weak};

#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

const SIZE: usize = 1024;

struct DropLogger<T>(T);
impl<T> Drop for DropLogger<T> {
    fn drop(&mut self) {
        println!("Dropping {:?}", std::any::type_name::<T>());
    }
}

fn main() {
    let _profiler = dhat::Profiler::new_heap();

    for _ in 0..1_000 {
        let rc: Rc<DropLogger<[u8; SIZE]>> = Rc::new(DropLogger([0u8; SIZE]));
        // leak as weak - this leaks memory, the drop impl will be called but the memory will not be freed
        std::mem::forget(Rc::downgrade(&rc));
    }
}
