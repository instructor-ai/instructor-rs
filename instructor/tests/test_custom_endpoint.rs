extern crate instruct_macros;
extern crate instruct_macros_types;

use instructor_ai::from_openai_with_endpoint;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_openai_with_endpoint_creates_client() {
        // Test that we can create an InstructorClient with a custom endpoint
        // This validates the function signature and basic instantiation
        let _instructor_client = from_openai_with_endpoint(
            "http://localhost:8080/v1".to_string(),
            "test-api-key".to_string(),
        );
        // If we reach here, the client was created successfully
    }

    #[test]
    fn test_from_openai_with_endpoint_empty_api_key() {
        // llama-cpp servers typically don't require an API key
        let _instructor_client = from_openai_with_endpoint(
            "http://localhost:8080/v1".to_string(),
            "".to_string(),
        );
    }

    #[test]
    fn test_from_openai_with_endpoint_various_endpoints() {
        // Test various endpoint formats that users might use
        let endpoints = vec![
            "http://localhost:8080/v1",
            "http://127.0.0.1:8000/v1",
            "https://my-server.example.com/v1",
            "http://localhost:11434/v1", // ollama default
        ];

        for endpoint in endpoints {
            let _instructor_client = from_openai_with_endpoint(
                endpoint.to_string(),
                "".to_string(),
            );
        }
    }
}
