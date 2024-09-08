params["_unit"];

_uuid = _unit getVariable "_atak_uid";

if (isNil "_uuid") then {
	_uuid = "armatak" callExtension "uuid";
	_unit setVariable ["_atak_uid", _uuid];
};

_uuid