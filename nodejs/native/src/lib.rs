use neon::prelude::*;
use rust_core::database_test;

fn call_database(mut cx: FunctionContext) -> JsResult<JsString> {
    let database_path = cx.argument::<JsString>(0).unwrap().value();

    let result = database_test(database_path);
    Ok(cx.string(result))
}

register_module!(mut cx, {
    cx.export_function("call_database", call_database)
});
