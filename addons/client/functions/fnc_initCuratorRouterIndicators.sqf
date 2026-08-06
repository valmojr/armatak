#include "..\script_component.hpp"
/*
 * Author: Codex
 * Draws a Zeus-only visual border over entities routed by the ArmaTAK CoT Router.
 *
 * Arguments:
 * None
 *
 * Return Value:
 * Draw3D event handler id <NUMBER>
 *
 * Public: No
 */

if (!hasInterface) exitWith {-1};

private _existingEh = missionNamespace getVariable [QGVAR(curatorRouterIndicatorEh), -1];
if (_existingEh >= 0) exitWith {_existingEh};

private _eh = addMissionEventHandler ["Draw3D", {
	if (isNull findDisplay 312) exitWith {};

	private _curator = getAssignedCuratorLogic player;
	if (isNull _curator) exitWith {};

	private _routed = missionNamespace getVariable ["armatak_server_syncedUnits", []];
	if (_routed isEqualTo []) exitWith {};

	private _editableObjects = curatorEditableObjects _curator;
	private _texture = "\a3\ui_f\data\IGUI\Cfg\Cursors\selected_ca.paa";
	private _color = [0, 0.85, 1, 0.95];
	private _camera = curatorCamera;

	{
		if (!isNull _x && {alive _x} && {_x in _editableObjects}) then {
			private _pos = ASLToAGL (getPosASLVisual _x);
			_pos set [2, (_pos select 2) + 0.12];

			private _distance = _camera distance _x;
			private _size = linearConversion [0, 2000, _distance, 0.8, 1.35, true];
			private _textSize = linearConversion [0, 2000, _distance, 0.032, 0.05, true];
			drawIcon3D [_texture, _color, _pos, _size, _size, 0, "TAK", 1, _textSize, "RobotoCondensed", "center"];
		};
	} forEach _routed;
}];

missionNamespace setVariable [QGVAR(curatorRouterIndicatorEh), _eh];

_eh
