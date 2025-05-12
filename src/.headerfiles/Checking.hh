#ifndef CHECKING_HH
#define CHECKING_HH

#include <string>
#include <filesystem>

namespace fs = std::filesystem;

namespace cy {

    fs::path getHomeDirectory();
    std::string quote(const std::string& str);
    void printMaskot();
    std::string get_gpg_keyname();
    std::string get_encryption_methode();
    std::string remove_extension(const std::string& filename);
    std::string configfile_value(const std::string& config_value);
    void check_cy_config();
    void checkGitleaksIgnore();
    void checkPreCommitConfig();
    void init();
    int pull();
    int push();

} // namespace cy

#endif // CHECKING_HH
