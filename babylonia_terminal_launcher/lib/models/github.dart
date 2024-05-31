import 'package:babylonia_terminal_launcher/messages/github.pb.dart';

class Github {
  static bool _isFetchingProtonVersions = false;
  static bool _isFetchingDXVKVersions = false;

  static Future<List<String>> getProtonVersions() async {
    if (!_isFetchingProtonVersions) {
      _isFetchingProtonVersions = true;
      AskProtonVersions().sendSignalToRust();
      final stream = ProtonVersions.rustSignalStream;

      await for (final rustSignal in stream) {
        _isFetchingProtonVersions = false;
        return rustSignal.message.versions;
      }
    }
    return [];
  }

  static Future<List<String>> getDXVKVersions() async {
    if (!_isFetchingDXVKVersions) {
      _isFetchingDXVKVersions = true;
      AskDXVKVersions().sendSignalToRust();
      final stream = DXVKVersions.rustSignalStream;

      await for (final rustSignal in stream) {
        _isFetchingDXVKVersions = false;
        return rustSignal.message.versions;
      }
    }
    return [];
  }
}
