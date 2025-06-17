#include "script_component.hpp"

if (!hasInterface) exitWith {};

_local_address = "armatak" callExtension ["local_ip", []] select 0;

SETVAR(player,GVAR(localAddress),_local_address);
SETVAR(player,GVAR(eudConnected),false);
