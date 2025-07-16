params ["_fullName"];

private _nameParts = _fullName splitString " ";
private _nameCount = count _nameParts;

if (_nameCount == 1) then {
	_fullName
} else {
	private _firstName = _nameParts select 0;
	private _lastName = _nameParts select (_nameCount - 1);

	if ((count _lastName) >= 7) then {
		_lastName
	} else {
		format ["%1 %2", _firstName select [0, 1], _lastName]
	};
};
