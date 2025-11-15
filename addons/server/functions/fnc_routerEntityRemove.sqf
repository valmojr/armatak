#include "..\script_component.hpp"
/*
	 * Author: Valmo
	 * Removes a unit from the global marked units array.
	 *
	 * Arguments:
	 * 0: The module logic <OBJECT>
	 *
	 * Return Value:
	 * None
	 *
	 * Example:
	 * [LOGIC] call armatak_server_fnc_routerEntityRemove;
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
	default {
		{
			if (_x isEqualTo _unit) then {
				GVAR(syncedUnits) deleteAt _forEachIndex;
			};
		} forEach GVAR(syncedUnits);

		missionNmaespace setVariable ["armatak_server_syncedUnits", GVAR(syncedUnits)];
		SETVAR(_unit,GVAR(isRouting),false);

		deleteVehicle _logic;
	};
};
