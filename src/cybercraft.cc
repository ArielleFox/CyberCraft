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

const string VERSION = "0.5.7-python3.14t";

// Helper for safely building shell commands
string quote(const string& str) {
  return "\"" + str + "\"";
}

fs::path getHomeDirectory() {
  const char* home = getenv("HOME");
  if (home) return fs::path(home);
 struct passwd *pw = getpwuid(getuid());
  if (pw) return fs::path(pw->pw_dir);
  throw runtime_error("Unable to determine home directory");
}

void printLogo() {
  string logo = "\n.oPYo.        8                   .oPYo.               d'b   o  \n8    8        8                   8    8               8     8  \n8      .o    o 8oPYo. .oPYo. oPYo. 8      .oPYo. .oPYo. o8P   o8P \n8      8    8 8    8 8oooo8 8  `' 8      8  `' .oooo8  8     8  \n8    8 8    8 8    8 8.     8     8    8 8     8    8  8     8  \n`YooP' `YooP8 `YooP' `Yooo' 8     `YooP' 8     `YooP8  8     8  \n:.....::....8 :.....::.....:..:::::.....:..:::::.....::..::::..:\n:::::::::ooP'.::::::::::::::::::::::::::::::::::::::::::::::::::\n:::::::::...::::::::::::::::::::::::::::::::::::::::::::::::::::";
  cout << logo << endl;
}

void printMaskot() {
  fs::path mascotPath = getHomeDirectory() / ".cybercraft/art/mascot";
  ifstream file(mascotPath);
  if (file) {
    cout << file.rdbuf();
  } else {
    cerr << "Mascot not found: " << mascotPath << endl;
  }
}

void generate_gpg_key() {
    system("source ~/.cybercraft/cybercraft-venv/bin/activate; python3 ~/.cybercraft/gpggen; deactivate;");
}


