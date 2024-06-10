import 'package:babylonia_terminal_launcher/messages/game_state.pb.dart';
import 'package:flutter/material.dart';

class GameStateProvider with ChangeNotifier {
  States? _gameState;
  bool isUpdating = false;
  bool haveToSetup = false;

  get gameState {
    return _gameState;
  }

  Future updateGameState() async {
    if (!isUpdating) {
      isUpdating = true;
      AskGameState().sendSignalToRust();

      final stream = GameState.rustSignalStream;
      await for (final rustSignal in stream) {
        _gameState = rustSignal.message.state;
        break;
      }
      isUpdating = false;
      notifyListeners();
    }
  }

  void updateSetup() {
    haveToSetup = needToSetup();
    notifyListeners();
  }

  bool needToSetup() {
    return _gameState == States.ProtonNotInstalled ||
        _gameState == States.DXVKNotInstalled ||
        _gameState == States.FontNotInstalled ||
        _gameState == States.DependecieNotInstalled;
  }

  bool isGameUpdating() {
    return _gameState == States.GameNeedUpdate;
  }
}
