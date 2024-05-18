import 'package:flutter/material.dart';
import 'package:flutter/widgets.dart';
import 'package:media_kit/media_kit.dart';

import './app.dart';
import './messages/generated.dart';
import './providers/providers.dart';

void main() async {
  await initializeRust();
  WidgetsFlutterBinding.ensureInitialized();
  MediaKit.ensureInitialized();

  final SettingsProvider settings = SettingsProvider();
  await settings.init();

  final GameStateProvider gameState = GameStateProvider();
  await gameState.updateGameState();

  runApp(BabyloniaLauncher(
    settingsProvider: settings,
    gameStateProvider: gameState,
  ));
}
