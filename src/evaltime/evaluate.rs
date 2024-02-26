use thiserror::Error;


#[allow(unused)]
struct Evaluation<P> {
    plugins: P,
}

#[allow(unused)]
#[derive(Error, Debug)]
enum EvalError {
    #[error("Value is not a function, {info}")]
    ValueIsNotAFunction { plugin_name: String, info: String },
}
