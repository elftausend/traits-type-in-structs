use std::{marker::PhantomData, ptr::null_mut};


pub trait PtrType<T>: Default {
    fn from_ptr(ptr: *mut T) -> Self;
    fn as_ptr(&self) -> *mut T;
    fn alloc() {}
}
pub trait Device {
    type P<T>: PtrType<T>;
}

pub struct CPU {

}

pub struct PtrCPU<T> {
    ptr: *mut T,
}

impl<T> Default for PtrCPU<T> {
    fn default() -> Self {
        Self { ptr: null_mut() }
    }
}

impl<T> PtrType<T> for PtrCPU<T> {
    fn from_ptr(ptr: *mut T) -> Self {
        PtrCPU {
            ptr
        }
    }

    fn as_ptr(&self) -> *mut T {
        self.ptr
    }
}

impl Device for CPU {
    type P<T> = PtrCPU<T>;
}

#[derive(Debug, Clone, Copy)]
enum Flag {
    None,
    Test,
}

impl Default for Flag {
    fn default() -> Self {
        Flag::None
    }
}

pub struct Buf<T, D: Device> {
    mem: D::P<T>,
    flag: Flag

}

impl<T, D: Device> Default for Buf<T, D> {
    fn default() -> Self {
        Self { 
            mem: D::P::<T>::default(), 
            flag: Default::default() 
        }
    }
}

impl<T, D: Device> Buf<T, D> {
    pub fn new() -> Self {
        Buf {
            mem: D::P::<T>::from_ptr(5 as *mut T),
            ..Default::default()
        }
    }
}

impl<T, D: Device> Drop for Buf<T, D> {
    fn drop(&mut self) {
        println!("ptr: {:?}", self.mem.as_ptr());
    }
}

fn main() {
    let buf = Buf::<i32, CPU>::new();
}
