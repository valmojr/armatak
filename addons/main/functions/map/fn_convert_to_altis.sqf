params["_unit"];

_playerPosition = getPos _unit;

_playerLatitude = _playerPosition select 0;
_playerLongitude = _playerPosition select 1;

_playerMaxLongitude = 30720;
_playerMaxLatitude = 30720;

_MapMaxLatitude = 25.481608;
_MapMinLatitude = 25.011719;

_MapMaxLongitude = 40.095572;
_MapMinLongitude = 39.717094;

_LongitudeDifference = _MapMaxLongitude - _MapMinLongitude;
_LatitudeDifference = _MapMaxLatitude - _MapMinLatitude;

_RealLongitude = (_playerLongitude / _playerMaxLongitude) * _LongitudeDifference + _MapMinLongitude;
_RealLatitude = (_playerLatitude / _playerMaxLatitude) * _LatitudeDifference + _MapMinLatitude;

[_RealLongitude, _RealLatitude, _playerPosition select 2]