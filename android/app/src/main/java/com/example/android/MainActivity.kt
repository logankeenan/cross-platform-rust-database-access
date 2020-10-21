package com.example.android

import android.os.Bundle
import android.util.Log
import androidx.appcompat.app.AppCompatActivity

class MainActivity : AppCompatActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)

        System.loadLibrary("rust")
        var databasePath = packageManager.getPackageInfo(packageName, 0).applicationInfo.dataDir
        Log.d("rust", calldatabase("$databasePath/database.sql"))
    }

    external fun calldatabase(to: String): String
}