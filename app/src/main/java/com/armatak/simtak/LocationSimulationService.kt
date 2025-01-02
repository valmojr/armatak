package com.armatak.simtak

import android.app.NotificationManager
import android.app.Service
import android.content.Intent
import android.location.Location
import android.location.LocationManager
import android.location.provider.ProviderProperties
import android.os.Binder
import android.os.Build
import android.os.Handler
import android.os.IBinder
import android.os.Looper
import android.os.SystemClock
import android.util.Log
import com.armatak.simtak.core.Util.createNotificationChannel
import com.armatak.simtak.core.Util.getActualTime
import com.armatak.simtak.core.Util.getMockLocationStoppedNotification
import com.armatak.simtak.core.Util.getNeedReconnectNotification
import com.armatak.simtak.core.Util.getRunningNotification
import com.armatak.simtak.core.Util.getServiceDestroyedNotification
import com.armatak.simtak.core.Util.getStartedServiceNotification
import com.armatak.simtak.trackerLog.data.models.ConnectionStatus
import com.armatak.simtak.trackerLog.data.models.LogModel
import com.armatak.simtak.trackerLog.data.models.LogTypes
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.collectLatest
import kotlinx.coroutines.launch
import org.json.JSONObject

private const val TAG = "LocationSimulationService"

class LocationSimulationService : Service() {
    private val binder = LocationMockBinder()
    private lateinit var webSocketClient: WebSocketClient

    private var logTrackerMutableList = mutableListOf<LogModel>()
    private var lastLineId = 0

    private var connectionAttemps = 0

    private val _connectionStatus = MutableStateFlow<ConnectionStatus>(ConnectionStatus.InitialValue)
    private val connectionStatus : StateFlow<ConnectionStatus> = _connectionStatus
    private val _logTracker = MutableStateFlow(emptyList<LogModel>())
    private val logTracker : StateFlow<List<LogModel>> = _logTracker

    init {
        Log.d(TAG, "$TAG, initialized")
    }

    private val socketListener = object : WebSocketClient.SocketListener {
        override fun onMessage(message: String) {
            try {
                if (message[0] == '{') {
                    val jsonObject = JSONObject(message)
                    val latitude = jsonObject.getDouble("latitude")
                    val longitude = jsonObject.getDouble("longitude")
                    val bearing = jsonObject.getDouble("bearing")
                    simulateLocation(latitude, longitude, bearing)
                    addEntryToLog(message, LogTypes.Normal)
                } else {
                    Log.e(TAG, "Non an JsonObject, text: $message")
                    addEntryToLog("Non an JsonObject, text: $message", LogTypes.Warning)
                }
            } catch (e: Exception) {
                Log.e(TAG, e.localizedMessage, e)
                addEntryToLog(e.localizedMessage, LogTypes.Error)
            }
        }

    }

    fun addEntryToLog(message: String?, type: LogTypes) {
        lastLineId += 1
        logTrackerMutableList.add(
            LogModel(
                lastLineId,
                getActualTime(),
                message?:"null",
                type
            )
        )
        val newList = logTrackerMutableList.toList()
        _logTracker.value = newList
    }

    fun getLog(): StateFlow<List<LogModel>> {
        return logTracker
    }
    fun getConnectionStatus(): StateFlow<ConnectionStatus> {
        return connectionStatus
    }

    fun connectToServer(url: String){
        CoroutineScope(Dispatchers.IO).launch {
            try {
                connectionAttemps ++
                webSocketClient = WebSocketClient.getInstance()
                addEntryToLog("Server Address: $url", LogTypes.NetworkOperation)
                webSocketClient.setSocketUrl(url)
                webSocketClient.setListener(socketListener)
                webSocketClient.connect()
                _connectionStatus.value = ConnectionStatus.Connected
                addEntryToLog("Connection Server Success", LogTypes.NetworkOperation)
            } catch (e: Exception) {
                Log.e(TAG, e.localizedMessage, e)
                when (e.localizedMessage){
                    "Expected URL scheme 'http' or 'https' but no scheme was found for test u..." -> {
                        addEntryToLog("Expected ws:// or wss:// scheme, this is only for debug", LogTypes.Warning)
                        _connectionStatus.value = ConnectionStatus.Disconnected
                    }
                    else -> {
                        if (connectionAttemps < 6){
                            addEntryToLog("Attemp: $connectionAttemps \nError:${e.localizedMessage}", LogTypes.Warning)
                            _connectionStatus.value = ConnectionStatus.OnReconnect
                            Handler(Looper.getMainLooper()).postDelayed({
                                connectToServer(url)
                            }, 1500)
                        } else {
                            addEntryToLog("Exceed Connection Attemps", LogTypes.Error)
                            connectionAttemps = -1
                            _connectionStatus.value = ConnectionStatus.Awaiting
                        }
                    }
                }
            }
        }
    }

