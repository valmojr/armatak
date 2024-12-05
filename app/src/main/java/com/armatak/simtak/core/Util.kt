package com.armatak.simtak.core

import android.app.Notification
import android.app.NotificationChannel
import android.app.NotificationManager
import android.content.Context
import androidx.core.app.NotificationCompat
import com.armatak.simtak.R
import java.time.LocalDateTime
import java.time.format.DateTimeFormatter

object Util {
    const val CHANNEL_ID = "websocketChannel"
    fun getActualTime(): String {
        val now = LocalDateTime.now()
        val formatter = DateTimeFormatter.ofPattern("HH:mm:ss:SSS")
        return now.format(formatter)
    }

    fun createNotificationChannel(manager: NotificationManager) {
        val websocketChannel = NotificationChannel(
            CHANNEL_ID, "websocketChannelName",
            NotificationManager.IMPORTANCE_HIGH
        )
        websocketChannel.description = "websocketChannelDescription"
        websocketChannel.enableVibration(true)
        manager.createNotificationChannel(websocketChannel)
    }

    fun getStartedServiceNotification(context: Context): Notification {
        return NotificationCompat.Builder(context, CHANNEL_ID)
            .setContentTitle("Service Started")
            .setContentText("Service is ready to Start Tracking")
            .setSmallIcon(R.drawable.appicon_simtak)
            .build()
    }
    fun getRunningNotification(context: Context): Notification {
        return NotificationCompat.Builder(context, CHANNEL_ID)
            .setContentTitle("Mocking Location")
            .setContentText("Service is running")
            .setSmallIcon(R.drawable.appicon_simtak)
            .build()
    }

    fun getNeedReconnectNotification(context: Context): Notification{
        return NotificationCompat.Builder(context, CHANNEL_ID)
            .setContentTitle("Need restart server connection")
            .setContentText("Connection attempts failed. Check your network/server and try again")
            .setSmallIcon(R.drawable.appicon_simtak)
            .build()
    }
    fun getMockLocationStoppedNotification(context: Context): Notification{
        return NotificationCompat.Builder(context, CHANNEL_ID)
            .setContentTitle("Mock Location Stopped")
            .setContentText("Disconnect from server")
            .setSmallIcon(R.drawable.appicon_simtak)
            .build()
    }
    fun getServiceDestroyedNotification(context: Context): Notification{
        return NotificationCompat.Builder(context, CHANNEL_ID)
            .setContentTitle("Service Destroyed")
            .setSmallIcon(R.drawable.appicon_simtak)
            .build()
    }
}