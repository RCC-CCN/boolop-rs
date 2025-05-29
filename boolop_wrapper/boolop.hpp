#pragma once
#include <vector>
#include <bitset>
#include <cstdint>

void boolean_pipeline(const std::vector<double> &in_coords, const std::vector<uint> &in_tris,
    const std::vector<uint> &in_labels, int op, std::vector<double> &bool_coords,
    std::vector<uint> &bool_tris, std::vector< std::bitset<32> > &bool_labels);
