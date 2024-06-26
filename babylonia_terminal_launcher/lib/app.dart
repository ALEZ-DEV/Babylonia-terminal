import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import 'package:yaru/yaru.dart';

import './screens/screens.dart';
import './screens/setup_screen.dart';
import './providers/providers.dart';
import './models/error_reporter.dart';
import './models/game.dart';

class BabyloniaLauncher extends StatelessWidget {
  BabyloniaLauncher(
      {super.key,
      required this.settingsProvider,
      required this.gameStateProvider});
  final SettingsProvider settingsProvider;
  final GameStateProvider gameStateProvider;
  final GlobalKey<NavigatorState> navigatorKey = GlobalKey<NavigatorState>();

  @override
  Widget build(BuildContext context) {
    ErrorReporter.listenAllRustError(navigatorKey);
    gameStateProvider.updateSetup();
    return MultiProvider(
      providers: [
        ChangeNotifierProvider(
          create: (context) => settingsProvider,
        ),
        ChangeNotifierProvider(
          create: (context) => gameStateProvider,
        ),
      ],
      child: YaruTheme(
        builder: (context, yaru, child) => MaterialApp(
          navigatorKey: navigatorKey,
          title: "Babylonia Terminal",
          debugShowCheckedModeBanner: false,
          theme: yaru.theme,
          darkTheme: yaru.darkTheme,
          themeMode: ThemeMode.system,
          highContrastTheme: yaruHighContrastLight,
          highContrastDarkTheme: yaruHighContrastDark,
          home: Provider.of<GameStateProvider>(context).haveToSetup
              ? const SetupScreen()
              : const Menu(),
        ),
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

    return ChangeNotifierProvider(
      create: (context) => Game(),
      child: Scaffold(
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
      ),
    );
  }
}
