Version 0.1.0 (2024-04-04)
==========================
Lots of breaking changes to correct some of my own concerns and also some feedback from other people.
Yanked version 1.x.x and reverted to 0.1.0 to follow semver since the api is not yet 100% stable.
I will try to avoid breaking changes but there almost certainly be a few more ones. 
When I think the library is stable I will set the version to 1.0.0.

Breaking changes
----------------
- Removed SlotMapGraph
- Reduced Clone requirement
- `add_edge` now returns EdgeID.
- CategoryGraph renamed to CategorizedGraph
