/*
Exercise: Protobuf Parsing
In this exercise, you will build a parser for the protobug binary encoding.
Don't wory, it's simpler than it seems! This illustrates a common parsing
pattern, passing slices of data. The underlying data itself is never copied.

Fully parsing a protobuf message requires knowing the types of the fields,
indexed by their field numbers. That is typically provided in a proto file.
In this exercise, we'll encode that information into match statements in
functions that get called for each field.

We'll use the following proto:

message PhoneNumber {
    optional string number = 1;
    optional string type = 2;
}

message Person {
    optional string name = 1;
    optional int32 id = 2;
    repeated PhoneNumber phones = 3;
}

Messages
A proto message is encoded as a series of fields, one after the next. Each
is implemented as a "tag" followed by the value. The tag contains a field
number (e.g., 2 for the id field of a Person message) and a wire type
defining how the payload should be determined from the byte stream. These
are combined into a single integer, as decoded in unpack_tag below.

Varint
Integers, including the tag, are represented with a variable-length
encoding called VARINT. Luckily, parse_varint is defined for you below.

Wire Types
Proto defines several wire types, only two of which are used in this
exercise.

The Varint wire type contains a single varint, and is used to encode proto
values of type int32 such as Person.id.

The Len wire type contains a length expressed as varint, followed by a
payload of that number of bytes. This is used to encode proto values of
type string such as Person.name. It is also used to encode proto values
containing sub-messages such as Person.phones, where the payload contains
an encoding of the sub-message.


Exercise
The given code also defines callbacks to handle Person and PhoneNumber
fields, and to parse a message into series of calls to those callbacks.

What remains for you is to implement the parse_field function and the
ProtoMessage trait for Person and PhoneNumber.
*/

/// A wire type as seen on the wire.
enum WireType {
    /// The Varint WireType indicates the value is a single VARINT.
    Varint,
    // The I64 WireType indicates that the value is precisely 8 bytes in
    // little-endian order containing a 64-bit signed integer or double type.
    //I64, -- not needed for this exercise
    /// The Len WireType indicates that the value is a length represented as a
    /// VARINT followed by exactly that number of bytes.
    Len,
    // The I32 WireType indicates that the value is precisely 4 bytes in
    // little-endian order containing a 32-bit signed integer or float type.
    //I32, -- not needed for this exercise
}

#[derive(Debug)]
/// A field's value, typed based on the wire type.
enum FieldValue<'a> {
    Varint(u64),
    //I64(i64), -- not needed for this exercise
    Len(&'a [u8]),
    //I32(i32), -- not needed for this exercise
}

#[derive(Debug)]
/// A field, containing the field number and its value.
struct Field<'a> {
    field_num: u64,
    value: FieldValue<'a>,
}

trait ProtoMessage<'a>: Default {
    fn add_field(&mut self, field: Field<'a>);
}

impl From<u64> for WireType {
    fn from(value: u64) -> Self {
        match value {
            0 => WireType::Varint,
            //1 => WireType::I64, -- not needed for this exercise
            2 => WireType::Len,
            //5 => WireType::I32, -- not needed for this exercise
            _ => panic!("Invalid wire type: {value}"),
        }
    }
}

impl<'a> Field<'a> {
    fn as_str(&self) -> &'a str {
        let FieldValue::Len(data) = self else {
            panic!("Expected string to be a `Len` field");
        };
        std::str::from_utf8(data).expect("Invalid string")
    }

    fn as_bytes(&self) -> &'a [u8] {
        let FieldValue::Len(data) = self else {
            panic!("Expected bytes to be a `Len` field")
        };
        data
    }

    fn as_u64(&self) -> u64 {
        let FieldValue::Varint(value) = self else {
            panic!("Expected `u64` to be a `Varint` field");
        };
        *value
    }
}
