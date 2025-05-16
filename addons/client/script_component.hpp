#define COMPONENT client
#define COMPONENT_BEAUTIFIED WebSocket Client
#include "\armatak\armatak\addons\main\script_mod.hpp"

// #define DEBUG_MODE_FULL
// #define DISABLE_COMPILE_CACHE
// #define ENABLE_PERFORMANCE_COUNTERS

#ifdef DEBUG_ENABLED_CLIENT
    #define DEBUG_MODE_FULL
#endif

#ifdef DEBUG_SETTINGS_CLIENT
    #define DEBUG_SETTINGS DEBUG_SETTINGS_CLIENT
#endif

#include "\z\ace\addons\main\script_macros.hpp"
