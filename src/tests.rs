#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::VulkanRenderer;

    // SDL3 doesn't allow for windows to be instantiated in a test. 
    // All tests involving actually rendering to a window must be done in /examples/*
    #[test]
    #[ignore]
    fn initialization_and_goofing_around() -> Result<(), Box<dyn std::error::Error>> {
        // Not sure what to use this for at the moment.
        println!("ALL GOOD\nALL GOOD\nALL GOOD\n");

        Ok(())
    }
}
