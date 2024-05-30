import 'package:fixnum/fixnum.dart';
import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

import './../messages/steps/proton.pb.dart';
import './../providers/providers.dart';

enum ProtonInstallationState {
  idle,
  downloading,
  decompressing,
}

class Proton with ChangeNotifier {
  ProtonInstallationState protonState = ProtonInstallationState.idle;

  Int64 currentProgress = Int64(0);
  Int64 maxProgress = Int64(0);

  Future startInstallation(BuildContext context, String protonVersion) async {
    protonState = ProtonInstallationState.downloading;
    notifyListeners();

    StartProtonInstallation(protonVersion: protonVersion).sendSignalToRust();
    final progressStream = ProtonDownloadProgress.rustSignalStream;
    await for (final rustSignal in progressStream) {
      currentProgress = rustSignal.message.current;
      maxProgress = rustSignal.message.max;
      notifyListeners();

      if (currentProgress >= maxProgress) {
        break;
      }
    }

    final notificationDecompressingStream =
        NotifyProtonStartDecompressing.rustSignalStream;
    await for (final _ in notificationDecompressingStream) {
      protonState = ProtonInstallationState.decompressing;
      notifyListeners();
      break;
    }

    final notificationInstalledStream =
        NotifiyProtonSuccessfullyInstalled.rustSignalStream;
    await for (final _ in notificationInstalledStream) {
      Provider.of<GameStateProvider>(context, listen: false).updateGameState();
      break;
    }
  }
}
