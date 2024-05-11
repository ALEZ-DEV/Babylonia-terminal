import 'package:babylonia_terminal_launcher/screens/screens.dart';
import 'package:flutter/material.dart';
import 'package:flutter/widgets.dart';
import 'package:media_kit/media_kit.dart';
import 'package:yaru/theme.dart';

import './screens/screens.dart';

void main() {
  WidgetsFlutterBinding.ensureInitialized();
  MediaKit.ensureInitialized();
  runApp(const BabyloniaLauncher());
}

class BabyloniaLauncher extends StatelessWidget {
  const BabyloniaLauncher({super.key});

  @override
  Widget build(BuildContext context) {
    return YaruTheme(
      builder: (context, yaru, child) => MaterialApp(
        title: "Babylonia Terminal",
        debugShowCheckedModeBanner: false,
        theme: yaru.theme,
        darkTheme: yaru.darkTheme,
        themeMode: ThemeMode.system,
        highContrastTheme: yaruHighContrastLight,
        highContrastDarkTheme: yaruHighContrastDark,
        home: const Menu(),
      ),
    );
  }
}

class Menu extends StatefulWidget {
  const Menu({super.key});

  @override
  State<Menu> createState() => _MenuState();
}

class _MenuState extends State<Menu> {
  int _selectedIndex = 0;

  @override
  Widget build(BuildContext context) {
    List<Widget> items = Screens.drawerItem(
      selectedIndex: _selectedIndex,
      onSelected: (index) {
        setState(() => _selectedIndex = index);
        Navigator.of(context).pop();
      },
    );
    items.insert(
      0,
      Center(
        child: Container(
          decoration: BoxDecoration(
            color: Theme.of(context).colorScheme.surface,
            border: Border(
              bottom: BorderSide(
                color: Theme.of(context).colorScheme.outline,
              ),
            ),
          ),
          child: Center(
            child: Padding(
              padding: const EdgeInsets.all(8.0),
              child: Column(
                children: [
                  Padding(
                    padding: const EdgeInsets.symmetric(
                      vertical: 8.0,
                      horizontal: 50.0,
                    ),
                    child: Image.asset(
                      'assets/images/Lee6.png',
                    ),
                  ),
                  const Text(
                    "Babylonia Terminal",
                    style: TextStyle(fontSize: 24),
                  ),
                ],
              ),
            ),
          ),
        ),
      ),
    );

    return Scaffold(
      drawer: Drawer(
        child: ListView(
          children: items,
        ),
      ),
      appBar: AppBar(
        title: const Text("Babylonia Terminal"),
        centerTitle: true,
      ),
      body: Screens.getCurrent(_selectedIndex),
    );
  }
}
