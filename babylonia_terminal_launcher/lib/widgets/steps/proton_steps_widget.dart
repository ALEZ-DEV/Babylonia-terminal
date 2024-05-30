import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import 'package:yaru/widgets.dart';

import './../../models/github.dart';
import './../../models/proton.dart';
import './../../widgets/simple_button.dart';

class ProtonSteps extends StatefulWidget {
  const ProtonSteps({super.key});

  @override
  State<ProtonSteps> createState() => _ProtonStepsState();
}

class _ProtonStepsState extends State<ProtonSteps> {
  final Proton proton = Proton();

  @override
  Widget build(BuildContext context) {
    return ChangeNotifierProvider(
      create: (_) => proton,
      child: Builder(
        builder: (context) {
          switch (Provider.of<Proton>(context).protonState) {
            case ProtonInstallationState.idle:
              return const InstallProton();
            case ProtonInstallationState.downloading:
              return const Center(
                child: Text('downloading...'),
              );
            case ProtonInstallationState.decompressing:
              return const Center(
                child: Text('decompressing...'),
              );
          }
        },
      ),
    );
  }
}

class InstallProton extends StatefulWidget {
  const InstallProton({super.key});

  @override
  State<InstallProton> createState() => _InstallProtonState();
}

class _InstallProtonState extends State<InstallProton> {
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
      padding: const EdgeInsets.all(20.0),
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
          : Row(
              mainAxisAlignment: MainAxisAlignment.center,
              children: [
                YaruPopupMenuButton(
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
                    selectedValue = v;
                  }),
                  child: Text(selectedValue!),
                ),
                Padding(
                  padding: const EdgeInsets.only(left: 8.0),
                  child: SimpleButton(
                    onPressed: () => Provider.of<Proton>(context, listen: false)
                        .startInstallation(context, selectedValue!),
                    child: const Text("Install"),
                  ),
                ),
              ],
            ),
    );
  }
}
