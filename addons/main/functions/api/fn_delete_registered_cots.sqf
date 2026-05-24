// function name: armatak_fnc_delete_registered_cots
// function author: Valmo, Codex
// function description: Sends forced delete CoTs for all registered CoTs in a scope.
//
// Arguments:
// 0: Scope/key used to group CoTs <STRING>
//
// Return Value:
// Number of delete CoTs sent <NUMBER>
//
// Public: Yes

params [
	["_scope", "", [""]]
];

if (_scope isEqualTo "") exitWith {0};

private _registry = missionNamespace getVariable ["armatak_registered_cots", []];
private _remaining = [];
private _deleted = 0;

{
	_x params ["_registeredScope", "_uid", "_type", "_lat", "_lon", "_hae"];

	if (_registeredScope isEqualTo _scope) then {
		"armatak" callExtension ["tcp_socket:cot:delete", [[_uid, _type, _lat, _lon, _hae]]];
		_deleted = _deleted + 1;
	} else {
		_remaining pushBack _x;
	};
} forEach _registry;

missionNamespace setVariable ["armatak_registered_cots", _remaining];

_deleted
