#include "booleans_rs.hpp"
#include "../boolop_wrapper/boolop.hpp"
#include <stdexcept>
#include <iostream>

void boolean_operation(const rust::Vec<double> &in_coords, const rust::Vec<uint32_t> &in_tris,
    const rust::Vec<uint32_t> &in_labels, int32_t op, rust::Vec<double> &bool_coords,
    rust::Vec<uint32_t> &bool_tris, rust::Vec<uint32_t> &bool_labels)
{
    try {
        // Validate input parameters
        if (in_coords.empty() || in_tris.empty()) {
            std::cerr << "Error: Empty input data" << std::endl;
            return;
        }
        
        // Check coordinate count is multiple of 3 (x,y,z)
        if (in_coords.size() % 3 != 0) {
            std::cerr << "Error: Coordinate count must be multiple of 3" << std::endl;
            return;
        }
        
        // Check triangle indices are valid
        if (in_tris.size() % 3 != 0) {
            std::cerr << "Error: Triangle indices count must be multiple of 3" << std::endl;
            return;
        }
        
        // Clear output vectors first to ensure clean state
        bool_coords.clear();
        bool_tris.clear();
        bool_labels.clear();
        
        // Convert rust::Vec to std::vector with explicit size checking
        std::vector<double> std_coords;
        std_coords.reserve(in_coords.size());
        for (size_t i = 0; i < in_coords.size(); ++i) {
            std_coords.push_back(in_coords[i]);
        }
        
        std::vector<uint32_t> std_tris;
        std_tris.reserve(in_tris.size());
        for (size_t i = 0; i < in_tris.size(); ++i) {
            std_tris.push_back(in_tris[i]);
        }
        
        std::vector<uint32_t> std_labels;
        std_labels.reserve(in_labels.size());
        for (size_t i = 0; i < in_labels.size(); ++i) {
            std_labels.push_back(in_labels[i]);
        }
        
        // Initialize output vectors
        std::vector<double> std_bool_coords;
        std::vector<uint32_t> std_bool_tris;
        std::vector<std::bitset<32>> bitset_labels;

        std::cout << "Running boolean_pipeline" << std::endl;
        
        // Call the boolean operation with exception handling
        boolean_pipeline(std_coords, std_tris, std_labels, op, std_bool_coords, std_bool_tris, bitset_labels);

        std::cout << "Done Running boolean_pipeline" << std::endl;

        
        // Convert results back to rust::Vec with bounds checking
        if (!std_bool_coords.empty()) {
            bool_coords.reserve(std_bool_coords.size());
            for (size_t i = 0; i < std_bool_coords.size(); ++i) {
                bool_coords.push_back(std_bool_coords[i]);
            }
        }
        
        if (!std_bool_tris.empty()) {
            bool_tris.reserve(std_bool_tris.size());
            for (size_t i = 0; i < std_bool_tris.size(); ++i) {
                bool_tris.push_back(std_bool_tris[i]);
            }
        }
        
        if (!bitset_labels.empty()) {
            bool_labels.reserve(bitset_labels.size());
            for (size_t i = 0; i < bitset_labels.size(); ++i) {
                // Safely convert bitset to uint32_t
                auto bits = bitset_labels[i];
                bool_labels.push_back(static_cast<uint32_t>(bits.to_ulong()));
            }
        }
        
    } catch (const std::exception& e) {
        std::cerr << "Exception in boolean_operation: " << e.what() << std::endl;
        // Ensure output vectors are in a valid state
        bool_coords.clear();
        bool_tris.clear();
        bool_labels.clear();
    } catch (...) {
        std::cerr << "Unknown exception in boolean_operation" << std::endl;
        // Ensure output vectors are in a valid state
        bool_coords.clear();
        bool_tris.clear();
        bool_labels.clear();
    }
}

std::vector<std::bitset<32>> uints_to_bitsets(const std::vector<uint32_t>& src) {
    std::vector<std::bitset<32>> dst;
    if (src.empty()) {
        return dst;
    }
    
    try {
        dst.reserve(src.size());
        for (size_t i = 0; i < src.size(); ++i) {
            dst.emplace_back(src[i]);
        }
    } catch (const std::exception& e) {
        std::cerr << "Exception in uints_to_bitsets: " << e.what() << std::endl;
        dst.clear();
    }
    
    return dst;
}