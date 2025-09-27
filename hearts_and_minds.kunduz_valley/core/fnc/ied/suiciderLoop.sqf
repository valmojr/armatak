
/* ----------------------------------------------------------------------------
Function: btc_ied_fnc_suiciderLoop

Description:
    Search for soldier around and when found, activate the suicider.

Parameters:
    _suicider - Suicider created. [Object]

Returns:

Examples:
    (begin example)
        [_suicider] call btc_ied_fnc_suiciderLoop;
    (end)

Author:
    Giallustio

---------------------------------------------------------------------------- */

[{
    params ["_suicider"];

    if (alive _suicider && !isNull _suicider) then {
        private _array = _suicider nearEntities [btc_player_type, 25];
        _array = _array select {side group _x isEqualTo btc_player_side && {!captive _x}};
        if (_array isNotEqualTo []) exitWith {
            _suicider call btc_ied_fnc_suicider_active;
        };
        _this call btc_ied_fnc_suiciderLoop;
    };
}, _this, 5] call CBA_fnc_waitAndExecute;
