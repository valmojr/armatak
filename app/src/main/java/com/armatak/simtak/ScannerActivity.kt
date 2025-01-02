package com.armatak.simtak

import android.annotation.SuppressLint
import android.content.Intent
import android.os.Bundle
import android.os.Handler
import android.os.Looper
import android.view.SurfaceHolder
import android.widget.Toast
import androidx.appcompat.app.AppCompatActivity
import androidx.lifecycle.lifecycleScope
import com.armatak.simtak.databinding.ActivityScannerBinding
import com.armatak.simtak.trackerLog.TrackerLogActivity
import com.google.android.gms.vision.CameraSource
import com.google.android.gms.vision.Detector
import com.google.android.gms.vision.barcode.Barcode
import com.google.android.gms.vision.barcode.BarcodeDetector
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.launch
import kotlinx.coroutines.withContext
import java.io.IOException

class ScannerActivity : AppCompatActivity() {

    private lateinit var binding: ActivityScannerBinding

    private var tentativa = false
    private lateinit var barcodeDetector: BarcodeDetector
    private lateinit var cameraSource: CameraSource
    var intentData = ""
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        binding = ActivityScannerBinding.inflate(layoutInflater)
        setContentView(binding.root)
        initScanBarcode()
    }

    private fun initScanBarcode() {
        barcodeDetector = BarcodeDetector.Builder(this)
            .setBarcodeFormats(Barcode.QR_CODE)
            .build()
        cameraSource = CameraSource.Builder(this, barcodeDetector)
            .setRequestedPreviewSize(1080, 1080)
            .setAutoFocusEnabled(true)
            .setFacing(CameraSource.CAMERA_FACING_BACK)
            .build()
        binding.surfaceView.holder.addCallback(object : SurfaceHolder.Callback {
            @SuppressLint("MissingPermission")
            override fun surfaceCreated(holder: SurfaceHolder) {
                try {
                    cameraSource.start(binding.surfaceView.holder)
                } catch (e: IOException) {
                    e.printStackTrace()
                }
            }

            override fun surfaceChanged(
                holder: SurfaceHolder,
                format: Int,
                width: Int,
                height: Int
            ) {

            }

            override fun surfaceDestroyed(holder: SurfaceHolder) {
                cameraSource.stop()
            }
        })
        barcodeDetector.setProcessor(object : Detector.Processor<Barcode> {
            override fun release() {
                Toast.makeText(applicationContext, "Scanner was stopped", Toast.LENGTH_SHORT).show()
            }

            override fun receiveDetections(detections: Detector.Detections<Barcode>) {
                val barcodes = detections.detectedItems
                if (barcodes.size() != 0) {
                    Thread.sleep(300)

                    intentData = barcodes.valueAt(0).displayValue


                    if (!tentativa) {
                        initTrackerActivity(intentData)
                        tentativa = true
                    }
                }
            }

        })
    }

    private fun initTrackerActivity(url: String?) {
        lifecycleScope.launch(Dispatchers.IO) {
            if (url != null) {
                val intent = Intent(this@ScannerActivity, TrackerLogActivity::class.java)
                intent.putExtra("webSocketUrl", url)
                startActivity(intent)
                Handler(Looper.getMainLooper()).postDelayed(
                    { tentativa = false },
                    2000
                )
            } else {
                withContext(Dispatchers.Main) {
                    Toast.makeText(baseContext, "Try Again, Scan Error", Toast.LENGTH_SHORT).show()
                }
                Handler(Looper.getMainLooper()).postDelayed(
                    { tentativa = false },
                    1500
                )
            }
        }
    }
}