    override fun onBind(intent: Intent): IBinder {
        return binder
    }


    private fun simulateLocation(latitude: Double, longitude: Double, bearing: Double) {
        val locationManager = baseContext.getSystemService(LOCATION_SERVICE) as LocationManager
        // Create a Location Object
        val location = Location(LocationManager.GPS_PROVIDER)
        location.latitude = latitude
        location.longitude = longitude
        location.accuracy = 3f
        location.altitude = 0.0
        location.time = System.currentTimeMillis()
        location.bearing = bearing.toFloat()
        location.setBearingAccuracyDegrees(0.1F)
        location.setVerticalAccuracyMeters(0.1F)
        location.setSpeedAccuracyMetersPerSecond(0.01F)
        location.elapsedRealtimeNanos = SystemClock.elapsedRealtimeNanos()
        var powerUsage = 3
        var accuracy = 5
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.S){
            powerUsage = ProviderProperties.POWER_USAGE_LOW
            accuracy = ProviderProperties.ACCURACY_COARSE
        }
        //Create Test Provider
        locationManager.addTestProvider(
            LocationManager.GPS_PROVIDER,
            false,
            false,
            false,
            false,
            false,
            true,
            true,
            powerUsage,
            accuracy
        )
        // Enable Mock Provider
        locationManager.setTestProviderEnabled(LocationManager.GPS_PROVIDER, true)

        // Mock Location on System
        locationManager.setTestProviderLocation(LocationManager.GPS_PROVIDER, location)
    }

    fun stopByActivity() {
        webSocketClient.disconnect()
        _connectionStatus.value = ConnectionStatus.Disconnected
        addEntryToLog("Connection Stopped", LogTypes.NetworkOperation)
        val locationManager = baseContext.getSystemService(LOCATION_SERVICE) as LocationManager
        locationManager.removeTestProvider(LocationManager.GPS_PROVIDER)
        addEntryToLog("MockLocation Stopped", LogTypes.Warning)
        this.stopSelf()
    }

    override fun onCreate() {
        super.onCreate()
        val manager = getSystemService(NOTIFICATION_SERVICE) as NotificationManager
        createNotificationChannel(manager)
        CoroutineScope(Dispatchers.Default).launch{
            notifyStatusByPushNotification()
        }
    }

    override fun onStartCommand(intent: Intent?, flags: Int, startId: Int): Int {
        val notification = getStartedServiceNotification(this)
        startForeground(5142, notification)
        return START_STICKY
    }

    private suspend fun notifyStatusByPushNotification(){
        getConnectionStatus().collectLatest { status ->
            when(status){
                ConnectionStatus.Awaiting -> {
                    val notification = getNeedReconnectNotification(this)
                    val manager = getSystemService(NOTIFICATION_SERVICE) as NotificationManager
                    manager.notify(5142, notification)
                }
                ConnectionStatus.Connected -> {
                    val notification = getRunningNotification(this)
                    val manager = getSystemService(NOTIFICATION_SERVICE) as NotificationManager
                    manager.notify(5142, notification)
                }
                ConnectionStatus.Disconnected -> {
                    val notification = getMockLocationStoppedNotification(this)
                    val manager = getSystemService(NOTIFICATION_SERVICE) as NotificationManager
                    manager.notify(5142, notification)
                }
                else -> {}
            }
        }
    }

    override fun onDestroy() {
        super.onDestroy()
        val notification = getServiceDestroyedNotification(this)
        val manager = getSystemService(NOTIFICATION_SERVICE) as NotificationManager
        manager.notify(5142, notification)
        Log.e(TAG, "Service destroyed")
    }

    inner class LocationMockBinder: Binder(){
        fun getService() = this@LocationSimulationService
    }
}