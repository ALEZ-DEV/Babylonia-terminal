import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_highlight/flutter_highlight.dart';
import 'package:flutter_highlight/themes/vs.dart';
import 'package:flutter_highlight/themes/vs2015.dart';
import 'package:yaru/icons.dart';
import 'package:yaru/widgets.dart';

import './../messages/error.pb.dart';

class ErrorReporter {
  static Future listenAllRustError(
      GlobalKey<NavigatorState> navigatorState) async {
    final stream = ReportError.rustSignalStream;
    await for (final rustSignal in stream) {
      final context = navigatorState.currentState!.overlay!.context;
      await showDialog<void>(
        context: context,
        builder: (context) => AlertDialog(
          titlePadding: EdgeInsets.zero,
          title: YaruDialogTitleBar(
            title: const Text('Error'),
            isClosable: true,
            leading: Center(
              child: YaruIconButton(
                icon: const Icon(YaruIcons.copy),
                tooltip: 'Copy',
                onPressed: () async {
                  Clipboard.setData(
                    ClipboardData(text: rustSignal.message.errorMessage),
                  );
                },
              ),
            ),
          ),
          contentPadding: EdgeInsets.zero,
          content: SizedBox(
            height: 400,
            width: 700,
            child: HighlightView(
              rustSignal.message.errorMessage,
              language: '',
              theme: Theme.of(context).brightness == Brightness.dark
                  ? vs2015Theme
                  : vsTheme,
              padding: const EdgeInsets.all(12),
              textStyle: const TextStyle(
                fontSize: 16,
              ),
            ),
          ),
        ),
      );
    }
  }
}
