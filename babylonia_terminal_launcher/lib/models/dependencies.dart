import 'package:flutter/material.dart';

import './../messages/steps/dependencies.pb.dart';
import './../providers/providers.dart';

enum DependenciesInstallationState {
  idle,
  installing,
}

class Dependencies with ChangeNotifier {
  DependenciesInstallationState dependeciesState =
      DependenciesInstallationState.idle;

  Future startInstallation(GameStateProvider gameState) async {
    StartDependenciesInstallation().sendSignalToRust();
    final stream = NotifyDependenciesSuccessfullyInstalled.rustSignalStream;
    await for (final _ in stream) {
      gameState.updateGameState();
      break;
    }
  }
}
