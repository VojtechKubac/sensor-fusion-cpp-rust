#include <cstdlib>
#include <iostream>
#include <string_view>

// Placeholder until feature loop 1 (linear KF + SPSC queue + latency probes).
int main(int argc, char** argv) {
  for (int i = 1; i < argc; ++i) {
    const std::string_view arg = argv[i];
    if (arg == "-h" || arg == "--help") {
      std::cout << "fusion_cpp — C++ sensor-fusion pipeline (stub)\n"
                << "Usage: fusion_cpp [--help]\n";
      return EXIT_SUCCESS;
    }
  }
  std::cout << "fusion_cpp stub — awaiting linear KF baseline\n";
  return EXIT_SUCCESS;
}
