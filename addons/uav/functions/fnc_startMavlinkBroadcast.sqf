#include "..\script_component.hpp"

if (!hasInterface) exitWith {};

private _existingPfh = player getVariable [QGVAR(mavlinkPFH), -1];
if (_existingPfh >= 0) then {
	[_existingPfh] call CBA_fnc_removePerFrameHandler;
};

player setVariable [QGVAR(broadcastingUav), objNull];

private _pfh = [{
	call FUNC(updateMavlinkBroadcast);
}, 0.5, []] call CBA_fnc_addPerFrameHandler;

player setVariable [QGVAR(mavlinkPFH), _pfh];
