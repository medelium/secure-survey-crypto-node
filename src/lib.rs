use neon::prelude::*;
use secure_survey::*;

fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("hello node"))
}

fn generate_key_pair(mut cx: FunctionContext) -> JsResult<JsString> {
    let key_pair = KeyPair::new();
    Ok(cx.string(serde_json::to_string(&key_pair).unwrap()))
}

fn encrypt_value(mut cx: FunctionContext) -> JsResult<JsString> {
    let encryption_key = cx.argument::<JsString>(0)?.value(&mut cx);
    let value = cx.argument::<JsNumber>(1)?.value(&mut cx) as u64;

    let ek = load_encryption_key(&*encryption_key);
    let encrypted = encrypt(&ek, value);
    Ok(cx.string(serde_json::to_string(&encrypted).unwrap()))
}


fn add(mut cx: FunctionContext) -> JsResult<JsString> {
    let encryption_key = cx.argument::<JsString>(0)?.value(&mut cx);
    let cipher_a = cx.argument::<JsString>(1)?.value(&mut cx);
    let cipher_b = cx.argument::<JsString>(2)?.value(&mut cx);

    let ek = load_encryption_key(&*encryption_key);
    let a: EncodedCiphertext<u64> = serde_json::from_str(&*cipher_a).unwrap();
    let b: EncodedCiphertext<u64> = serde_json::from_str(&*cipher_b).unwrap();

    let result = add_ciphers(&ek, &a, &b);

    Ok(cx.string(serde_json::to_string(&result).unwrap()))
}


fn add_many(mut cx: FunctionContext) -> JsResult<JsString> {
    let encryption_key = cx.argument::<JsString>(0)?.value(&mut cx);
    let cyphers = cx.argument::<JsArray>(1)?;

    let ek = load_encryption_key(&*encryption_key);
    let mut ciphers_owned: Vec<EncodedCiphertext<u64>> = cyphers
        .to_vec(&mut cx)
        .unwrap()
        .into_iter()
        .map(|x| {
            // let z = x.downcast(&mut cx).unwrap().to_string();
            let val = x
                .downcast::<JsString, _>(&mut cx)
                .or_throw(&mut cx)
                .map(|x| x.value(&mut cx));
            let z = val.unwrap();
            let cipher: EncodedCiphertext<u64> = serde_json::from_str(&z).unwrap();
            cipher
        }
        )
        .collect();

    println!("ciphers_owned: {:?}", ciphers_owned);

    let sum = add_all(&ek, &ciphers_owned);
    Ok(cx.string(serde_json::to_string(&sum).unwrap()))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("hello", hello)?;
    cx.export_function("generate_key_pair", generate_key_pair)?;
    cx.export_function("encrypt_value", encrypt_value)?;
    cx.export_function("add_cyphers", add)?;
    cx.export_function("add_many", add_many)?;
    Ok(())
}
