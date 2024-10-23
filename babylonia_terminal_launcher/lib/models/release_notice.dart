import 'package:flutter/services.dart';
import 'package:http/http.dart' as http;
import 'package:xml/xml.dart';
import 'package:xml/xpath.dart';

class ReleaseNoticeInfo {
  static const _releaseInfoUrl =
      "https://raw.githubusercontent.com/ALEZ-DEV/Babylonia-terminal-flatpak-builds/refs/heads/main/moe.celica.BabyloniaTerminal.metainfo.xml";

  static Future<String> getInfo(String currentVersion) async {
    final content = await rootBundle.loadString("CHANGELOG.md");
    return parseChangelog(content, currentVersion);
  }

  static String parseChangelog(String content, String currentVersion) {
    final regex = RegExp(r'(## \[' +
        currentVersion +
        '].+)(?<content>(.|\n)*)(?=(\[unreleased\]|(## \[\d\.\d\.\d\])))');
    final match = regex.firstMatch(content);
    if (match != null) {
      final changelogContent = match.namedGroup("content");

      if (changelogContent != null) {
        return changelogContent;
      }
    }
    return "### Failed to parse info";
  }
}
