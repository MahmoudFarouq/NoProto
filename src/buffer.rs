//! Top level abstraction for buffer objects

use alloc::boxed::Box;
use alloc::vec::Vec;
use crate::pointer::{NP_Value};
use crate::error::NP_Error;
use crate::pointer::{any::NP_Any, NP_Ptr, NP_Lite_Ptr};
use crate::memory::{NP_Size, NP_Memory};
use crate::{schema::{NP_TypeKeys, NP_Schema, NP_Schema_Ptr}, json_flex::NP_JSON};
use alloc::{borrow::ToOwned};

/// The address location of the root pointer.
pub const ROOT_PTR_ADDR: u32 = 2;

/// Buffers contain the bytes of each object and allow you to perform reads, updates, deletes and compaction.
/// 
/// ## Next Step
/// 
/// Read about how to use pointers to access and mutate the internals of the buffer.
/// 
/// [Go to NP_Ptr docs](../pointer/struct.NP_Ptr.html)
#[derive(Debug)]
pub struct NP_Buffer<'buffer> {
    /*memory: NP_Memory,
    schema: &'buffer NP_Schema,
    schema_ptr: NP_Schema_Ptr<'buffer>,
    root_ptr: Option<NP_Ptr<'buffer, R>>*/
    memory: NP_Memory,
    schema_ptr: NP_Schema_Ptr<'buffer>
}

/// When calling `maybe_compact` on a buffer, this struct is provided to help make a choice on wether to compact or not.
#[derive(Debug)]
pub struct NP_Compact_Data {
    /// The size of the old buffer
    pub current_buffer: u32,
    /// The estimated size of the new buffer after compaction
    pub estimated_new_size: u32,
    /// How many known wasted bytes in the old buffer
    pub wasted_bytes: u32
}

impl<'buffer> NP_Buffer<'buffer> {

    #[doc(hidden)]
    pub fn _new(model: &'buffer NP_Schema, memory: NP_Memory) -> Self { // make new buffer

        NP_Buffer {
            memory: memory,
            schema_ptr: NP_Schema_Ptr { address: 0, schema: &model}
        }
        /*
        NP_Buffer {
            memory: memory,
            schema: model,
            root_ptr: None,
            schema_ptr: NP_Schema_Ptr { address: 0, schema: &model}
        };

        let addr = ROOT_PTR_ADDR as usize;
        
        buffer.root_ptr = Some(NP_Ptr {
            location: ROOT_PTR_ADDR,
            kind: NP_PtrKinds::Standard { addr: match &buffer.memory.size {
                NP_Size::U32 => u32::from_be_bytes(*buffer.memory.get_4_bytes(addr).unwrap_or(&[0; 4])),
                NP_Size::U16 => u16::from_be_bytes(*buffer.memory.get_2_bytes(addr).unwrap_or(&[0; 2])) as u32,
                NP_Size::U8 => u8::from_be_bytes([buffer.memory.get_1_byte(addr).unwrap_or(0)]) as u32
            }},
            memory: &buffer.memory,
            schema: &buffer.schema_ptr,
            value: R::default()
        });

        // buffer.root_ptr = Some(NP_Ptr::_new_standard_ptr(ROOT_PTR_ADDR, &buffer.schema_ptr, &buffer.memory));

        buffer*/
    }

