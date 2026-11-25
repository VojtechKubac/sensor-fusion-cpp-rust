#ifndef FUSION_REPLAY_RECORDS_HPP
#define FUSION_REPLAY_RECORDS_HPP

#include "fusion/kf/types.hpp"
#include "fusion/time.hpp"

// One row of each replay file (sim/SCHEMA.md). Vector element order follows
// the schema's state_layout / measurement_layout.
namespace fusion::replay {

// sensor.csv: t, zx, zy.
struct SensorSample {
  Seconds timestamp;
  kf::MeasurementVector measurement;
};

// ground_truth.csv: t, x, y, vx, vy.
struct TruthSample {
  Seconds timestamp;
  kf::StateVector state;
};

} // namespace fusion::replay

#endif // FUSION_REPLAY_RECORDS_HPP
