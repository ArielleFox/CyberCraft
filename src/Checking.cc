#include "Checking.hh"
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

    const string VERSION = "0.5.8";

    fs::path getHomeDirectory() {
        const char* home = getenv("HOME");
        if (home) return fs::path(home);
        struct passwd *pw = getpwuid(getuid());
        if (pw) return fs::path(pw->pw_dir);
        throw runtime_error("Unable to determine home directory");
    }

    string quote(const std::string& str) {
        return "\"" + str + "\"";
    }

    string remove_extension(const string& filename) {
        size_t pos = filename.rfind('.');
        return (pos != string::npos) ? filename.substr(0, pos) : filename;
    }

    void printMaskot() {
        fs::path mascotPath = cy::getHomeDirectory() / ".cybercraft/art/mascot";
        ifstream file(mascotPath);
        if (file) {
            cout << file.rdbuf();
        } else {
            cerr << "Mascot not found: " << mascotPath << endl;
        }
    }

    string get_gpg_keyname() {
        return configfile_value("keyname");
    }

    string get_encryption_methode() {
        return configfile_value("methode");
    }

    string configfile_value(const string& config_value) {
        try {
            YAML::Node config;
            if (fs::exists(".git/config")) {
              config = YAML::LoadFile(".cy_config.yaml");
            } else if (fs::exists("../.git/config")) {
              config = YAML::LoadFile("../.cy_config.yaml");
            } else if (fs::exists("../../.git/config")) {
              config = YAML::LoadFile("../../.cy_config.yaml");
            } else if (fs::exists("../../../.git/config")) {
              config = YAML::LoadFile("../../../.cy_config.yaml");
            }
            if (!config[config_value]) {
                cerr << "Config value '" << config_value << "' not found" << endl;
                return "nothing";
            }
            string val = config[config_value].as<string>();
            cout << config_value << " ==> " << val << endl;
            return val;
        } catch (const exception& e) {
            cerr << "Error reading config: " << e.what() << endl;
            return "nothing";
        }
    }

    void check_cy_config() {
        if (!fs::exists(".cy_config.yaml")) {
            if (fs::exists(".git/config")) {
                cerr << "[Missing] .cy_config.yaml\n[Init] Creating default..." << endl;
                ofstream file(".cy_config.yaml");
                file << "methode: none\n";
                file.close();
            } if (fs::exists("../.git/config")) {
                cerr << "[Missing] ../.cy_config.yaml\n[Init] Creating default..." << endl;
                ofstream file("../.cy_config.yaml");
                file << "methode: none\n";
                file.close();
            } if (fs::exists("../../.git/config")) {
                cerr << "[Missing] ../../.cy_config.yaml\n[Init] Creating default..." << endl;
                ofstream file("../../.cy_config.yaml");
                file << "methode: none\n";
                file.close();
            } if (fs::exists("../../../.git/config")) {
                cerr << "[Missing] ../../../.cy_config.yaml\n[Init] Creating default..." << endl;
                ofstream file("../../../.cy_config.yaml");
                file << "methode: none\n";
                file.close();
            }

        } else {
            string method = get_encryption_methode();
            if (method == "gpg" && get_gpg_keyname() == "nothing") {
                cerr << "Missing GPG keyname, running keygen..." << endl;
                system("source ~/.cybercraft/cybercraft-venv/bin/activate; python3 ~/.cybercraft/gpggen; deactivate;");
            }
        }
    }

    void checkGitleaksIgnore() {
        if (fs::exists(".gitleaksignore")) {
            cout << "[Warning] .gitleaksignore EXISTS.\n[Action] Removing for safety..." << endl;
            system("git rm -f .gitleaksignore");
            cout << "Use .gitignore instead." << endl;
        } else if (fs::exists("../.gitleaksignore")) {
            cout << "[Warning] ../.gitleaksignore EXISTS.\n[Action] Removing for safety..." << endl;
            system("git rm -f ../.gitleaksignore");
            cout << "Use .gitignore instead." << endl;
        } else if (fs::exists("../../.gitleaksignore")) {
            cout << "[Warning] ../../.gitleaksignore EXISTS.\n[Action] Removing for safety..." << endl;
            system("git rm -f ../../.gitleaksignore");
            cout << "Use .gitignore instead." << endl;
        } else {
            cout << "[Secure] .gitleaksignore not found." << endl;
        }
    }

    void checkPreCommitConfig() {
        if (fs::exists(".pre-commit-config.yaml")) {
            cout << "[Found] .pre-commit-config.yaml" << endl;
            system("pre-commit install");
            system("pre-commit autoupdate");
            system("gitleaks detect --source .");
        } else if (fs::exists("../.pre-commit-config.yaml")) {
            cout << "[Found] ../.pre-commit-config.yaml" << endl;
            system("pre-commit install");
            system("pre-commit autoupdate");
            system("gitleaks detect --source .");
        } else if (fs::exists("../../.pre-commit-config.yaml")) {
            cout << "[Found] ../../.pre-commit-config.yaml" << endl;
            system("pre-commit install");
            system("pre-commit autoupdate");
            system("gitleaks detect --source .");
        } else if (fs::exists("../../../.pre-commit-config.yaml")) {
            cout << "[Found] ../../../.pre-commit-config.yaml" << endl;
            system("pre-commit install");
            system("pre-commit autoupdate");
            system("gitleaks detect --source .");
        } else {
            fs::path src = getHomeDirectory() / ".cybercraft/.pre-commit-config.yaml";
            fs::path dst = ".pre-commit-config.yaml";
            if (fs::exists(src)) {
                fs::copy_file(src, dst, fs::copy_options::overwrite_existing);
                cout << "[Init] .pre-commit-config.yaml copied." << endl;
            } else {
                cerr << "Missing template .pre-commit-config.yaml" << endl;
            }
        }
    }

    void init() {
        if (!fs::exists(".gitignore")) {
            fs::path src = getHomeDirectory() / ".cybercraft/.gitignore";
            fs::path dst = ".gitignore";
            if (fs::exists(src)) fs::copy_file(src, dst, fs::copy_options::overwrite_existing);
        }
        check_cy_config();
    }

    int pull() {
        string method = get_encryption_methode();
        if (method == "gpg") return system("git pull");
        if (method == "yubikey") {
            int res = system("git pull");
            if (res == 0) system("cybercraft --decrypt");
            return res;
        }
        return system("git pull");
    }

    int push() {
        string method = get_encryption_methode();
        if (method == "gpg") return system("git push");
        if (method == "yubikey") {
            system("cybercraft --encrypt; git add .");
            system(("echo " + VERSION + " > .commitid").c_str());
            system("git commit -F .commitid");
            fs::remove(".commitid");
            return system("git push");
        }
        return system("git push");
    }
} // namespace cy
