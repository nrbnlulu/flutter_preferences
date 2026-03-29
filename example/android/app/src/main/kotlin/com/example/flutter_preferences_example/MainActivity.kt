package com.example.flutter_preferences_example

import android.content.Context
import androidx.annotation.NonNull
import io.flutter.embedding.android.FlutterActivity
import io.flutter.embedding.engine.FlutterEngine
import io.flutter.embedding.engine.plugins.FlutterPlugin
import io.flutter.plugin.common.MethodCall
import io.flutter.plugin.common.MethodChannel.MethodCallHandler
import io.flutter.plugin.common.MethodChannel.Result

class MainActivity: FlutterActivity() {
    override fun configureFlutterEngine(
        @NonNull flutterEngine: FlutterEngine,
    ) {
        flutterEngine.plugins.add(FlutterPreferencesPlugin())
        super.configureFlutterEngine(flutterEngine)
    }
}

class FlutterPreferencesPlugin : FlutterPlugin, MethodCallHandler {
    companion object {
        init {
            System.loadLibrary("flutter_preferences")
        }
    }

    external fun initAndroid(ctx: Context)

    override fun onAttachedToEngine(
        @NonNull flutterPluginBinding: FlutterPlugin.FlutterPluginBinding,
    ) {
        initAndroid(flutterPluginBinding.applicationContext)
    }

    override fun onMethodCall(
        @NonNull call: MethodCall,
        @NonNull result: Result,
    ) {
        result.notImplemented()
    }

    override fun onDetachedFromEngine(
        @NonNull binding: FlutterPlugin.FlutterPluginBinding,
    ) {
    }
}
