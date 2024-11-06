import 'package:flutter/services.dart';

class ReleaseNoticeInfo {
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
