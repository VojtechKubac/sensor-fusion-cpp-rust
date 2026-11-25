#ifndef FUSION_REPLAY_CSV_READER_HPP
#define FUSION_REPLAY_CSV_READER_HPP

#include "fusion/replay/records.hpp"

#include <cstddef>
#include <expected>
#include <istream>
#include <vector>

// Readers for the replay CSVs (sim/SCHEMA.md): exact header row, then one
// sample per line, timestamps strictly increasing. Loading happens before the
// real-time loop, so returning std::vector (heap) is fine here.
namespace fusion::replay {

enum class ReplayErrorKind {
  HeaderMismatch,         // first line missing or not the schema's header
  WrongFieldCount,        // row has more or fewer comma-separated fields
  InvalidNumber,          // field is not a complete decimal number
  NonIncreasingTimestamp, // t <= previous row's t
};

struct ReplayError {
  ReplayErrorKind kind;
  std::size_t line; // 1-based; the header is line 1
};

[[nodiscard]] std::expected<std::vector<SensorSample>, ReplayError>
readSensorCsv(std::istream& input);

[[nodiscard]] std::expected<std::vector<TruthSample>, ReplayError>
readGroundTruthCsv(std::istream& input);

} // namespace fusion::replay

#endif // FUSION_REPLAY_CSV_READER_HPP
