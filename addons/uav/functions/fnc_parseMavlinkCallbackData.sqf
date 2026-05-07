#include "..\script_component.hpp"

params [["_raw", "", [""]]];

private _pairs = createHashMap;

{
	private _entry = _x;
	private _separatorIndex = _entry find "=";
	if (_separatorIndex > 0) then {
		private _key = _entry select [0, _separatorIndex];
		private _value = _entry select [_separatorIndex + 1];
		_pairs set [_key, _value];
	};
} forEach (_raw splitString ";");

_pairs
