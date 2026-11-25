#ifndef FUSION_TIME_HPP
#define FUSION_TIME_HPP

#include <chrono>

namespace fusion {

// Time span or timestamp in seconds. The unit lives in the type: a bare double
// does not convert implicitly (compile error), other std::chrono durations
// convert with correct scaling (250ms -> 0.25 s). .count() yields the double.
using Seconds = std::chrono::duration<double>;

} // namespace fusion

#endif // FUSION_TIME_HPP
