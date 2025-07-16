#include "..\script_component.hpp"
/*
	 * Author: Valmo
	 * Adds a unit into the global marked units array.
	 *
	 * Arguments:
	 * 0: The module logic <OBJECT>
	 *
	 * Return Value:
	 * None
	 *
	 * Example:
	 * [LOGIC] call armatak_server_fnc_routerEntityAdd;
	 *
	 * Public: No
 */

params ["_logic"];

if (!local _logic) exitWith {};

private _unit = attachedTo _logic;

switch (false) do {
	case (!isNull _unit): {
		["Nothing selected", "error", "TCP Socket"] call EFUNC(main,notify);
		deleteVehicle _logic;
	};
	default {
		if (_unit in (missionNamespace getVariable ["armatak_marked_units", []])) exitWith {
			["Unit already marked", "warning", "TCP Socket"] call EFUNC(main,notify);
			deleteVehicle _logic;
		};

		GVAR(syncedUnits) = missionNamespace getVariable "armatak_marked_units";

		GVAR(syncedUnits) pushBack _unit;

		missionNamespace setVariable ["armatak_marked_units", GVAR(syncedUnits)];
		SETVAR(_unit,GVAR(isRouting),true);

		deleteVehicle _logic;
	};
};
