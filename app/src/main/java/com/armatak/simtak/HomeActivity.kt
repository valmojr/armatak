package com.armatak.simtak

import android.Manifest
import android.app.Dialog
import android.content.Intent
import android.content.pm.PackageManager
import android.graphics.Color
import android.graphics.drawable.ColorDrawable
import android.net.Uri
import android.os.Build
import android.os.Bundle
import android.provider.Settings
import android.view.Window
import android.widget.Button
import android.widget.TextView
import android.widget.Toast
import androidx.activity.enableEdgeToEdge
import androidx.activity.result.ActivityResultLauncher
import androidx.activity.result.contract.ActivityResultContracts
import androidx.appcompat.app.AppCompatActivity
import androidx.core.content.ContextCompat
import androidx.core.view.ViewCompat
import androidx.core.view.WindowInsetsCompat
import com.armatak.simtak.databinding.ActivityHomeBinding
import com.armatak.simtak.trackerLog.TrackerLogActivity

class HomeActivity : AppCompatActivity() {
    private lateinit var binding: ActivityHomeBinding
    private var requestCamera: ActivityResultLauncher<String>? = null
    private var requestPermissionLauncher: ActivityResultLauncher<String>? = null
    private var requested = false

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        binding = ActivityHomeBinding.inflate(layoutInflater)
        setContentView(binding.root)
        enableEdgeToEdge()
        ViewCompat.setOnApplyWindowInsetsListener(binding.root) { v, insets ->
            val systemBars = insets.getInsets(WindowInsetsCompat.Type.systemBars())
            v.setPadding(systemBars.left, systemBars.top, systemBars.right, systemBars.bottom)
            insets
        }

        requestCamera = registerForActivityResult(ActivityResultContracts.RequestPermission()) {
            if (it) {
                initScan()
            } else {
                Toast.makeText(
                    this,
                    "Sem permissão para acessar a camera. Permita o acesso para continuar",
                    Toast.LENGTH_LONG
                ).show()
            }
        }

        requestPermissionLauncher = registerForActivityResult(
            ActivityResultContracts.RequestPermission(),
        ) { isGranted: Boolean ->
            if (isGranted) {
                Toast.makeText(baseContext, "Notificações Habilitadas", Toast.LENGTH_SHORT).show()
                requestCamera?.launch(Manifest.permission.CAMERA)
            } else {
                Toast.makeText(
                    baseContext,
                    "Por favor confirme a permissão",
                    Toast.LENGTH_SHORT
                ).show()
            }
        }

        initUI()

    }

    private fun validateUrl() {
        val etServerAddressLayout = binding.etServerAddressLayout
        val etServerAddress = binding.etServerAddress
        val serverAddress = etServerAddress.text.toString()

        val regexPatterns = listOf(
            "^ws://\\b(?:(?:2(?:[0-4][0-9]|5[0-5])|[0-1]?[0-9]?[0-9])\\.){3}(?:(?:2([0-4][0-9]|5[0-5])|[0-1]?[0-9]?[0-9]))\\b:[0-9]+\$",
            "^ws://([A-Za-z0-9]+(\\.[A-Za-z0-9]+)+)\$"
        )

        val allowed = regexPatterns.any { patterns ->
            Regex(patterns).matches(serverAddress)
        }

        if(allowed) {
            requested = false
            val intent = Intent(this, TrackerLogActivity::class.java)
            intent.putExtra("webSocketUrl", serverAddress)
            startActivity(intent)
        } else {
            requested = false
            etServerAddressLayout.error = "Server Address need to be a valid URL"
        }
    }

    private fun initScan() {
        startActivity(Intent(this, ScannerActivity::class.java))
    }


    private fun initUI() {
        binding.btnScanQrCode.setOnClickListener {
            if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.TIRAMISU) {
                if (ContextCompat.checkSelfPermission(
                        this,
                        Manifest.permission.POST_NOTIFICATIONS
                    ) ==
                    PackageManager.PERMISSION_GRANTED
                ) {
                    requestCamera?.launch(Manifest.permission.CAMERA)
                } else if (shouldShowRequestPermissionRationale(Manifest.permission.POST_NOTIFICATIONS)) {
                    requestNotificationByDialog()
                } else {
                    requestPermissionLauncher?.launch(Manifest.permission.POST_NOTIFICATIONS)
                }
            }
        }

        binding.btnConnectToServer.setOnClickListener {
            val serverAddress = binding.etServerAddress.text.toString()
            if (!requested && serverAddress.isNotBlank()){
                validateUrl()
            } else {
                binding.etServerAddressLayout.error = "This Input cannot be blank"
            }
        }
        configureFooterLinks()
    }


    private fun requestNotificationByDialog() {
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
        tvTitle.text = getString(R.string.allowNotificationsPermission)
        tvMessage.text = getString(R.string.needNotificationsPermissionMessage)
        btnAccept.text = getString(R.string.goToSettings)
        btnAccept.setOnClickListener {
            val intent = Intent(Settings.ACTION_APP_NOTIFICATION_SETTINGS)
            intent.putExtra(Settings.EXTRA_APP_PACKAGE, this.packageName)
            startActivity(intent)
            dialog.dismiss()
        }
        dialog.show()
    }

    private fun configureFooterLinks() {
        binding.btnGithubProject.setOnClickListener {
            openLink(getString(R.string.githubProjectUrl))
        }
        binding.btnWiki.setOnClickListener {
            openLink(getString(R.string.wikiUrl))
        }
        binding.btnDiscord.setOnClickListener {
            openLink(getString(R.string.discordUrl))
        }
        binding.btnSteamProject.setOnClickListener {
            openLink(getString(R.string.steamUrl))
        }
    }

    private fun openLink(url: String) {
        val intent = Intent(Intent.ACTION_VIEW, Uri.parse(url))
        startActivity(intent)
    }

}