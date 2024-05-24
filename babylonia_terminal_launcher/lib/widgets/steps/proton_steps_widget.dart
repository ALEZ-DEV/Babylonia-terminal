import 'package:flutter/material.dart';
import 'package:yaru/widgets.dart';

import './../../models/github.dart';

class ProtonSteps extends StatefulWidget {
  const ProtonSteps({super.key});

  @override
  State<ProtonSteps> createState() => _ProtonStepsState();
}

class _ProtonStepsState extends State<ProtonSteps> {
  bool hasLoaded = false;
  bool isLoading = false;
  late List<String> protonVersions;
  String? selectedValue;

  @override
  void didChangeDependencies() async {
    if (!hasLoaded) {
      isLoading = true;
      protonVersions = await Github.getProtonVersions();
      setState(() {
        isLoading = false;
        hasLoaded = true;
      });
    }

    super.didChangeDependencies();
  }

  @override
  Widget build(BuildContext context) {
    if (hasLoaded && selectedValue == null) {
      selectedValue = protonVersions.first;
    }

    return Padding(
      padding: const EdgeInsets.all(8.0),
      child: isLoading
          ? const Row(
              mainAxisAlignment: MainAxisAlignment.center,
              children: [
                Padding(
                  padding: EdgeInsets.all(8.0),
                  child: CircularProgressIndicator(),
                ),
                Text('Fetching versions...'),
              ],
            )
          : YaruPopupMenuButton(
              initialValue: selectedValue,
              itemBuilder: (_) => protonVersions
                  .map(
                    (e) => PopupMenuItem(
                      value: e,
                      child: Text(e),
                    ),
                  )
                  .toList(),
              onSelected: (v) => setState(() {
                selectedValue = v!;
              }),
              child: Text(selectedValue!),
            ),
    );
  }
}
