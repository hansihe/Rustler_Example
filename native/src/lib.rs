#![feature(plugin)]
#![plugin(rustler_codegen)]

extern crate rustler;
use rustler::{NifEnv, NifTerm, NifError, NifDecoder, NifEncoder, get_atom_init};

rustler_export_nifs!("Elixir.NativeTest", [
                     ("add", 2, add), 
                     ("panic_test", 0, panic_test),
                     ("struct_argument", 1, struct_argument)]);

fn add<'a>(env: &'a NifEnv, args: &Vec<NifTerm>) -> Result<NifTerm<'a>, NifError> {
    let num1: i32 = try!(NifDecoder::decode(args[0], env));
    let num2: i32 = try!(NifDecoder::decode(args[0], env));
    Ok(NifEncoder::encode(num1 + num2, env))
}

fn panic_test<'a>(env: &'a NifEnv, args: &Vec<NifTerm>) -> Result<NifTerm<'a>, NifError> {
    let thing: Option<i32> = None;
    // This code will cause a panic. Rust panics when an invariant you have provided is broken.
    // In this case unwrap() is called on an Option that is None, which causes a panic in the code.
    // In safe rust code (not within a unsafe {} block) this is the worst thing that can happen. No
    // segfaults. This means that unless there is a bug in rust, or in rustler, it should be
    // impossible to crash the BEAM.
    // An easier way to cause a panic would be panic!(reason);
    Ok(NifEncoder::encode(thing.unwrap(), env))
}

#[derive(Debug)]
#[ExStruct(module = "Elixir.TestStruct")]
struct TestStruct {
    test_num: i32,
    test_str: String,
    test_bool: bool
}

fn struct_argument<'a>(env: &'a NifEnv, args: &Vec<NifTerm>) -> Result<NifTerm<'a>, NifError> {
    let test_struct: TestStruct = try!(NifDecoder::decode(args[0], env));
    println!("Hello from rust! Struct parameter is: {:?}", test_struct);
    Ok(get_atom_init("ok").to_term(env))
}
