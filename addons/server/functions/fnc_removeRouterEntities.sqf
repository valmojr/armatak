#include "..\script_component.hpp"
/*
 * Author: Valmo, Codex
 * Removes entities from the server-side CoT router entity list.
 *
 * Arguments:
 * 0: Entity or entities to remove from the CoT router <OBJECT|ARRAY>
 *
 * Return Value:
 * Current routed entities <ARRAY>
 *
 * Public: Yes
 */

params [
	["_entities", [], [objNull, []]]
];

private _toRemove = if (_entities isEqualType objNull) then {[_entities]} else {_entities};
private _routed = missionNamespace getVariable ["armatak_server_syncedUnits", []];
_routed = (_routed - _toRemove) select {!isNull _x && {alive _x}};

missionNamespace setVariable ["armatak_server_syncedUnits", _routed, true];
GVAR(syncedUnits) = _routed;

_routed
