#![allow(refining_impl_trait)]
// #![allow(refining_impl_trait_internal)]

use crate::first_test_actor::test_function;

mod first_test_actor;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    test_function().await;
    println!("Done");
}
