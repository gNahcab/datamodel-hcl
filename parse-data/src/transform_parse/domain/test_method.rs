enum FunctionType {
    Combine,
    Replace,
    Upper,
    Lower,
    ToDate,
}
trait Function {
    fn function_type() -> FunctionType;
}
struct UpperFunction;
struct LowerFunction{
    input_variable: String,
    output_variable: String,
}
struct CombineFunction{
}

