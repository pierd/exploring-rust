use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    future::Future,
    hash::Hash,
    pin::Pin,
    rc::Rc,
    sync::atomic::{AtomicUsize, Ordering},
    task::{Context, RawWaker, RawWakerVTable},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FutureHandle(usize);

impl FutureHandle {
    fn new() -> Self {
        static NEXT_ID: AtomicUsize = AtomicUsize::new(0);
        Self(NEXT_ID.fetch_add(1, Ordering::Relaxed))
    }
}

mod waker {
    use super::*;

    pub struct Waker {
        handle: FutureHandle,
        woken_up_handles: Rc<RefCell<HashSet<FutureHandle>>>,
    }

    impl Waker {
        pub fn new_wrapped(
            handle: FutureHandle,
            woken_up_handles: Rc<RefCell<HashSet<FutureHandle>>>,
        ) -> std::task::Waker {
            let waker = Self {
                handle,
                woken_up_handles,
            };
            let raw = Rc::new(waker).into_raw();
            unsafe { std::task::Waker::from_raw(raw) }
        }

        pub fn into_raw(self: Rc<Self>) -> RawWaker {
            let data = Rc::into_raw(self) as *const ();
            RawWaker::new(data, &VTABLE)
        }
    }

    impl From<Waker> for RawWaker {
        fn from(waker: Waker) -> Self {
            Rc::new(waker).into_raw()
        }
    }

    unsafe fn clone(waker: *const ()) -> RawWaker {
        let waker: Rc<Waker> = Rc::from_raw(waker as *const _);
        waker.clone().into_raw()
    }

    unsafe fn wake(waker: *const ()) {
        let waker: Rc<Waker> = Rc::from_raw(waker as *const _);
        RefCell::borrow_mut(&waker.woken_up_handles).insert(waker.handle);
    }

    unsafe fn wake_by_ref(waker: *const ()) {
        let waker: Rc<Waker> = Rc::from_raw(waker as *const _);
        let waker = waker.clone();
        RefCell::borrow_mut(&waker.woken_up_handles).insert(waker.handle);
    }

    unsafe fn drop(waker: *const ()) {
        let _: Rc<Waker> = Rc::from_raw(waker as *const _);
    }

    pub const VTABLE: RawWakerVTable = RawWakerVTable::new(clone, wake, wake_by_ref, drop);
}

type SharedRef<T> = Rc<RefCell<T>>;
type FutureStore<T> = HashMap<FutureHandle, SharedRef<Pin<Box<dyn Future<Output = T>>>>>;

#[derive(Clone, Default)]
struct Runtime {
    futures: FutureStore<()>,
    woken_up_handles: Rc<RefCell<HashSet<FutureHandle>>>,
}

impl Runtime {
    fn spawn(&mut self, future: Pin<Box<dyn Future<Output = ()>>>) {
        let wrapped = Rc::new(RefCell::new(future));
        let handle = FutureHandle::new();
        self.futures.insert(handle, wrapped.clone());
        self.poll_future_by_handle(handle);
    }

    fn poll_future_by_handle(&mut self, handle: FutureHandle) {
        if let Some(future) = self.futures.get(&handle) {
            let waker = waker::Waker::new_wrapped(handle, self.woken_up_handles.clone());
            let poll_result = Future::poll(
                RefCell::borrow_mut(future).as_mut(),
                &mut Context::from_waker(&waker),
            );
            match poll_result {
                std::task::Poll::Ready(_) => {
                    self.futures.remove(&handle);
                }
                std::task::Poll::Pending => {}
            }
        }
    }

    fn runloop(&mut self) {
        while !self.futures.is_empty() {
            let woken_up_handles =
                std::mem::take(&mut *RefCell::borrow_mut(&self.woken_up_handles));
            for handle in woken_up_handles {
                self.poll_future_by_handle(handle);
            }
        }
    }
}

#[pin_project::pin_project]
struct CountDown(usize, &'static str);
impl Future for CountDown {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> std::task::Poll<Self::Output> {
        let this = self.project();
        println!("CountDown::poll {} {}", *this.1, *this.0);
        if *this.0 == 0 {
            std::task::Poll::Ready(())
        } else {
            *this.0 -= 1;
            cx.waker().wake_by_ref();
            std::task::Poll::Pending
        }
    }
}

fn main() {
    let mut runtime = Runtime::default();
    runtime.spawn(Box::pin(async {
        println!("Hello, world!");
    }));
    runtime.spawn(Box::pin(CountDown(3, "just counting down")));
    runtime.spawn(Box::pin(async {
        CountDown(5, "counting down to late hello").await;
        println!("Late hello!");
    }));
    runtime.runloop();
}
