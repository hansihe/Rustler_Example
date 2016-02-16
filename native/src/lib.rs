#![feature(plugin)]
#![plugin(rustler_codegen)]

#[macro_use]
extern crate rustler;
use rustler::{NifEnv, NifTerm, NifError, NifDecoder, NifEncoder};
use rustler::resource::ResourceCell;

rustler_export_nifs!(
    "Elixir.NativeTest", 
    [("add", 2, add), 
     ("panic_test", 0, panic_test),
     ("struct_argument", 1, struct_argument),
     ("make_resource_struct", 0, make_resource_struct),
     ("read_resource_struct", 1, read_resource_struct),
     ("string_test", 0, string_test)],
    Some(on_load)
);

#[NifResource]
struct ResourceStructTest {
    test_field: i32
}

#[NifTuple]
struct TestTuple {
    test_field: i32,
    woohoo: i32
}

fn on_load(env: &NifEnv, load_info: NifTerm) -> bool {
    println!("Runs on library load");
    resource_struct_init!(ResourceStructTest, env);
    true
}

fn add<'a>(env: &'a NifEnv, args: &Vec<NifTerm>) -> Result<NifTerm<'a>, NifError> {
    let num1: i32 = try!(args[0].decode());
    let num2: i32 = try!(args[1].decode());
    Ok((num1 + num2).encode(env))
}

fn panic_test<'a>(env: &'a NifEnv, _args: &Vec<NifTerm>) -> Result<NifTerm<'a>, NifError> {
    let thing: Option<i32> = None;
    // This code will cause a panic. Rust panics when an invariant you have provided is broken.
    // In this case unwrap() is called on an Option that is None, which causes a panic in the code.
    // In safe rust code (not within a unsafe {} block) this is the worst thing that can happen. No
    // segfaults. This means that unless there is a bug in rust, or in rustler, it should be
    // impossible to crash the BEAM.
    // An easier way to cause a panic would be panic!(reason);
    Ok(thing.unwrap().encode(env))
}

#[derive(Debug)]
#[ExStruct(module = "Elixir.TestStruct")]
struct TestStruct<'a> {
    test_num: i32,
    test_str: &'a str,
    test_bool: bool
}

fn struct_argument<'a>(env: &'a NifEnv, args: &Vec<NifTerm>) -> Result<NifTerm<'a>, NifError> {
    let test_struct: TestStruct = try!(args[0].decode());
    println!("Hello from rust! Struct parameter is: {:?}", test_struct);
    Ok(test_struct.encode(env))
}

fn make_resource_struct<'a>(env: &'a NifEnv, args: &Vec<NifTerm>) -> Result<NifTerm<'a>, NifError> {
    let test = ResourceCell::new(ResourceStructTest {
        test_field: 523
    });
    Ok(test.encode(env))
}
fn read_resource_struct<'a>(env: &'a NifEnv, args: &Vec<NifTerm>) -> Result<NifTerm<'a>, NifError> {
    let test: ResourceCell<ResourceStructTest> = try!(args[0].decode());
    println!("WOOOO: {:?}", test.read().unwrap().test_field);
    Ok(12.encode(env))
}

fn string_test<'a>(env: &'a NifEnv, args: &Vec<NifTerm>) -> Result<NifTerm<'a>, NifError> {
    Ok("A static string".encode(env))
}
