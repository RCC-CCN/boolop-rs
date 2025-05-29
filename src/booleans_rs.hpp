#pragma once
#include <vector>
#include <cstdint>
#include <bitset>
#include "rust/cxx.h"

void boolean_operation(const rust::Vec<double> &in_coords, const rust::Vec<uint32_t> &in_tris,
    const rust::Vec<uint32_t> &in_labels, int32_t op, rust::Vec<double> &bool_coords,
    rust::Vec<uint32_t> &bool_tris, rust::Vec<uint32_t> &bool_labels);

std::vector<std::bitset<32>> uints_to_bitsets(const std::vector<uint32_t>& src);