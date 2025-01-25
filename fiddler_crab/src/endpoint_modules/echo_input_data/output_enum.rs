
/// Defines the possible output field types for the echo_input_data endpoint
/// 
/// This enum represents the structure of the response data that this endpoint
/// will return. Currently handles echoing back string data.
#[derive(Debug, Clone)]
pub enum EchoInputDataOutputFields {
    /// The echoed string data
    EchoedString(String),
}
