import 'package:flutter/material.dart';
import 'package:flutter/widgets.dart';
import 'package:media_kit/media_kit.dart';

import './app.dart';
import './messages/generated.dart';

void main() async {
  await initializeRust();
  WidgetsFlutterBinding.ensureInitialized();
  MediaKit.ensureInitialized();
  runApp(const BabyloniaLauncher());
}
