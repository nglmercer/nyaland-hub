package com.nyaland.desktop

import android.content.Intent
import android.net.Uri
import android.os.Bundle
import android.os.Environment
import android.provider.Settings
import android.webkit.MimeTypeMap
import androidx.activity.enableEdgeToEdge
import androidx.core.content.FileProvider
import java.io.File

class MainActivity : TauriActivity() {
  override fun onCreate(savedInstanceState: Bundle?) {
    enableEdgeToEdge()
    super.onCreate(savedInstanceState)
    requestStoragePermission()
  }

  private fun requestStoragePermission() {
    if (android.os.Build.VERSION.SDK_INT >= android.os.Build.VERSION_CODES.R) {
      if (!Environment.isExternalStorageManager()) {
        val intent = Intent(Settings.ACTION_MANAGE_APP_ALL_FILES_ACCESS_PERMISSION).apply {
          data = Uri.parse("package:$packageName")
          addFlags(Intent.FLAG_ACTIVITY_NEW_TASK)
        }
        startActivity(intent)
      }
    }
  }

  private fun getMimeType(filePath: String): String {
    val extension = MimeTypeMap.getFileExtensionFromUrl(filePath)
    return MimeTypeMap.getSingleton().getMimeTypeFromExtension(extension)
      ?: when {
        filePath.endsWith(".mkv", true) -> "video/x-matroska"
        filePath.endsWith(".mp4", true) -> "video/mp4"
        filePath.endsWith(".avi", true) -> "video/x-msvideo"
        filePath.endsWith(".webm", true) -> "video/webm"
        filePath.endsWith(".mov", true) -> "video/quicktime"
        filePath.endsWith(".flv", true) -> "video/x-flv"
        filePath.endsWith(".wmv", true) -> "video/x-ms-wmv"
        filePath.endsWith(".m4v", true) -> "video/x-m4v"
        filePath.endsWith(".ts", true) -> "video/mp2t"
        filePath.endsWith(".rmvb", true) -> "application/vnd.rn-realmedia-vbr"
        filePath.endsWith(".mp3", true) -> "audio/mpeg"
        filePath.endsWith(".flac", true) -> "audio/flac"
        filePath.endsWith(".wav", true) -> "audio/wav"
        filePath.endsWith(".ogg", true) -> "audio/ogg"
        filePath.endsWith(".txt", true) -> "text/plain"
        filePath.endsWith(".srt", true) -> "application/x-subrip"
        filePath.endsWith(".ass", true) -> "text/x-ssa"
        else -> "*/*"
      }
  }

  @Suppress("unused")
  fun openFile(path: String): Boolean {
    return try {
      val file = File(path)
      if (!file.exists()) return false

      val uri: Uri = FileProvider.getUriForFile(
        this,
        "${applicationContext.packageName}.fileprovider",
        file
      )

      val mimeType = getMimeType(path)

      val intent = Intent(Intent.ACTION_VIEW).apply {
        setDataAndType(uri, mimeType)
        addFlags(Intent.FLAG_GRANT_READ_URI_PERMISSION)
        addFlags(Intent.FLAG_ACTIVITY_NEW_TASK)
      }

      startActivity(intent)
      true
    } catch (e: Exception) {
      e.printStackTrace()
      false
    }
  }

  @Suppress("unused")
  fun openFolder(path: String): Boolean {
    return try {
      val file = File(path)
      if (!file.exists()) return false

      val uri: Uri = FileProvider.getUriForFile(
        this,
        "${applicationContext.packageName}.fileprovider",
        file
      )

      val intent = Intent(Intent.ACTION_VIEW).apply {
        setDataAndType(uri, "resource/folder")
        addFlags(Intent.FLAG_GRANT_READ_URI_PERMISSION)
        addFlags(Intent.FLAG_ACTIVITY_NEW_TASK)
      }

      // Fallback: try with generic file browser if folder intent fails
      try {
        startActivity(intent)
      } catch (_: Exception) {
        val fallbackIntent = Intent(Intent.ACTION_VIEW).apply {
          setDataAndType(uri, "*/*")
          addFlags(Intent.FLAG_GRANT_READ_URI_PERMISSION)
          addFlags(Intent.FLAG_ACTIVITY_NEW_TASK)
        }
        startActivity(fallbackIntent)
      }
      true
    } catch (e: Exception) {
      e.printStackTrace()
      false
    }
  }
}
