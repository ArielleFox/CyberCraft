#ifndef PLOTING_HH
#define PLOTING_HH

#include <string>
#include <filesystem>

namespace fs = std::filesystem;


namespace cy {
    void generatePlotingEngine(const std::string& functionnameA,
                               const std::string& functionnameB,
                               const std::string& processname);
}

#endif // PLOTING_HH
