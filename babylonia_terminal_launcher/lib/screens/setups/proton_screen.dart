import 'package:flutter/material.dart';
import 'package:flutter/widgets.dart';
import 'package:yaru/yaru.dart';

class ProtonScreen extends StatelessWidget {
  const ProtonScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Center(
      child: ConstrainedBox(
        constraints: const BoxConstraints(
          maxHeight: 600,
          maxWidth: 750,
        ),
        child: const Padding(
          padding: EdgeInsets.symmetric(horizontal: 25.0),
          child: YaruSection(
            headline: Center(
              child: Text('Proton'),
            ),
            child: Text('Some content'),
          ),
        ),
      ),
    );
  }
}
