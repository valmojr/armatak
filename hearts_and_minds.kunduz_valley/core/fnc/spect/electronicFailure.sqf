
/* ----------------------------------------------------------------------------
Function: btc_spect_fnc_electronicFailure

Description:
    Apply electronic failure.

Parameters:
    _vehicles - Array of vehicles. [Array]

Returns:

Examples:
    (begin example)
        [[vehicle (allPlayers select 0)]] call btc_spect_fnc_electronicFailure;
    (end)

Author:
    Vdauphin

---------------------------------------------------------------------------- */

params [
    ["_vehicles", [], [[]]]
];

{
    private _veh = _x;
    if (isEngineOn _x && selectRandom [false, true]) then {
        [_veh, false] remoteExecCall ["engineOn", _veh];
    };
    if (isLightOn _veh && selectRandom [false, true]) then {
        [_veh, false] remoteExecCall ["setPilotLight", _veh];
    };
    if (isCollisionLightOn _veh && selectRandom [false, true]) then {
        [_veh, false] remoteExecCall ["setCollisionLight", _veh];
    };
    {
        if (
            "avionics" in _x || {"missiles" in _x} || {"svetlo" in _x} || {"battery" in _x} || {"cam" in _x} || {"com" in _x}
        ) then {
            if (_veh getHitIndex _forEachIndex < 1 && selectRandom [false, true]) then {
                [_veh, [_forEachIndex, 1]] remoteExecCall ["setHitIndex", _veh];
                break;
            };
        };
    } foreach (getAllHitPointsDamage _veh select 0);
} forEach _vehicles;
