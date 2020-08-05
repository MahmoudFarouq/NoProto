## High Performance Serialization Library
FlatBuffers/CapNProto with Flexible Runtime Schemas

[Github](https://github.com/ClickSimply/NoProto) | [Crates.io](https://crates.io/crates/no_proto) | [Documentation](https://docs.rs/no_proto)

### TODO: 
- [x] Finish implementing Lists, Tuples & Maps
- [x] Collection Iterator
- [x] Compaction
- [ ] Documentation
- [ ] Tests

### Features  
- Zero dependencies
- #![no_std] support, WASM ready
- Supports bytewise sorting of buffers
- Thorough Documentation
- Automatic & instant serilization
- Nearly instant deserialization
- Schemas are dynamic/flexible at runtime
- Mutate/Update/Delete values in existing buffers
- Supports native data types
- Supports collection types (list, map, table & tuple)
- Supports deep nesting of collection types

NoProto allows you to store, read & mutate structured data with near zero overhead. It's like Cap'N Proto/Flatbuffers except buffers and schemas are dynamic at runtime instead of requiring compilation.  It's like JSON but faster, type safe and allows native types.

Bytewise sorting comes in the box and is a first class operation. The result is two NoProto buffers can be compared at the byte level *without deserializing* and a correct ordering between the buffer's internal values will be the result.  This is extremely useful for storing ordered keys in databases. 

NoProto moves the cost of deserialization to the access methods instead of deserializing the entire object ahead of time. This makes it a perfect use case for things like database storage or file storage of structured data.

*Compared to FlatBuffers / Cap'N Proto*
- Schemas are dynamic at runtime, no compilation step
- Supports more types and better nested type support
- Bytewise sorting is first class operation
- Mutate (add/delete/update) existing/imported buffers

*Compared to JSON*
- Has schemas / type safe
- Supports bytewise sorting
- Faster serialization & deserialization
- Supports raw bytes & other native types

*Compared to BSON*
- Faster serialization & deserialization
- Has schemas / type safe
- Bytewise sorting is first class operation
- Supports much larger documents (4GB vs 16MB)
- Better collection support & more supported types

*Compared to Serde*
- Supports bytewise sorting
- Objects & schemas are dynamic at runtime
- Faster serialization & deserialization

| Format           | Free De/Serialization | Size Limit | Mutatable | Schemas | Language Agnostic | Runtime Dynamic | Bytewise Sorting |
|------------------|-----------------------|------------|-----------|---------|-------------------|-----------------|------------------|
| **NoProto**      | ✓                     | ~4GB       | ✓         | ✓       | ✓                 | ✓               | ✓                |
| JSON             | 𐄂                     | Unlimited  | ✓         | 𐄂       | ✓                 | ✓               | 𐄂                |
| BSON             | 𐄂                     | ~16KB      | ✓         | 𐄂       | ✓                 | ✓               | 𐄂                |
| MessagePack      | 𐄂                     | Unlimited  | ✓         | 𐄂       | ✓                 | ✓               | 𐄂                |
| FlatBuffers      | ✓                     | ~2GB       | 𐄂         | ✓       | ✓                 | 𐄂               | 𐄂                |
| Protocol Buffers | 𐄂                     | ~2GB       | 𐄂         | ✓       | ✓                 | 𐄂               | 𐄂                |
| Cap'N Proto      | ✓                     | 2^64 Bytes | 𐄂         | ✓       | ✓                 | 𐄂               | 𐄂                |
| Serde            | 𐄂                     | ?          | ✓         | ✓       | 𐄂                 | 𐄂               | 𐄂                |



#### Limitations
- Buffers cannot be larger than 2^32 bytes (~4GB).
- Tables & List collections cannot have more than 2^16 items (~16k).
- Enum/Option types are limited to 2^8 or 255 choices.
- Tuple types are limited to 2^8 or 255 items.
- Buffers are not validated or checked before deserializing.

# Quick Example
```rust
use no_proto::error::NP_Error;
use no_proto::NP_Factory;
use no_proto::NP;
use no_proto::collection::table::NP_Table;
use no_proto::pointer::NP_Ptr;

// JSON is used to describe schema for the factory
// Each factory represents a single schema
// One factory can be used to serialize/deserialize any number of buffers
let user_factory = NP_Factory::new(r#"{
    "type": "table",
    "columns": [
        ["name",   {"type": "string"}],
        ["pass",   {"type": "string"}],
        ["age",    {"type": "uint16"}]
    ]
}"#)?;

// creating a new buffer from the `user_factory` schema
// user_buffer contains a serialized Vec<u8> containing our data

let user_vec: Vec<u8> = user_factory.open(NP::new, |mut buffer| {
    
    // set "name" column to "some name"
    buffer.deep_set("name", "some name".to_owned())?;

    // set "age" column to 75
    buffer.deep_set("age", 75u16)?;

   // done with the buffer
   Ok(buffer)
})?;
 
// open the new buffer, `user_vec`, we just created
// user_vec_2 contains the serialized Vec<u8>
let user_vec_2: Vec<u8> = user_factory.open(NP::buffer(user_vec), |mut buffer| {

   // read the name column
   let mut user_name = buffer.deep_get::<String>("name")?;
   assert_eq!(user_name, Some(Box::new(String::from("some name"))));

   // password value will be None since we haven't set it.
   let mut password = buffer.deep_get::<String>("pass")?;
   assert_eq!(password, None);

   // read age value    
   let mut age = buffer.deep_get::<u16>("age")?;
   assert_eq!(age, Some(Box::new(75)));    

   // done with the buffer
   Ok(buffer)
})?;

// we can now save user_vec_2 to disk, 
// send it over the network, or whatever else is needed with the data
```

## Guided Learning / Next Steps:
1. [`Schemas`](https://docs.rs/no_proto/latest/no_proto/schema/index.html) - Learn how to build & work with schemas.
2. [`Factories`](https://docs.rs/no_proto/latest/no_proto/struct.NP_Factory.html) - Parsing schemas into something you can work with.
3. [`Buffers`](https://docs.rs/no_proto/latest/no_proto/buffer/index.html) - How to create, update & compact buffers.
4. [`Pointers`](https://docs.rs/no_proto/latest/no_proto/pointer/index.html) - How to add, remove and edit values in a buffer.

----------------------

MIT License

Copyright (c) 2020 Scott Lott

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.