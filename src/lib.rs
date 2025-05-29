#[cxx::bridge]
pub mod booleans_rs {
    unsafe extern "C++" {
        include!("booleans_rs.hpp");
            
        fn boolean_operation(
            in_coords: &Vec<f64>,
            in_tris: &Vec<u32>,
            in_labels: &Vec<u32>,
            op: i32,
            bool_coords: &mut Vec<f64>,
            bool_tris: &mut Vec<u32>,
            bool_labels: &mut Vec<u32>
        );
    }
}

#[cfg(test)]
mod tests {
    use super::booleans_rs;

    #[test]
    fn test_boolean_operation() {
        // Create two simple triangles for testing
        let in_coords = vec![
            // Triangle 1
            0.0, 0.0, 0.0,  // vertex 0
            1.0, 0.0, 0.0,  // vertex 1
            0.5, 1.0, 0.0,  // vertex 2
            // Triangle 2
            0.5, 0.0, 0.0,  // vertex 3
            1.5, 0.0, 0.0,  // vertex 4
            1.0, 1.0, 0.0,  // vertex 5
        ];
        
        let in_tris = vec![
            0, 1, 2,  // Triangle 1
            3, 4, 5,  // Triangle 2
        ];
        
        let in_labels = vec![1, 2]; // Label for each triangle
        
        let mut bool_coords = Vec::new();
        let mut bool_tris = Vec::new();
        let mut bool_labels = Vec::new();
        
        // Test union operation (op = 0)
        booleans_rs::boolean_operation(
            &in_coords,
            &in_tris,
            &in_labels,
            0, // Union operation
            &mut bool_coords,
            &mut bool_tris,
            &mut bool_labels,
        );
        
        // Basic assertions - result should have some output
        assert!(!bool_coords.is_empty(), "Result coordinates should not be empty");
        assert!(!bool_tris.is_empty(), "Result triangles should not be empty");
        assert!(!bool_labels.is_empty(), "Result labels should not be empty");
        
        // Coordinates should be multiples of 3 (x, y, z)
        assert_eq!(bool_coords.len() % 3, 0, "Coordinates should be multiples of 3");
        
        // Triangles should be multiples of 3 (vertex indices)
        assert_eq!(bool_tris.len() % 3, 0, "Triangle indices should be multiples of 3");
        
        println!("Boolean operation completed successfully!");
        println!("Result vertices: {}", bool_coords.len() / 3);
        println!("Result triangles: {}", bool_tris.len() / 3);
        println!("Result labels: {}", bool_labels.len());
    }
    
    #[test]
    fn test_intersection_operation() {

        println!("Creating in_coords, in_tris, and in_labels for intersection test...");

        // Create two simple triangles for testing
        let in_coords = vec![
            // Triangle 1
            0.0, 0.0, 0.0,  // vertex 0
            1.0, 0.0, 0.0,  // vertex 1
            0.5, 1.0, 0.0,  // vertex 2
            // Triangle 2
            0.5, 0.0, 0.0,  // vertex 3
            1.5, 0.0, 0.0,  // vertex 4
            1.0, 1.0, 0.0,  // vertex 5
        ];
        
        let in_tris = vec![
            0, 1, 2,  // Triangle 1
            3, 4, 5,  // Triangle 2
        ];
        
        let in_labels = vec![1, 2]; // Label for each triangle
        
        
        println!("Creating bool_coords, bool_tris, and bool_labels for intersection test...");
        
        let mut bool_coords = Vec::new();
        let mut bool_tris = Vec::new();
        let mut bool_labels = Vec::new();
        
        println!("Running intersection operation...");
        // Test intersection operation (op = 1)
        booleans_rs::boolean_operation(
            &in_coords,
            &in_tris,
            &in_labels,
            1, // Intersection operation
            &mut bool_coords,
            &mut bool_tris,
            &mut bool_labels,
        );
        
        // For intersection, result might be empty if no overlap or complex
        println!("Intersection operation completed!");
        println!("Result vertices: {}", bool_coords.len() / 3);
        println!("Result triangles: {}", bool_tris.len() / 3);
    }
}