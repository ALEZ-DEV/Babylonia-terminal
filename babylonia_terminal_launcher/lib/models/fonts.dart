import 'package:fixnum/fixnum.dart';
import 'package:flutter/material.dart';

import './../messages/steps/fonts.pb.dart';
import './../providers/providers.dart';

enum FontsInstallationState {
  idle,
  installing,
}

class Fonts with ChangeNotifier {
  FontsInstallationState fontsState = FontsInstallationState.idle;

  Int64 currentProgress = Int64(0);
  Int64 maxProgress = Int64(0);

  Future startInstallation(GameStateProvider gameState) async {
    StartFontsInstallation().sendSignalToRust();
    final stream = FontsInstallationProgress.rustSignalStream;
    await for (final rustSignal in stream) {
      maxProgress = rustSignal.message.max;
      currentProgress = rustSignal.message.current;

      if (fontsState == FontsInstallationState.idle) {
        fontsState = FontsInstallationState.installing;
      }

      notifyListeners();

      if (currentProgress >= maxProgress) {
        break;
      }
    }

    gameState.updateGameState();
  }
}
