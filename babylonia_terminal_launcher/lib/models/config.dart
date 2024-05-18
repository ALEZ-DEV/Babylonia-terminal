import 'package:babylonia_terminal_launcher/messages/config.pb.dart';

class Config {
  String path;

  static late Config instance;
  Config._({required this.path});

  static bool _isLoadingConfig = false;

  static Future update() async {
    if (!_isLoadingConfig) {
      _isLoadingConfig = true;
      ConfigInput().sendSignalToRust();
      final stream = ConfigOutput.rustSignalStream;

      await for (final rustSignal in stream) {
        instance = Config._(
          path: rustSignal.message.configPath,
        );
        break;
      }
      _isLoadingConfig = false;
    }
  }
}
