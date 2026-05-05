params ["_drone"];

private _override = _drone getVariable ["armatak_uas_camera_data_override", []];
private _isLocalController = hasInterface && {!isNull player} && {(getConnectedUAV player) isEqualTo _drone};

if (!_isLocalController && {_override isEqualType []} && {(count _override) >= 7}) then {
    private _updatedAt = _override param [6, -1000];
    if ((time - _updatedAt) <= 5) exitWith {
        private _overrideSpiAsl = _override param [4, []];
        private _overrideSpiGeo = _override param [5, []];
        _drone setVariable ["armatak_uas_spi_asl", _overrideSpiAsl, false];
        _drone setVariable ["armatak_uas_spi_geo", _overrideSpiGeo, false];
        _override select [0, 6]
    };
};

private _defaultFov = _drone getVariable ["armatak_uas_fov", 60];
private _maxRange = _drone getVariable ["armatak_uas_max_range", 15000];
private _originASL = getPosASL _drone;
private _originAGL = ASLToAGL _originASL;
private _cameraDir = [];
private _spiASL = [];
private _slantRange = 0;

private _laserTarget = laserTarget _drone;
if (!isNull _laserTarget) then {
    private _laserTargetWorld = getPosWorld _laserTarget;
    private _laserTargetAslZ = (getPosASL _laserTarget) select 2;
    _spiASL = [_laserTargetWorld select 0, _laserTargetWorld select 1, _laserTargetAslZ];
    _cameraDir = _spiASL vectorDiff _originASL;
    _slantRange = _originASL vectorDistance _spiASL;
};

if (_cameraDir isEqualTo []) then {
    private _uavControl = UAVControl _drone;
    private _controlledTurretPath = _uavControl param [1, []];
    private _candidateTurrets = [];

    if ((_controlledTurretPath isEqualType []) && {_controlledTurretPath isNotEqualTo []}) then {
        _candidateTurrets pushBack _controlledTurretPath;
    };

    {
        if !(_x in _candidateTurrets) then {
            _candidateTurrets pushBack _x;
        };
    } forEach (allTurrets _drone);

    {
        private _turretWeapons = _drone weaponsTurret _x;
        if (_turretWeapons isNotEqualTo []) exitWith {
            private _weapon = _turretWeapons select 0;
            private _weaponDirection = _drone weaponDirection _weapon;
            if (_weaponDirection isNotEqualTo [0, 0, 0]) then {
                _cameraDir = _weaponDirection;
            };
        };
    } forEach _candidateTurrets;
};

if (_cameraDir isEqualTo []) then {
    _cameraDir = vectorDirVisual _drone;
};

private _dirMagnitude = vectorMagnitude _cameraDir;
if (_dirMagnitude <= 0) then {
    private _fallbackAzimuth = getDir _drone;
    _cameraDir = [sin _fallbackAzimuth, cos _fallbackAzimuth, -1];
    _dirMagnitude = vectorMagnitude _cameraDir;
};

_cameraDir = _cameraDir vectorMultiply (1 / _dirMagnitude);

private _dirX = _cameraDir select 0;
private _dirY = _cameraDir select 1;
private _dirZ = _cameraDir select 2;
private _horizontalMagnitude = sqrt ((_dirX * _dirX) + (_dirY * _dirY));

private _azimuth = (((_dirX atan2 _dirY) + 360) mod 360);
private _elevation = (_dirZ atan2 (_horizontalMagnitude max 0.001));

if (_spiASL isEqualTo []) then {
    private _altitudeAGL = (_originAGL select 2) max 0.1;
    private _probeASL = _originASL vectorAdd (_cameraDir vectorMultiply _maxRange);

    if (_dirZ < -0.01 && {terrainIntersectASL [_originASL, _probeASL]}) then {
        private _near = _originASL;
        private _far = _probeASL;

        for "_i" from 0 to 24 do {
            private _mid = [
                ((_near select 0) + (_far select 0)) / 2,
                ((_near select 1) + (_far select 1)) / 2,
                ((_near select 2) + (_far select 2)) / 2
            ];

            if (terrainIntersectASL [_originASL, _mid]) then {
                _far = _mid;
            } else {
                _near = _mid;
            };
        };

        _spiASL = _far;
        _slantRange = _originASL vectorDistance _spiASL;
    } else {
        private _verticalComponent = abs _dirZ;

        if (_verticalComponent > 0.01) then {
            _slantRange = (_altitudeAGL / _verticalComponent) min _maxRange;
        } else {
            _slantRange = _maxRange;
        };

        _slantRange = _slantRange max 1;
        _spiASL = _originASL vectorAdd (_cameraDir vectorMultiply _slantRange);
        _spiASL set [2, getTerrainHeightASL [_spiASL select 0, _spiASL select 1]];
    };
};

if (_slantRange <= 0) then {
    _slantRange = (_originASL vectorDistance _spiASL) max 1;
};

private _spiAgl = ASLToAGL _spiASL;
private _spiWorld = [_spiAgl select 0, _spiAgl select 1, (_spiAgl select 2) max 0];
private _spiGeo = _spiWorld call armatak_client_fnc_convertClientLocation;

_drone setVariable ["armatak_uas_spi_asl", _spiASL, false];
_drone setVariable ["armatak_uas_spi_geo", _spiGeo, false];

[
    round _azimuth,
    round _elevation,
    round _defaultFov,
    round (_slantRange max 1),
    _spiASL,
    _spiGeo
]


