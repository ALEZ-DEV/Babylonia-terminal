import 'package:babylonia_terminal_launcher/messages/steps/game.pb.dart';
import 'package:fixnum/fixnum.dart';
import 'package:flutter/material.dart';

import './../providers/providers.dart';

enum GameInstallationState {
  idle,
  checkingFile,
  downloading,
  patching,
}

enum GameRunState {
  idle,
  running,
}

class Game with ChangeNotifier {
  GameInstallationState gameInstallationState = GameInstallationState.idle;
  GameRunState gameRunState = GameRunState.idle;

  Int64 currentProgress = Int64(0);
  Int64 maxProgress = Int64(0);

  Future startInstallation(GameStateProvider gameState, bool isUpdating) async {
    StartGameInstallation(isUpdating: isUpdating).sendSignalToRust();
    gameInstallationState = GameInstallationState.checkingFile;
    notifyListeners();

    //final downloadStream = NotifyGameStartDownloading.rustSignalStream;
    //await for (final _ in downloadStream) {
    //  gameInstallationState = GameInstallationState.downloading;
    //  notifyListeners();
    //  break;
    //}

    final downloadProgresStream = GameInstallationProgress.rustSignalStream;
    await for (final rustSignal in downloadProgresStream) {
      if (gameInstallationState == GameInstallationState.checkingFile) {
        gameInstallationState = GameInstallationState.downloading;
      }

      currentProgress = rustSignal.message.current;
      maxProgress = rustSignal.message.max;
      print("progress current : $currentProgress / $maxProgress");
      notifyListeners();

      if (currentProgress >= maxProgress) {
        break;
      }
    }

    print("patching game...");
    gameInstallationState = GameInstallationState.patching;
    notifyListeners();

    final successStream = NotifyGameSuccessfullyInstalled.rustSignalStream;
    await for (final _ in successStream) {
      gameState.updateGameState();
      break;
    }
  }

  Future startGame() async {
    RunGame().sendSignalToRust();
    gameRunState = GameRunState.running;
    notifyListeners();

    final stream = GameStopped.rustSignalStream;
    await for (final _ in stream) {
      gameRunState = GameRunState.idle;
      notifyListeners();
      break;
    }
  }
}
