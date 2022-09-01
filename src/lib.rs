use neon::prelude::*;
use secure_survey::*;

fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("hello node"))
}

fn generate_key_pair(mut cx: FunctionContext) -> JsResult<JsString> {
    let key_pair = KeyPair::new();
    Ok(cx.string(serde_json::to_string(&key_pair).unwrap()))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("hello", hello)?;
    cx.export_function("generate_key_pair", generate_key_pair)?;
    Ok(())
}
