// function name: armatak_fnc_register_cot
// function author: Valmo, Codex
// function description: Registers a CoT object under a scope so it can be deleted later.
//
// Arguments:
// 0: Scope/key used to group CoTs <STRING>
// 1: CoT UID <STRING>
// 2: CoT type <STRING>
// 3: Latitude <NUMBER>
// 4: Longitude <NUMBER>
// 5: HAE altitude <NUMBER>
//
// Public: Yes

params [
	["_scope", "", [""]],
	["_uid", "", [""]],
	["_type", "", [""]],
	["_lat", 0, [0]],
	["_lon", 0, [0]],
	["_hae", 0, [0]]
];

if (_scope isEqualTo "" || {_uid isEqualTo ""} || {_type isEqualTo ""}) exitWith {false};

private _registry = missionNamespace getVariable ["armatak_registered_cots", []];
_registry pushBack [_scope, _uid, _type, _lat, _lon, _hae];
missionNamespace setVariable ["armatak_registered_cots", _registry];

true
