// COMPONENT should be defined in the script_component.hpp and included BEFORE this hpp

#define MAINPREFIX armatak
#define PREFIX armatak

#include "script_version.hpp"

#define VERSION     MAJOR.MINOR
#define VERSION_STR MAJOR.MINOR.PATCHLVL.BUILD
#define VERSION_AR  MAJOR,MINOR,PATCHLVL,BUILD

#define PROJECT_AUTHOR QUOTE(Valmo Trindade)

// MINIMAL required version for the Mod. Components can specify others..
#define REQUIRED_VERSION 2.18
#define REQUIRED_CBA_VERSION {3,18,2}

#ifndef COMPONENT_BEAUTIFIED
    #define COMPONENT_BEAUTIFIED COMPONENT
#endif
#ifdef SUBCOMPONENT_BEAUTIFIED
    #define COMPONENT_NAME QUOTE(ARMATAK - COMPONENT_BEAUTIFIED - SUBCOMPONENT_BEAUTIFIED)
#else
    #define COMPONENT_NAME QUOTE(ARMATAK - COMPONENT_BEAUTIFIED)
#endif

// Custom ARMATAK MACRO defines

#define EXTENSION_NAME QUOTE(armatak)

#define CALLEXT(var) EXTENSION_NAME callExtension [QUOTE(var),[]]
#define CALLEXTP(var1, var2) EXTENSION_NAME callExtension [QUOTE(var1), var2] select 0