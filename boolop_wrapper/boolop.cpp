#include "boolop.hpp"
#include "InteractiveAndRobustMeshBooleans/code/booleans.h"


void boolean_pipeline(const std::vector<double> &in_coords, const std::vector<uint> &in_tris,
    const std::vector<uint> &in_labels, int op, std::vector<double> &bool_coords,
    std::vector<uint> &bool_tris, std::vector< std::bitset<32> > &bool_labels)
{
    booleanPipeline(in_coords, in_tris,
        in_labels, (BoolOp)op, bool_coords,
        bool_tris,bool_labels);
}