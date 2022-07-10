use yex::{
    error::InterpretResult, fields as create_fields, gc::GcRef, insert, insert_fn, literal::TryGet,
    EnvTable, Fn, FnKind, Symbol, Value, VirtualMachine, YexModule,
};

fn simple_fn(args: &[Value]) -> InterpretResult<Value> {
    // if args[0] is not a string will throw a error
    let my_string: String = args[0].get()?;
    Ok(Value::Str(GcRef::new(my_string)))
}

fn print_name(_: *mut VirtualMachine, args: Vec<Value>) -> InterpretResult<Value> {
    let name: String = args[0].get()?;
    println!("Hello, {name}!");
    Ok(Value::Nil)
}

fn console() -> YexModule {
    let mut fields = EnvTable::new();
    // Adding fields to our module
    create_fields!(Console => {
        printName @ print_name => 1
    }, fields);
    YexModule::new(Symbol::new("Console"), fields)
}

// This is first function what will be called on `FFI.open`
#[no_mangle]
pub extern "C" fn init() -> EnvTable {
    let mut my_module = EnvTable::new();
    insert_fn!(my_module, "simple_fn", simple_fn, 1);
    insert!(my_module, "Console", console().into());
    my_module
}

