use super::input_enum::EchoInputDataFields;
use super::output_enum::EchoInputDataOutputFields;

/// Holds both input and output data for the echo_input_data endpoint
/// 
/// This struct maintains the state of both the input received and
/// the output to be sent for a single echo request-response cycle.
#[derive(Debug)]
pub struct EchoInputDataModuleData {
    /// The input received from the request
    pub input: EchoInputDataFields,
    /// The output to be sent in the response
    pub output: EchoInputDataOutputFields,
}
