params["_unit"];

private _result = "armatak" callExtension ["ManageGeoObject/putGeoObject",[_unit] call armatak_fnc_extract_info];

private _uid_string = _result select 0;

if (_uid_string != "") then {
	_uid_string = (_uid_string splitString '"') select 3;
	_unit setVariable ["_atak_uid",_uid_string, true];
} else {
	_uid_string = "ERROR: " + _result joinString " | ";
};

_uid_string