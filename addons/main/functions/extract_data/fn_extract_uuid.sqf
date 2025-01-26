// function name: armatak_fnc_extract_uuid
// function author: Valmo
// function description: Defines a random v4 uuid if the doesn't have one already

params["_unit"];

_uuid = _unit getVariable "_atak_uid";

if (isNil "_uuid") then {
	_uuid = "armatak" callExtension ["uuid", []] select 0;
	_unit setVariable ["_atak_uid", _uuid];
};

_uuid