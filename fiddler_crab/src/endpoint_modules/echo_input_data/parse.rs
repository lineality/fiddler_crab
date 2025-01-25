use super::input_enum::EchoInputDataFields;
use super::struct::EchoInputDataModuleData;
use super::output_enum::EchoInputDataOutputFields;

/// Parses raw request body string into the EchoInputDataModuleData structure
/// 
/// # Arguments
/// * `request_body` - The raw string from the request body
/// 
/// # Returns
/// * `Result<EchoInputDataModuleData, String>` - The parsed data structure or an error
pub fn parse_echo_input_data(request_body: &str) -> Result<EchoInputDataModuleData, String> {
    // For now, simply wrap the input string
    let input_fields = EchoInputDataFields::InputString(request_body.to_string());
    
    // Initialize with empty output
    let output_fields = EchoInputDataOutputFields::EchoedString(String::new());
    
    Ok(EchoInputDataModuleData {
        input: input_fields,
        output: output_fields,
    })
}
