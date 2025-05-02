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
#include "Checking.hh"
#include "Ploting.hh"

using namespace std;

namespace fs = std::filesystem;

using cy::quote;

const string VERSION = "0.5.71";




void printLogo() {
  string logo = "\n  ___________________________________________________________________\n        __                               __                   _      \n      /    )         /                 /    )               /  `     \n  ---/--------------/__----__---)__---/--------)__----__--_/__---_/_-\n    /        /   / /   ) /___) /   ) /        /   ) /   ) /      /   \n  _(____/___(___/_(___/_(___ _/_____(____/___/_____(___(_/______(_ __\n               /                                                     \n           (_ /                                                      ";
  cout << logo << endl;
}



void generate_gpg_key() {
  system("source ~/.cybercraft/cybercraft-venv/bin/activate; python3 ~/.cybercraft/gpggen; deactivate;");
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



void gpg_encrypt(const string& filename) {
  string key = cy::get_gpg_keyname();
  if (key != "nothing") {
    ostringstream cmd;

    cmd << "gpg --output " << cy::quote(filename + ".gpg") << " --encrypt --recipient " << cy::quote(key) << " " << cy::quote(filename);
    system(cmd.str().c_str());
    cout << "Encrypted: " << filename << ".gpg" << endl;
  }
}

void gpg_decrypt(const string& filename) {
  string out = cy::remove_extension(filename);
  ostringstream cmd;
  cmd << "gpg --output " << cy::quote(out) << " --decrypt " << cy::quote(filename);
  system(cmd.str().c_str());
}

void yubikey_encryption(bool folder, const string& value) {
  ostringstream cmd;
  fs::path home = cy::getHomeDirectory();
  if (folder) {
    cmd << "python3 " << cy::quote((home / "dcde/src/encrypt.py").string()) << " " << cy::quote(value);
  } else {
    cmd << cy::quote((home / ".cybercraft/shell/Folder-Anonymizer").string());

  }
  system(cmd.str().c_str());
}

void yubikey_decryption(bool folder, const string& value) {
  ostringstream cmd;
  fs::path home = cy::getHomeDirectory();
  if (folder) {
    cmd << "python3 " << cy::quote((home / "dcde/src/decrypt.py").string()) << " " << cy::quote(value);
  } else {
    cmd << cy::quote((home / ".cybercraft/shell/Folder-Anonymizer").string());
  }
  system(cmd.str().c_str());
}

int add(const string& filename) {
  string method = cy::get_encryption_methode();
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

void show_help(const string& cmd) {
  cerr << "Usage: " << cmd << " [\n  init \n   update \n    push \n     pull \n      --about \n       --newgpgkey Generate a new GPG Key.\n        --version \n         --encrypt <FILE> or <nothing for folder>\n          --decrypt <FILE> or <nothing for folder>\n]" << endl;
}



void update() {
  system("cd ~/CyberCraft && git pull && make && cd -");
}

void readPlots() {
  // /tmp/cybercraft/checkProject.svg
}

int main(int argc, char* argv[]) {
  if (argc < 2) {
    printLogo();
    show_help(argv[0]);
    return 1;
  }

  string cmd = argv[1];

  if (cmd == "--about") {
    cy::printMaskot();
  } else if (cmd == "--check") {
    cy::generatePlotingEngine("argv[1]: --check", "Security Check", "checkProject");
    system("mv checkProject.dot /tmp/cybercraft/checkProject.dot");
    cy::checkGitleaksIgnore();
    cy::check_cy_config();
    cy::checkPreCommitConfig();
  } else if (cmd == "--version") {
    cout << "Version: " << VERSION << endl;
  } else if (cmd == "--newgpgkey") {
    generate_gpg_key();
  } else if (cmd == "init") {
    cy::init();
  } else if (cmd == "update") {
    update();
  } else if (cmd == "push") {
    return cy::push();
  } else if (cmd == "pull") {
    return cy::pull();
  } else if (cmd == "--encrypt") {
    string method = cy::get_encryption_methode();
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
    string method = cy::get_encryption_methode();
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