    /// Open the buffer to do something fancy with the internals.
    /// 
    /// The type of the root schema should be provided, if the type provided does not match the root schema this operation will fail.
    /// 
    /// Opening the buffer is most common when you want to iterate through a collection or something similar.  [Read more here.](../pointer/struct.NP_Ptr.html#using-collection-types-with-pointers)
    /// 
    pub fn open<R: NP_Value<'buffer> + Default>(&'buffer mut self, callback: &mut (dyn FnMut(NP_Ptr<'buffer, R>) -> Result<(), NP_Error>)) -> Result<(), NP_Error>
    {   

        let root_type =  self.schema_ptr.to_type_data();

        let root: NP_Ptr<R> = NP_Ptr::_new_standard_ptr(ROOT_PTR_ADDR, self.schema_ptr.copy(), &self.memory);

    
        // casting to ANY type -OR- schema is ANY type
        if R::type_idx().0 == NP_TypeKeys::Any as u8 || root_type.0 == NP_TypeKeys::Any as u8  {
            return callback(root);
        }

        // casting matches root schema
        if R::type_idx().0 == root_type.0 {
            return callback(root);
        }


        let mut err = "TypeError: Attempted to cast type (".to_owned();
        err.push_str(R::type_idx().1.as_str());
        err.push_str(") to schema of type (");
        err.push_str(root_type.1.as_str());
        err.push_str(")");
        Err(NP_Error::new(err))
    }

    /// Open the buffer to extract an internal value using custom logic.  This does not work for collection types, only scalar types.
    /// 
    /// The type of the root schema should be provided, if the type provided does not match the root schema this operation will fail.
    /// 
    /// The type of the return value should be provided.
    /// 
    pub fn extract<R: NP_Value<'buffer> + Default, RESULT: NP_Value<'buffer> + Default, F>(&'buffer mut self, callback: &mut (dyn FnMut(NP_Ptr<'buffer, R>) -> Result<RESULT, NP_Error>)) -> Result<RESULT, NP_Error>
    {

        match NP_TypeKeys::from(RESULT::type_idx().0) {
            NP_TypeKeys::Table => { Err(NP_Error::new("Can't extract table type!")) },
            NP_TypeKeys::Map => { Err(NP_Error::new("Can't extract map type!")) },
            NP_TypeKeys::List => { Err(NP_Error::new("Can't extract list type!")) },
            NP_TypeKeys::Tuple => { Err(NP_Error::new("Can't extract tuple type!")) },
            _ => {

                let root_type =  self.schema_ptr.to_type_data();

                let root: NP_Ptr<R> = NP_Ptr::_new_standard_ptr(ROOT_PTR_ADDR, self.schema_ptr.copy(), &self.memory);
    
                // casting to ANY type -OR- schema is ANY type
                if R::type_idx().0 == NP_TypeKeys::Any as u8 || root_type.0 == NP_TypeKeys::Any as u8  {
                    return callback(root);
                }
        
                // casting matches root schema
                if R::type_idx().0 == root_type.0 {
                    return callback(root);
                }
        
                let mut err = "TypeError: Attempted to cast type (".to_owned();
                err.push_str(R::type_idx().1.as_str());
                err.push_str(") to schema of type (");
                err.push_str(root_type.1.as_str());
                err.push_str(")");
                Err(NP_Error::new(err))
            }
        }
    }

    /// Copy the entire buffer into a JSON object
    /// 
    pub fn json_encode(&'buffer self) -> NP_JSON {
        // let root_schema: NP_Schema_Ptr = NP_Schema_Ptr { address: 0, schema: &self.schema};
        let root: NP_Ptr<NP_Any> = NP_Ptr::<NP_Any>::_new_standard_ptr(ROOT_PTR_ADDR, self.schema_ptr.copy(), &self.memory);
        root.json_encode()
    }

    /// Moves the underlying bytes out of the buffer, consuming the buffer in the process.
    /// 
    pub fn close(self) -> Vec<u8> {
        self.memory.dump()
    }

