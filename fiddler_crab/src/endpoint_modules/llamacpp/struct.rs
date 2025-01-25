// endpoint_modules/llamacpp/struct.rs
use super::input_enum::LlamacppInputFields;
use super::output_enum::LlamacppOutputFields;

#[derive(Debug)] 
pub struct LlamacppModuleData {
    pub input: LlamacppInputFields,
    pub output: LlamacppOutputFields, 
    // ... other metadata fields (e.g., data types) ...
}