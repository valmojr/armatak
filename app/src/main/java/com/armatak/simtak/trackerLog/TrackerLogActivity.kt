package com.armatak.simtak.trackerLog

import android.Manifest
import android.app.Dialog
import android.content.ComponentName
import android.content.Context
import android.content.Intent
import android.content.ServiceConnection
import android.graphics.Color
import android.graphics.drawable.ColorDrawable
import android.os.Build
import android.os.Bundle
import android.os.IBinder
import android.view.Window
import android.widget.Button
import android.widget.TextView
import android.widget.Toast
import androidx.activity.enableEdgeToEdge
import androidx.activity.result.ActivityResultLauncher
import androidx.activity.result.contract.ActivityResultContracts
import androidx.appcompat.app.AppCompatActivity
import androidx.core.view.ViewCompat
import androidx.core.view.WindowInsetsCompat
import androidx.lifecycle.lifecycleScope
import androidx.recyclerview.widget.LinearLayoutManager
import androidx.recyclerview.widget.RecyclerView
import com.armatak.simtak.LocationSimulationService
import com.armatak.simtak.R
import com.armatak.simtak.databinding.ActivityTrackerLogBinding
import com.armatak.simtak.trackerLog.data.adapters.AdapterLogTracker
import com.armatak.simtak.trackerLog.data.models.ConnectionStatus
import kotlinx.coroutines.flow.collectLatest
import kotlinx.coroutines.launch

class TrackerLogActivity : AppCompatActivity() {
    private var connectedToServer: Boolean = false
    private lateinit var binding: ActivityTrackerLogBinding
    private var url: String = ""
    private lateinit var locationPermissionRequest : ActivityResultLauncher<Array<String>>

    private var serviceStarted = false

    private var rvAdapter = AdapterLogTracker()
    private var lastThreeElementsAreVisible = false

    private lateinit var mService: LocationSimulationService
    private var mBound = false
    private val mConnection = object : ServiceConnection{
        override fun onServiceConnected(className: ComponentName?, binder : IBinder?) {
            val service = binder as LocationSimulationService.LocationMockBinder
            mService = service.getService()
            mBound = true
            initCollectors()
        }

        override fun onServiceDisconnected(name: ComponentName?) {
            mBound = false
        }

    }

    private fun initCollectors() {
        lifecycleScope.launch {
            if (mBound){
                mService.getLog().collect { logModelList ->
                    rvAdapter.submitList(logModelList)
                    if (lastThreeElementsAreVisible){
                        binding.rvLogTracker.smoothScrollToPosition(rvAdapter.itemCount - 1)
                    }
                }
            }
        }
        lifecycleScope.launch {
            if (mBound){
                mService.getConnectionStatus().collectLatest {
                    when (it){
                        ConnectionStatus.Connected -> {
                            binding.txtServerConnectionStatus.text = getString(R.string.serverConnectionStatusPropertyFormat, "Connected")
                        }
                        ConnectionStatus.Disconnected -> {
                            binding.txtServerConnectionStatus.text = getString(R.string.serverConnectionStatusPropertyFormat, "Disconnected")
                        }
                        ConnectionStatus.OnReconnect -> {
                            binding.txtServerConnectionStatus.text = getString(R.string.serverConnectionStatusPropertyFormat, "OnReconnect")
                        }
                        ConnectionStatus.Awaiting -> {
                            binding.txtServerConnectionStatus.text = getString(R.string.serverConnectionStatusPropertyFormat, "Awaiting")
                            showConnectionErrorDialog()
                        }
                        ConnectionStatus.InitialValue -> {
                            binding.txtServerConnectionStatus.text = getString(R.string.serverConnectionStatusPropertyFormat, "Not Initialized")
                        }
                    }
                }
            }
        }
    }

    private fun showConnectionErrorDialog() {
        val dialog = Dialog(this)
        dialog.requestWindowFeature(Window.FEATURE_NO_TITLE)
        dialog.setContentView(R.layout.dialog_alert)
        dialog.window?.setBackgroundDrawable(ColorDrawable(Color.TRANSPARENT))

        val tvTitle: TextView = dialog.findViewById(R.id.title)
        val tvMessage: TextView = dialog.findViewById(R.id.message)
        val btnCancel: Button = dialog.findViewById(R.id.btnCancel)
        val btnAccept: Button = dialog.findViewById(R.id.btnAccept)

        btnCancel.text = getString(R.string.cancel)
        btnCancel.setOnClickListener {
            dialog.dismiss()
        }
        tvTitle.text = getString(R.string.serverConnectionProblems)
        tvMessage.text = getString(R.string.errorDescriptionFormat, "Connection attempts failed. Check your network/server and try again")
        btnAccept.text = getString(R.string.tryAgain)
        btnAccept.setOnClickListener {
            if(mBound) mService.connectToServer(url)
            dialog.dismiss()
        }
        dialog.show()
    }

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        binding = ActivityTrackerLogBinding.inflate(layoutInflater)
        setContentView(binding.root)
        enableEdgeToEdge()
        ViewCompat.setOnApplyWindowInsetsListener(binding.root) { v, insets ->
            val systemBars = insets.getInsets(WindowInsetsCompat.Type.systemBars())
            v.setPadding(systemBars.left, systemBars.top, systemBars.right, systemBars.bottom)
            insets
        }

