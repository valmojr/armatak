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
deleteVehicle _logic;

switch (false) do {
	case (!isNull _unit): {
		deleteVehicle _logic;
		["Nothing selected", "error", "TCP Socket"] call EFUNC(main,notify);
	};
	case (!isNull (units _unit)): {
		GVAR(syncedUnits) = missionNamespace getVariable "armatak_marked_units";

		{
			GVAR(syncedUnits) pushBack _unit;
		} forEach (units _unit);

		missionNamespace setVariable ["armatak_marked_units", GVAR(syncedUnits)];

		deleteVehicle _logic;
	};
	default {
		GVAR(syncedUnits) = missionNamespace getVariable "armatak_marked_units";

		GVAR(syncedUnits) pushBack _unit;

		missionNamespace setVariable ["armatak_marked_units", GVAR(syncedUnits)];

		deleteVehicle _logic;
	};
};