    /// Used to set scalar values inside the buffer, the path only works with dot notation.
    /// This does not work with collection types or `NP_JSON`.
    /// 
    /// The type that you cast the request to will be compared to the schema, if it doesn't match the schema the request will fail.
    /// 
    pub fn deep_set<X: NP_Value<'buffer> + Default>(&'buffer self, path: &str, value: X) -> Result<(), NP_Error> {

        match NP_TypeKeys::from(X::type_idx().0) {
            NP_TypeKeys::JSON => { Err(NP_Error::new("Can't deep set with JSON type!")) },
            NP_TypeKeys::Table => { Err(NP_Error::new("Can't deep set table type!")) },
            NP_TypeKeys::Map => { Err(NP_Error::new("Can't deep set map type!")) },
            NP_TypeKeys::List => { Err(NP_Error::new("Can't deep set list type!")) },
            NP_TypeKeys::Tuple => { Err(NP_Error::new("Can't deep set tuple type!")) },
            _ => {
                let vec_path: Vec<&str> = path.split(".").filter(|v| { v.len() > 0 }).collect();
                let root: NP_Ptr<NP_Any> = NP_Ptr::_new_standard_ptr(ROOT_PTR_ADDR, self.schema_ptr.copy(), &self.memory);
                root._deep_set(vec_path, 0, value)
            }
        }
    }

    /// Clear an inner value from the buffer.  The path only works with dot notation.
    /// This can also be used to clear deeply nested collection objects or scalar objects.
    /// 
    pub fn deep_clear(&'buffer self, path: &str) -> Result<(), NP_Error> {
        let vec_path: Vec<&str> = path.split(".").filter(|v| { v.len() > 0 }).collect();
        let root: NP_Ptr<NP_Any> = NP_Ptr::_new_standard_ptr(ROOT_PTR_ADDR, self.schema_ptr.copy(), &self.memory);
        root._deep_clear(vec_path, 0)
    }
  
    /// Retrieve an inner value from the buffer.  The path only works with dot notation.
    /// You can also use this to get JSON of internal values by casting the request type to `NP_JSON`.
    /// This can also be used to retrieve deeply nested scalar values, but not collection types.
    /// 
    /// The type that you cast the request to will be compared to the schema, if it doesn't match the schema the request will fail.
    /// 
    pub fn deep_get<X: NP_Value<'buffer> + Default>(&'buffer self, path: &str) -> Result<Option<Box<X>>, NP_Error> {

        match NP_TypeKeys::from(X::type_idx().0) {
            NP_TypeKeys::Table => { Err(NP_Error::new("Can't deep get table type from here!")) },
            NP_TypeKeys::Map => { Err(NP_Error::new("Can't deep get map type from here!")) },
            NP_TypeKeys::List => { Err(NP_Error::new("Can't deep get list type from here!")) },
            NP_TypeKeys::Tuple => { Err(NP_Error::new("Can't deep get tuple type from here!")) },
            _ => {
                let vec_path: Vec<&str> = path.split(".").filter(|v| { v.len() > 0 }).collect();
                let root: NP_Ptr<NP_Any> = NP_Ptr::_new_standard_ptr(ROOT_PTR_ADDR, self.schema_ptr.copy(), &self.memory);
                root._deep_get::<X>(vec_path, 0)
            }
        }
    }

    /// This performs a compaction if the closure provided as the second argument returns `true`.
    /// Compaction is a pretty expensive operation (requires full copy of the whole buffer) so should be done sparingly.
    /// The closure is provided an argument that contains the original size of the buffer, how many bytes could be saved by compaction, and how large the new buffer would be after compaction.
    /// 
    /// The first argument, new_capacity, is the capacity of the underlying Vec<u8> that we'll be copying the data into.  The default is the size of the old buffer.
    /// 
    /// The second argument, new_size, can be used to change the size of the address space in the new buffer.  Default behavior is to copy the address size of the old buffer.  Be careful, if you're going from a larg address space down to a smaller one the data might not fit in the new buffer.
    /// 
    pub fn maybe_compact<F>(&'buffer mut self, new_capacity: Option<u32>, new_size: Option<NP_Size>, mut callback: F) -> Result<(), NP_Error> where F: FnMut(NP_Compact_Data) -> bool {

        let bytes_data = self.calc_bytes()?;

        let do_compact = callback(bytes_data);

        if do_compact {
            self.compact(new_capacity, new_size)?
        }

        Ok(())
    }

    /// Compacts a buffer to remove an unused bytes or free space after a mutation.
    /// This is a pretty expensive operation (requires full copy of the whole buffer) so should be done sparingly.
    /// 
    /// The first argument, new_capacity, is the capacity of the underlying Vec<u8> that we'll be copying the data into.  The default is the size of the old buffer.
    /// 
    /// The second argument, new_size, can be used to change the size of the address space in the new buffer.  Default behavior is to copy the address size of the old buffer.  Be careful, if you're going from a larg address space down to a smaller one the data might not fit in the new buffer.
    /// 
    pub fn compact(&'buffer mut self, new_capacity: Option<u32>, new_size: Option<NP_Size>) -> Result<(), NP_Error> {

        let capacity = match new_capacity {
            Some(x) => { x as usize },
            None => self.memory.read_bytes().len()
        };

        let size = match new_size {
            None => self.memory.size,
            Some(x) => { x }
        };

        let old_root = NP_Lite_Ptr::new_standard(ROOT_PTR_ADDR, self.schema_ptr.copy(), &self.memory);
 
        let new_bytes = NP_Memory::new(Some(capacity), size);
        let new_root = NP_Lite_Ptr::new_standard(ROOT_PTR_ADDR, self.schema_ptr.copy(), &new_bytes);

        old_root.compact(new_root)?;

        self.memory = new_bytes;

        Ok(())
    }

    /// Recursively measures how many bytes each element in the buffer is using and subtracts that from the size of the buffer.
    /// This will let you know how many bytes can be saved from a compaction.
    /// 
    pub fn calc_bytes(&'buffer self) -> Result<NP_Compact_Data, NP_Error> {

        let root: NP_Ptr<NP_Any> = NP_Ptr::_new_standard_ptr(ROOT_PTR_ADDR, self.schema_ptr.copy(), &self.memory);

        let real_bytes = root.calc_size()? + ROOT_PTR_ADDR;
        let old_size = self.memory.read_bytes().len() as u32;

        if old_size >= real_bytes {
            return Ok(NP_Compact_Data {
                current_buffer: real_bytes,
                estimated_new_size: real_bytes - (old_size - real_bytes),
                wasted_bytes: old_size - real_bytes
            });
        } else {
            return Err(NP_Error::new("Error calculating wasted bytes!"));
        }
    }
}