#include "..\..\script_component.hpp"

params["_group"];

_group_name = _group getVariable QGVAR(group_name);

if (isNil "_group_name") then {
	_group_colors = missionNamespace getVariable QGVAR(group_colors);
	_group_name = selectRandom _group_colors;

	if (count _group_colors > 0) then {
		_randomIndex = floor (random (count _group_colors));

		_selectedColor = _group_colors select _randomIndex;

		_group_colors deleteAt _randomIndex;

		_group_name = _selectedColor;
		_group setVariable [QGVAR(group_name), _group_name];
		missionNamespace setVariable [QGVAR(group_colors), _group_colors]
	} else {
		_group_name = "Red";
		_group setVariable [QGVAR(group_name), _group_name];
	};
};

_group_name
