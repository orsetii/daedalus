//! This module defines the endpoints for
//! receiving and processing the CAN traffic from the Van.
//!
//! The key part of this functionality is filtering. Obviously the CAN program
//! receiving the traffic from the bus performs filtering/processing to determine
//! what packets are what, what is happening. Events deemed major enough are then
//! sent to the endpoints below.

#[post("/")]
pub fn post() -> &'static str {
    return "CAN message receieved";
}
