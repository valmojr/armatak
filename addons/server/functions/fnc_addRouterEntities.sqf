#include "..\script_component.hpp"
/*
 * Author: Valmo, Codex
 * Adds entities to the server-side CoT router entity list.
 *
 * Arguments:
 * 0: Entity or entities to publish through the CoT router <OBJECT|ARRAY>
 *
 * Return Value:
 * Current routed entities <ARRAY>
 *
 * Public: Yes
 */

params [
	["_entities", [], [objNull, []]]
];

private _toAdd = if (_entities isEqualType objNull) then {[_entities]} else {_entities};
private _routed = missionNamespace getVariable ["armatak_server_syncedUnits", []];
_routed append _toAdd;
_routed = _routed arrayIntersect _routed;
_routed = _routed select {!isNull _x && {alive _x}};

missionNamespace setVariable ["armatak_server_syncedUnits", _routed, true];
GVAR(syncedUnits) = _routed;

_routed
