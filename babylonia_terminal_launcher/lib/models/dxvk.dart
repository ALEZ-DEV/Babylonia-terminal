import 'package:fixnum/fixnum.dart';
import 'package:flutter/material.dart';

import './../messages/steps/dxvk.pb.dart';
import './../providers/providers.dart';

enum DXVKInstallationState {
  idle,
  downloading,
  decompressing,
}

class DXVK with ChangeNotifier {
  DXVKInstallationState dxvkState = DXVKInstallationState.idle;

  Int64 currentProgress = Int64(0);
  Int64 maxProgress = Int64(0);

  Future startInstallation(
      GameStateProvider gameStateProvider, String protonVersion) async {
    StartDXVKInstallation(protonVersion: protonVersion).sendSignalToRust();
    final progressStream = DXVKDownloadProgress.rustSignalStream;
    await for (final rustSignal in progressStream) {
      currentProgress = rustSignal.message.current;
      maxProgress = rustSignal.message.max;

      if (dxvkState == DXVKInstallationState.idle) {
        dxvkState = DXVKInstallationState.downloading;
      }

      notifyListeners();

      if (currentProgress >= maxProgress) {
        break;
      }
    }

    final notificationDecompressingStream =
        NotifyDXVKStartDecompressing.rustSignalStream;
    await for (final _ in notificationDecompressingStream) {
      dxvkState = DXVKInstallationState.decompressing;
      notifyListeners();
      break;
    }

    final notificationInstalledStream =
        NotifyDXVKSuccessfullyInstalled.rustSignalStream;
    await for (final _ in notificationInstalledStream) {
      gameStateProvider.updateGameState();
      break;
    }
  }
}
