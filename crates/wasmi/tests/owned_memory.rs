//! Platform-independent checks for the owned Wasmi memory hook.
#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};
    use wasmi::{Config, Engine, Memory, MemoryAllocator, MemoryBuffer, MemoryType, Store};

    #[derive(Debug)]
    struct Allocator {
        capacity: usize,
        released: Arc<Mutex<Vec<Vec<u8>>>>,
    }

    #[derive(Debug)]
    struct Buffer {
        bytes: Vec<u8>,
        released: Arc<Mutex<Vec<Vec<u8>>>>,
    }

    // The Vec exclusively owns its allocation and never changes capacity.
    unsafe impl MemoryBuffer for Buffer {
        fn data_ptr(&self) -> *mut u8 {
            self.bytes.as_ptr().cast_mut()
        }
        fn capacity(&self) -> usize {
            self.bytes.len()
        }
    }

    impl Drop for Buffer {
        fn drop(&mut self) {
            self.released
                .lock()
                .unwrap()
                .push(std::mem::take(&mut self.bytes));
        }
    }

    impl MemoryAllocator for Allocator {
        fn allocate(
            &self,
            _: usize,
            _: Option<usize>,
        ) -> Result<Box<dyn MemoryBuffer>, wasmi::errors::MemoryError> {
            Ok(Box::new(Buffer {
                bytes: vec![7; self.capacity],
                released: Arc::clone(&self.released),
            }))
        }
    }

    fn engine(capacity: usize) -> (Engine, Arc<Mutex<Vec<Vec<u8>>>>) {
        let released = Arc::default();
        let mut config = Config::default();
        config.with_memory_allocator(Arc::new(Allocator {
            capacity,
            released: Arc::clone(&released),
        }));
        (Engine::new(&config), released)
    }

    #[test]
    fn owned_growth_zeros_only_visible_bytes_and_releases_once() {
        let (engine, released) = engine(3 << 16);
        let mut store = Store::new(&engine, ());
        let memory = Memory::new(&mut store, MemoryType::new(1, Some(4))).unwrap();
        let pointer = memory.data_ptr(&store);
        assert!(memory.data(&store).iter().all(|b| *b == 0));
        memory.write(&mut store, 0, b"alive").unwrap();
        memory.grow(&mut store, 1).unwrap();
        assert_eq!(memory.data_ptr(&store), pointer);
        assert_eq!(&memory.data(&store)[..5], b"alive");
        assert!(memory.data(&store)[5..].iter().all(|b| *b == 0));
        assert!(memory.grow(&mut store, 2).is_err());
        assert_eq!(memory.size(&store), 2);
        assert_eq!(memory.ty(&store).maximum(), Some(4));
        assert!(released.lock().unwrap().is_empty());
        drop(store);
        let buffers = released.lock().unwrap();
        assert_eq!(buffers.len(), 1);
        assert!(buffers[0][2 << 16..].iter().all(|b| *b == 7));
    }

    #[test]
    fn insufficient_owned_capacity_releases_failed_allocation() {
        let (engine, released) = engine(65535);
        let mut store = Store::new(&engine, ());
        assert!(Memory::new(&mut store, MemoryType::new(1, None)).is_err());
        assert_eq!(released.lock().unwrap().len(), 1);
        drop(store);
        assert_eq!(released.lock().unwrap().len(), 1);
    }
}
