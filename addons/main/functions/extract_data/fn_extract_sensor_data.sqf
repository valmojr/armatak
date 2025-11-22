params["_unit"];

_target = getSensorTargets (_unit);

{
  _unit = _x select 0;
  _position = _x select 1;
  _status = _x select 2;

  if (isNil {_unit getVariable "armatak_current_side"}) then {
      _unit setVariable ["armatak_current_side", side _unit];
  };

  if (_status != "destroyed") then {
	  _unit call armatak_fnc_send_enemy_cot;
  };
} forEach _target;
