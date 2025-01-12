use core::alloc::GlobalAlloc;
use core::alloc::Layout;
use core::mem::size_of;
use std::prelude::rust_2015::Vec;

use crate::FrameAllocator;
use crate::Heap;
use crate::linked_list;
use crate::LockedHeapWithRescue;

#[test]
fn test_linked_list() {
    let mut value1: usize = 0;
    let mut value2: usize = 0;
    let mut value3: usize = 0;
    let mut list = linked_list::LinkedList::new();
    unsafe {
        list.push(&mut value1 as *mut usize);
        list.push(&mut value2 as *mut usize);
        list.push(&mut value3 as *mut usize);
    }

    // Test links
    assert_eq!(value3, &value2 as *const usize as usize);
    assert_eq!(value2, &value1 as *const usize as usize);
    assert_eq!(value1, 0);

    // Test iter
    let mut iter = list.iter();
    assert_eq!(iter.next(), Some(&mut value3 as *mut usize));
    assert_eq!(iter.next(), Some(&mut value2 as *mut usize));
    assert_eq!(iter.next(), Some(&mut value1 as *mut usize));
    assert_eq!(iter.next(), None);

    // Test iter_mut

    let mut iter_mut = list.iter_mut();
    assert_eq!(iter_mut.next().unwrap().pop(), &mut value3 as *mut usize);

    // Test pop
    assert_eq!(list.pop(), Some(&mut value2 as *mut usize));
    assert_eq!(list.pop(), Some(&mut value1 as *mut usize));
    assert_eq!(list.pop(), None);
}

#[test]
fn test_empty_heap() {
    let mut heap = Heap::<32>::new();
    assert!(heap.alloc(Layout::from_size_align(1, 1).unwrap()).is_err());
}

#[test]
fn test_heap_add() {
    let mut heap = Heap::<32>::new();
    assert!(heap.alloc(Layout::from_size_align(1, 1).unwrap()).is_err());

    let space: [usize; 100] = [0; 100];
    unsafe {
        heap.add_to_heap(space.as_ptr() as usize, space.as_ptr().add(100) as usize);
    }
    let addr = heap.alloc(Layout::from_size_align(1, 1).unwrap());
    assert!(addr.is_ok());
}

#[test]
fn test_heap_oom() {
    let mut heap = Heap::<32>::new();
    let space: [usize; 100] = [0; 100];
    unsafe {
        heap.add_to_heap(space.as_ptr() as usize, space.as_ptr().add(100) as usize);
    }

    assert!(heap
        .alloc(Layout::from_size_align(100 * size_of::<usize>(), 1).unwrap())
        .is_err());
    assert!(heap.alloc(Layout::from_size_align(1, 1).unwrap()).is_ok());
}

#[test]
fn test_heap_oom_rescue() {
    static mut SPACE: [usize; 100] = [0; 100];
    let heap = LockedHeapWithRescue::new(|heap: &mut Heap<32>, _layout: &Layout| unsafe {
        heap.add_to_heap(SPACE.as_ptr() as usize, SPACE.as_ptr().add(100) as usize);
    });

    unsafe {
        assert!(heap.alloc(Layout::from_size_align(1, 1).unwrap()) as usize != 0);
    }
}

#[test]
fn test_heap_alloc_and_free() {
    let mut heap = Heap::<32>::new();
    assert!(heap.alloc(Layout::from_size_align(1, 1).unwrap()).is_err());

    let space: [usize; 100] = [0; 100];
    unsafe {
        heap.add_to_heap(space.as_ptr() as usize, space.as_ptr().add(100) as usize);
    }
    for _ in 0..100 {
        let addr = heap.alloc(Layout::from_size_align(1, 1).unwrap()).unwrap();
        heap.dealloc(addr, Layout::from_size_align(1, 1).unwrap());
    }
}

#[test]
fn test_empty_frame_allocator() {
    let mut frame = FrameAllocator::new();
    assert!(frame.alloc(1).is_none());
}

#[test]
fn test_frame_allocator_add() {
    let mut frame = FrameAllocator::new();
    assert!(frame.alloc(1).is_none());

    frame.insert(0..3);
    let num = frame.alloc(1);
    assert_eq!(num, Some(2));
    let num = frame.alloc(2);
    assert_eq!(num, Some(0));
    assert!(frame.alloc(1).is_none());
    assert!(frame.alloc(2).is_none());
}

#[test]
fn test_frame_allocator_alloc_and_free() {
    let mut frame = FrameAllocator::new();
    assert!(frame.alloc(1).is_none());

    frame.add_frame(0, 1024);
    for _ in 0..100 {
        let addr = frame.alloc(512).unwrap();
        frame.dealloc(addr, 512);
    }
}

#[test]
fn test_frame_allocator_alloc_and_free_complex() {
    let mut frame = FrameAllocator::new();
    frame.add_frame(100, 1024);
    for _ in 0..10 {
        let addr = frame.alloc(1).unwrap();
        frame.dealloc(addr, 1);
    }
    let addr1 = frame.alloc(1).unwrap();
    let addr2 = frame.alloc(1).unwrap();
    assert_ne!(addr1, addr2);
}

#[test]
fn test_alloc() {
    const HEAP_SIZE: usize = 64 * 1024 * 1024;
    const ALLOC_SIZE: usize = 32 * 1024 * 1024;
    let buf: Vec<u8> = Vec::with_capacity(HEAP_SIZE);
    unsafe {
        let mut allocator = Heap::<32>::new();
        allocator.init(buf.as_ptr() as usize, HEAP_SIZE);
        let alloc_size: usize = 16;
        for _i in 0..(ALLOC_SIZE / alloc_size) {
            let addr = allocator.alloc(Layout::from_size_align(alloc_size, 1).unwrap());
            assert!(addr.is_ok());
        }
    }
}