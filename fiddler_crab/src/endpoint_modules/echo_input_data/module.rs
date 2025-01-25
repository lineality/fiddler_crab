use super::input_enum::EchoInputDataFields;
use super::output_enum::EchoInputDataOutputFields;
use super::struct::EchoInputDataModuleData;

/// Processes the echo_input_data request by echoing back the input
/// 
/// # Arguments
/// * `module_data` - The EchoInputDataModuleData containing the input to echo
/// 
/// # Returns
/// * `Result<EchoInputDataModuleData, String>` - The processed data or an error
pub fn process_echo_request(mut module_data: EchoInputDataModuleData) 
    -> Result<EchoInputDataModuleData, String> {
    
    // Extract the input string
    let input_string = match &module_data.input {
        EchoInputDataFields::InputString(s) => s.clone(),
    };
    
    // Echo it back as output
    module_data.output = EchoInputDataOutputFields::EchoedString(input_string);
    
    Ok(module_data)
}
