// run-rustfix

fn produces_string() -> Option<String> {
    Some("my cool string".to_owned())
}

fn takes_str(_: &str) -> Option<()> {
    Some(())
}

fn takes_str_mut(_: &mut str) -> Option<()> {
    Some(())
}

fn generic<T>(_: T) -> Option<()> {
    Some(())
}

fn main() {
    let _: Option<()> = produces_string().and_then(takes_str);
    //~^ ERROR type mismatch in function arguments
    //~| HELP call `Option::as_deref()` first
    let _: Option<Option<()>> = produces_string().map(takes_str);
    //~^ ERROR type mismatch in function arguments
    //~| HELP call `Option::as_deref()` first
    let _: Option<Option<()>> = produces_string().map(takes_str_mut);
    //~^ ERROR type mismatch in function arguments
    //~| HELP call `Option::as_deref_mut()` first
    let _ = produces_string().and_then(generic);
}
