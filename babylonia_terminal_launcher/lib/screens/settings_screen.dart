import 'package:flutter/material.dart';
import 'package:yaru/yaru.dart';

import './settings/settings_pages.dart';

class SettingsScreen extends StatefulWidget {
  const SettingsScreen({super.key});

  @override
  State<SettingsScreen> createState() => _SettingsScreenState();
}

class _SettingsScreenState extends State<SettingsScreen>
    with TickerProviderStateMixin {
  late TabController tabController;

  @override
  void initState() {
    super.initState();
    tabController =
        TabController(length: SettingsPage.getPages().length, vsync: this);
  }

  @override
  void dispose() {
    super.dispose();
    tabController.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return Center(
      child: Column(
        children: [
          SizedBox(
            child: YaruTabBar(
              tabController: tabController,
              tabs: SettingsPage.tabItems(),
            ),
          ),
          Expanded(
            child: TabBarView(
              controller: tabController,
              children: SettingsPage.getPages(),
            ),
          ),
        ],
      ),
    );
  }
}
