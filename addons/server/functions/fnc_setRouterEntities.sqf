#include "..\script_component.hpp"
/*
 * Author: Valmo, Codex
 * Replaces the server-side CoT router entity list with a de-duplicated list.
 *
 * Arguments:
 * 0: Entities to publish through the CoT router <ARRAY>
 *
 * Return Value:
 * Current routed entities <ARRAY>
 *
 * Public: Yes
 */

params [
	["_entities", [], [[]]]
];

private _routed = _entities arrayIntersect _entities;
_routed = _routed select {!isNull _x && {alive _x}};

missionNamespace setVariable ["armatak_server_syncedUnits", _routed, true];
GVAR(syncedUnits) = _routed;

_routed
