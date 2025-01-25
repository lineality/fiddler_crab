/// Defines the possible input field types for the echo_input_data endpoint
/// 
/// This enum represents the structure of incoming data that this endpoint
/// can process. Currently handles string input data for echoing back.
#[derive(Debug, Clone)]
pub enum EchoInputDataFields {
    /// String data to be echoed back
    InputString(String),
}
