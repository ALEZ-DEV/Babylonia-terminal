import 'package:flutter/material.dart';
import 'package:flutter_markdown/flutter_markdown.dart';
import 'package:yaru/yaru.dart';

class ReleaseNotice extends StatelessWidget {
  const ReleaseNotice(
      {super.key, required this.currentVersion, required this.info});

  final String currentVersion;
  final String info;

  @override
  Widget build(BuildContext context) {
    return AlertDialog(
      titlePadding: EdgeInsets.zero,
      title: const YaruDialogTitleBar(
        title: Text('Release notice'),
        isClosable: true,
      ),
      contentPadding: EdgeInsets.zero,
      content: SizedBox(
        height: 400,
        width: 700,
        child: Column(
          children: [
            Padding(
              padding: const EdgeInsets.only(top: 8.0),
              child: Text(
                "Version $currentVersion just dropped!",
                style: const TextStyle(
                  fontSize: 32,
                ),
              ),
            ),
            const Align(
              alignment: Alignment.centerLeft,
              child: Padding(
                padding: EdgeInsets.only(left: 25.0),
                child: Text(
                  "What's new :",
                  style: TextStyle(
                    fontSize: 16,
                  ),
                ),
              ),
            ),
            Padding(
              padding: const EdgeInsets.symmetric(horizontal: 30.0),
              child: SizedBox(
                height: 300,
                child: Markdown(data: info),
              ),
            ),
          ],
        ),
      ),
    );
  }
}
