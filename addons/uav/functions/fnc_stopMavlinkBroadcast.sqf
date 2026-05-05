#include "..\script_component.hpp"

if (!hasInterface) exitWith {};

private _existingPfh = player getVariable [QGVAR(mavlinkPFH), -1];
if (_existingPfh >= 0) then {
	[_existingPfh] call CBA_fnc_removePerFrameHandler;
	player setVariable [QGVAR(mavlinkPFH), -1];
};

private _broadcastingUav = player getVariable [QGVAR(broadcastingUav), objNull];
if (!isNull _broadcastingUav) then {
	systemChat "UAV broadcasting stopped";
};

player setVariable [QGVAR(broadcastingUav), objNull];
