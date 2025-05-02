#include "Ploting.hh"
#include <cstdlib>
#include <iostream>
#include <fstream>
#include <string>
#include <exception>
#include <filesystem>
#include <unistd.h>
#include <pwd.h>
#include <yaml-cpp/yaml.h>
#include <sstream>


namespace fs = std::filesystem;

using namespace std;

namespace cy {

    void generatePlotingEngine(const string& functionnameA,
                               const string& functionnameB,
                               const string& processname) {
        string dotFilename = processname + ".dot";
        string svgFilename = processname + ".svg";

        system("mkdir -p /tmp/cybercraft/");

        // Write safe DOT file
        ofstream dotFile(dotFilename);
        if (dotFile.is_open()) {
            dotFile << "digraph G {\n"
            << "  \"" << functionnameA << "\" -> \"" << functionnameB << "\";\n"
            << "}\n";
            dotFile.close();
        } else {
            cerr << "Error: Could not write DOT file." << endl;
            return;
        }

        // Now call Graphviz safely
        string command = "dot -Tsvg " + dotFilename + " -o /tmp/cybercraft/" + svgFilename;
        int result = system(command.c_str());
        if (result != 0) {
            cerr << "Error: Graphviz command failed." << endl;
        }
                               }

} // namespace cy