        url = intent.getStringExtra("webSocketUrl") ?: ""

        locationPermissionRequest = registerForActivityResult(
            ActivityResultContracts.RequestMultiplePermissions()
        ) { permissions ->
            when {
                permissions.getOrDefault(Manifest.permission.ACCESS_FINE_LOCATION, false) -> {
                    if(!serviceStarted){
                        Intent(this, LocationSimulationService::class.java).also {
                            startService(it)
                            bindService(it, mConnection, Context.BIND_AUTO_CREATE)
                        }
                        serviceStarted = true
                    }
                    if(mBound && !connectedToServer){
                        mService.connectToServer(url)
                        connectedToServer = true
                    }
                }

                permissions.getOrDefault(Manifest.permission.ACCESS_COARSE_LOCATION, false) -> {
                    if(!serviceStarted){
                        Intent(this, LocationSimulationService::class.java).also {
                            startService(it)
                            bindService(it, mConnection, Context.BIND_AUTO_CREATE)
                        }
                        serviceStarted = true
                    }
                    if(mBound && !connectedToServer){
                        mService.connectToServer(url)
                        connectedToServer = true
                    }
                }
                else -> {
                    Toast.makeText(baseContext, "Problems with permission", Toast.LENGTH_SHORT)
                        .show()
                }
            }
        }

    }

    override fun onStart() {
        super.onStart()
        initUI()
    }

    private fun initUI() {

        binding.txtServerAddress.text = getString(R.string.serverAddressPropertyFormat, url)

        binding.btnBack.setOnClickListener {
            if (!connectedToServer){
                onBackPressedDispatcher.onBackPressed()
            }
        }

        binding.startService.setOnClickListener {
            val connectionStatus = if (mBound){
                mService.getConnectionStatus().value
            } else {
                ConnectionStatus.InitialValue
            }
            when (connectionStatus){
                ConnectionStatus.Awaiting -> requestLocationPermission()
                ConnectionStatus.InitialValue -> requestLocationPermission()
                ConnectionStatus.Disconnected -> requestLocationPermission()
                else -> {
                    Toast.makeText(this, "SIMTAK Service is already running", Toast.LENGTH_SHORT).show()
                }
            }
        }
        if (!serviceStarted){
            binding.startService.performClick()
        }
        binding.stopService.setOnClickListener {
            val connectionStatus = if (mBound){
                mService.getConnectionStatus().value
            } else {
                ConnectionStatus.InitialValue
            }
            when (connectionStatus){
                ConnectionStatus.Awaiting -> secureStopService()
                ConnectionStatus.Connected -> secureStopService()
                ConnectionStatus.OnReconnect -> secureStopService()
                else -> {
                    Toast.makeText(this, "SIMTAK Service is already stopped", Toast.LENGTH_SHORT).show()
                }
            }
        }
        setUpLogTrackerRV()
    }

    private fun setUpLogTrackerRV() {
        val rv = binding.rvLogTracker
        rv.adapter = rvAdapter
        rv.layoutManager = LinearLayoutManager(this)
        rv.addOnScrollListener(object : RecyclerView.OnScrollListener(){
            override fun onScrolled(recyclerView: RecyclerView, dx: Int, dy: Int) {
                super.onScrolled(recyclerView, dx, dy)
                val layoutManager = recyclerView.layoutManager as LinearLayoutManager?
                layoutManager?.let {
                    val totalItemCount = it.itemCount
                    val lastVisibleItemPosition = it.findLastVisibleItemPosition()

                    lastThreeElementsAreVisible = totalItemCount - lastVisibleItemPosition <= 3
                }
            }
        })
    }

    private fun secureStopService(){
        if (mBound){
            mService.stopByActivity()
            connectedToServer = false
        }
    }

    override fun onResume() {
        super.onResume()
        if (rvAdapter.itemCount -1 > 0){
            binding.rvLogTracker.smoothScrollToPosition(rvAdapter.itemCount - 1)
        }
    }

    private fun requestLocationPermission() {
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.UPSIDE_DOWN_CAKE) {
            locationPermissionRequest.launch(
                arrayOf(
                    Manifest.permission.FOREGROUND_SERVICE_LOCATION,
                    Manifest.permission.ACCESS_FINE_LOCATION,
                    Manifest.permission.ACCESS_COARSE_LOCATION
                )
            )
        }
    }

    override fun onDestroy() {
        super.onDestroy()
        mService.stopByActivity()
        unbindService(mConnection)
    }
}