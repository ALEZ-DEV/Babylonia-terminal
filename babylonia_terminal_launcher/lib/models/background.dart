import 'dart:convert';
import 'dart:io';

import 'package:babylonia_terminal_launcher/models/config.dart';
import 'package:http/http.dart' as http;

import './../providers/providers.dart';

class Background {
  String path;

  Background._({required this.path});

  static Future<Background> get(SettingsProvider provider) async {
    final json = await _fetchMainMenu();
    final info = json['pcTopPicture'];
    final backgroundId = info['topPictureId'];
    final videoLink = info['backgroundVideo'];
    final path = "${Config.instance.path}/background.mp4";

    if (provider.backgroundId == null ||
        provider.backgroundId != backgroundId) {
      provider.backgroundId = backgroundId;

      await _updateVideo(videoLink, path);
    }

    return Background._(path: path);
  }

  static Future<Map<String, dynamic>> _fetchMainMenu() async {
    // ignore: prefer_interpolation_to_compose_strings, prefer_adjacent_string_concatenation
    final response = await http.get(
      Uri.parse('https://media-cdn-zspms.' +
          'k' +
          'u' +
          'r' +
          'o' +
          'game.net/pnswebsite/website2.0/json/G167/MainMenu.json'),
    );
    return jsonDecode(response.body);
  }

  static _updateVideo(String link, String path) async {
    final response = await http.get(Uri.parse(link));
    final file = File(path);
    if (await file.exists()) {
      // just to be sure to overwrite the file
      await file.delete();
      await file.create();
    }
    await file.writeAsBytes(response.bodyBytes);
  }
}
