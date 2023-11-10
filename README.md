# just-convert
Easy conversion of structures 

[<img alt="github" src="https://img.shields.io/badge/github-vettich/just--convert--rs-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/vettich/just-convert-rs)
[<img alt="crates.io" src="https://img.shields.io/crates/v/just-convert.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/just-convert)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-just--convert-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/just-convert)

# Example

```rust
#[derive(JustConvert, Default)]
#[convert(from_into(other::Mouse))]
#[convert(from_into(Cat, default))]
struct Dog {
    #[convert(unwrap(from(Cat)))]
    name: String,
    #[convert(skip(from(other::Mouse)))]
    #[convert(map(from(Cat, ". as i64"), into(Cat, ". as u64")))]
    age: i64,

    // inner of Option must be autoconvert
    #[convert(skip(from_into(other::Mouse)))]
    #[convert(map = ".map(Into::into)")]
    error: Option<DogError>,
}

#[derive(JustConvert, Default)]
struct Cat {
    name: Option<String>,
    age: u64,
    // inner of Option must be autoconvert
    error: Option<CatError>,
}

mod other {
    pub struct Mouse {
        pub name: String,
    }
}
```

## Convert both sides at once

Specify the value for `from_into` as `#[convert(from_into(Struct))]`, instead of specifying `from` and `into` separately

```rust
#[derive(JustConvert)]
#[convert(from_into(some::B))]
struct A {
    // ...
}
```

## Rename field

```rust
#[derive(JustConvert)]
#[convert(from_into(some::B))]
struct A {
    #[convert(rename = user_id)]
    id: String,
}

struct B {
    user_id: String,
}
```

## Execute an arbitrary expression for the conversion

Use the `map` attribute to specify an arbitrary expression

To access the current field, use the dot character, for e.g., ".get_value()".
To access the current structure, use the `this` construct, e.g. "this.field.get_value()"

```rust
#[derive(JustConvert)]
#[convert(from(B))]
struct A {
    // call any method of this field
    #[convert(map = ".to_hex_string()")]
    id: Uuid,
    // casting (or "this.age as i64")
    #[convert(map = ". as i64")]
    age: i64,
    // call any expression
    #[convert(map = "format!(\"id: {}, age: {}\", this.id, this.age)")]
    message: String,
}

struct B {
    id: String,
    age: u64,
}
```

## Auto convert types inside Option or Vec (and Option<Vec<T>> and Vec<Option<T>>)

```rust
#[derive(JustConvert)]
#[convert(from(B))]
struct A {
    value: Option<ValueA>,
    items: Vec<ValueA>,
}

struct B {
    value: Option<ValueB>,
    items: Vec<ValueB>,
}
```

## Ignore some fields

Use the `skip` attribute to ignore convert

```rust
#[derive(JustConvert)]
#[convert(from(B))]
struct A {
    id: String,
    #[convert(skip)]
    age: i64,
}

struct B {
    id: String
}
```

## Declare multiple conversions

Use the `#[convert(...)]` attribute as many times as needed

```rust
#[derive(JustConvert)]
#[convert(from(B))]
#[convert(from_into(C))]
struct A {
    id: String,
    #[convert(skip)]
    age: i64,
}

struct B {
    id: String,
}

struct C {
    id: String,
}
```

## Specialize the value of attributes for each transformation

Optionally, you can specify for which transformation the value
specified in the attribute applies. For example, to rename a field
only for a `from' conversion: `#[convert(rename(from = new_name))]`
or for a specific `#[convert(rename(from(StructName, new_name)))]`

Rules by which values can be specialized:

- Boolean values `#[convert(wrap)]`: `#[convert(wrap(from))]` and `#[convert(wrap(from(StructName)))]`
- Other (with assignment) `#[convert(map = "some_expr")]`: `#[convert(map(from = "some_expr"))]` and `#[convert(map(from(StructName, "some_expr")))]`

```rust
#[derive(JustConvert)]
#[convert(from(B))]
#[convert(from_into(C))]
struct A {
    #[convert(rename(from_into(C, c_id)))]
    #[convert(rename(from = user_id))] // or #[convert(rename(from(B, user_id)))]
    id: String,
    #[convert(skip)]
    age: i64,
}

struct B {
    user_id: String,
}

struct C {
    c_id: String,
}
```


# Inspiration

Thanks to the [struct-convert](https://crates.io/crates/struct-convert) and [derive-from-ext](https://crates.io/crates/derive-from-ext) libraries
