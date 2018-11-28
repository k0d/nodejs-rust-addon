#[macro_use]
extern crate neon;

use neon::prelude::*;

fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
  let val = cx.string("A fancy hello to all ye 🦄s out there.");
  Ok(val)
}

register_module!(mut cx, {
  cx.export_function("hello", hello)
});