string configfile_value(const string& config_value) {
  try {
    YAML::Node config = YAML::LoadFile(".cy_config.yaml");
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

string get_gpg_keyname() {
  return configfile_value("keyname");
}

string get_encryption_methode() {
  return configfile_value("methode");
}

void commit_message() {
  string message;
  cout << "Enter a commit message: ";
  getline(cin, message);

  if (!message.empty()) {
    ofstream temp(".commitmsg.tmp");
    if (temp) {
      temp << message << endl;
      temp.close();
      system("git commit -F .commitmsg.tmp");
      fs::remove(".commitmsg.tmp");
    } else {
      cerr << "Unable to write commit message to temp file." << endl;
    }
  }
}

string remove_extension(const string& filename) {
  size_t pos = filename.rfind('.');
  return (pos != string::npos) ? filename.substr(0, pos) : filename;
}

void gpg_encrypt(const string& filename) {
  string key = get_gpg_keyname();
  if (key != "nothing") {
    ostringstream cmd;
    cmd << "gpg --output " << quote(filename + ".gpg")
    << " --encrypt --recipient " << quote(key)
    << " " << quote(filename);
    system(cmd.str().c_str());
    cout << "Encrypted: " << filename << ".gpg" << endl;
  }
}

void gpg_decrypt(const string& filename) {
  string out = remove_extension(filename);
  ostringstream cmd;
  cmd << "gpg --output " << quote(out) << " --decrypt " << quote(filename);
  system(cmd.str().c_str());
}

void yubikey_encryption(bool folder, const string& value) {
  ostringstream cmd;
  fs::path home = getHomeDirectory();
  if (folder) {
    cmd << "python3.14t " << quote((home / "dcde/src/encrypt.py").string()) << " " << quote(value);
  } else {
    cmd << quote((home / ".cybercraft/shell/Folder-Anonymizer").string());
  }
  system(cmd.str().c_str());
}

void yubikey_decryption(bool folder, const string& value) {
  ostringstream cmd;
  fs::path home = getHomeDirectory();
  if (folder) {
    cmd << "python3.14t " << quote((home / "dcde/src/decrypt.py").string()) << " " << quote(value);
  } else {
    cmd << quote((home / ".cybercraft/shell/Folder-Anonymizer").string());
  }
  system(cmd.str().c_str());
}

int add(const string& filename) {
  string method = get_encryption_methode();
  if (method == "yubikey") {
    yubikey_encryption(true, filename);
    system(("git add " + filename + ".age").c_str());
  } else if (method == "gpg") {
    gpg_encrypt(filename);
    system(("git add " + filename + ".gpg").c_str());
  } else {
    return 1;
  }
  commit_message();
  return 0;
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

void check_cy_config() {
  if (!fs::exists(".cy_config.yaml")) {
    cerr << "[Missing] .cy_config.yaml\n[Init] Creating default..." << endl;
    ofstream file(".cy_config.yaml");
    file << "methode: none\n";
    file.close();
  } else {
    string method = get_encryption_methode();
    if (method == "gpg" && get_gpg_keyname() == "nothing") {
      cerr << "Missing GPG keyname, running keygen..." << endl;
      system("source ~/.cybercraft/cybercraft-venv/bin/activate; python3 ~/.cybercraft/gpggen; deactivate;");
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

void show_help(const string& cmd) {
  cerr << "Usage: " << cmd << " [\n    init \n    update \n    push \n     pull \n     --about \n      --newgpgkey Generate a new GPG Key.\n      --version \n    --encrypt <FILE> or <nothing for folder>\n    --decrypt <FILE> or <nothing for folder>\n]" << endl;
}

void generatePlotingEngine(const string& functionnameA, const string& functionnameB, const string& processname) {
  string dotFilename = processname + ".dot";
  string svgFilename = processname + ".svg";

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

void checkGitleaksIgnore() {
  if (fs::exists(".gitleaksignore")) {
    cout << "[Warning] .gitleaksignore EXISTS.\n[Action] Removing for safety..." << endl;
    system("git rm -f .gitleaksignore");
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


void update() {
  system("cd ~/CyberCraft && git pull && make && cd -");
}


int main(int argc, char* argv[]) {
  if (argc < 2) {
    printLogo();
    show_help(argv[0]);
    return 1;
  }

  string cmd = argv[1];

   if  (cmd == "--about") {
    printMaskot();
  } else if (cmd == "--check") {
    // generatePlotingEngine("main", "--check", "checkProject");
     checkGitleaksIgnore();
     check_cy_config();
     checkPreCommitConfig();
  } else if (cmd == "--version") {
    cout << "Version: " << VERSION << endl;
  } else if (cmd == "--newgpgkey") {
    generate_gpg_key();
  } else if (cmd == "init") {
    init();
  } else if (cmd == "update") {
    update();
  } else if (cmd == "push") {
    return push();
  } else if (cmd == "pull") {
    return pull();
  } else if (cmd == "--encrypt") {
    string method = get_encryption_methode();
    if (argc >= 3) {
      string target = argv[2];
      if (method == "gpg") {
        gpg_encrypt(target);
      } else if (method == "yubikey") {
         yubikey_encryption(true, target);  // file/folder provided
      }
    } else {
      if (method == "gpg") {
        cout << "[Notice] Folder encryption for GPG not implemented." << endl;
      } else if (method == "yubikey") {
        yubikey_encryption(false, "");  // folder-based encryption
      }
    }
  } else if (cmd == "--decrypt") {
    string method = get_encryption_methode();
    if (argc >= 3) {
      string target = argv[2];
      if (method == "gpg") {
        gpg_decrypt(target);
      } else if (method == "yubikey") {
        yubikey_decryption(true, target);  // file/folder provided
      }
    } else {
      if (method == "gpg") {
        cout << "[Notice] Folder decryption for GPG not implemented." << endl;
      } else if (method == "yubikey") {
        yubikey_decryption(false, "");  // folder-based decryption
            }
      }
    }


  return 0;
}
