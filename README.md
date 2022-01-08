# mac-disk-monitor

Pet project to practice rust.

The goal is to execute the command `diskutil activity` and parse every
line in real-time, extracting structured event data.

Developers should be able to subscribe to events and take action, for
example emitting a notification if a specific disk is mounted.


## Development

This repo is in active development with best effort to classic
Test-Driven Development, commits happen generaly in this order:

- "TDD [red] - write a failing test that given an input expects an output"
- "TDD [green] - write the smallest and/or simplest code to make the test pass, probably via hardcoding expected value"
- "TDD [refactor] - write real code to make the test pass


## Test Data

<details>
<summary>Click here to see the test data used as input for the "unit" tests</summary>

```log
***Begin monitoring DiskArbitration activity
***DiskAppeared ('disk4', DAVolumePath = 'file:///Volumes/my%20backups/', DAVolumeKind = 'hfs', DAVolumeName = 'Time Machine Backups') Time=20220108-20:22:05.1438
***DiskAppeared ('disk3s2', DAVolumePath = '<null>', DAVolumeKind = '<null>', DAVolumeName = '<null>') Time=20220108-20:22:05.1453
***DiskAppeared ('disk3s1', DAVolumePath = '<null>', DAVolumeKind = 'msdos', DAVolumeName = 'EFI') Time=20220108-20:22:05.1454
***DiskAppeared ('disk3s3', DAVolumePath = '<null>', DAVolumeKind = 'hfs', DAVolumeName = 'Boot OS X') Time=20220108-20:22:05.1455
***DiskAppeared ('disk3', DAVolumePath = '<null>', DAVolumeKind = '<null>', DAVolumeName = '<null>') Time=20220108-20:22:05.1456
***DiskAppeared ((no BSD name), DAVolumePath = 'file:///System/Volumes/Data/home/', DAVolumeKind = 'autofs', DAVolumeName = '<null>') Time=20220108-20:22:05.1457
***DiskAppeared ('disk2s1', DAVolumePath = 'file:///Volumes/garuda-ext/', DAVolumeKind = 'hfs', DAVolumeName = 'garuda-ext') Time=20220108-20:22:05.1458
***DiskAppeared ('disk2', DAVolumePath = '<null>', DAVolumeKind = '<null>', DAVolumeName = '<null>') Time=20220108-20:22:05.1459
***DiskAppeared ('disk0', DAVolumePath = '<null>', DAVolumeKind = '<null>', DAVolumeName = '<null>') Time=20220108-20:22:05.1460
***DiskAppeared ('disk0s1', DAVolumePath = '<null>', DAVolumeKind = 'msdos', DAVolumeName = 'EFI') Time=20220108-20:22:05.1461
***DiskAppeared ('disk0s2', DAVolumePath = '<null>', DAVolumeKind = '<null>', DAVolumeName = '<null>') Time=20220108-20:22:05.1462
***DiskAppeared ('disk1', DAVolumePath = '<null>', DAVolumeKind = '<null>', DAVolumeName = '<null>') Time=20220108-20:22:05.1463
***DiskAppeared ('disk1s1', DAVolumePath = 'file:///System/Volumes/Data/', DAVolumeKind = 'apfs', DAVolumeName = 'maindisk - Data') Time=20220108-20:22:05.1464
***DiskAppeared ('disk1s2', DAVolumePath = '<null>', DAVolumeKind = 'apfs', DAVolumeName = 'Preboot') Time=20220108-20:22:05.1465
***DiskAppeared ('disk1s3', DAVolumePath = 'file:///Volumes/Recovery/', DAVolumeKind = 'apfs', DAVolumeName = 'Recovery') Time=20220108-20:22:05.1466
***DiskAppeared ('disk1s4', DAVolumePath = 'file:///private/var/vm/', DAVolumeKind = 'apfs', DAVolumeName = 'VM') Time=20220108-20:22:05.1467
***DiskAppeared ('disk1s5', DAVolumePath = 'file:///', DAVolumeKind = 'apfs', DAVolumeName = 'maindisk') Time=20220108-20:22:05.1469
***DAIdle (no DADiskRef) Time=20220108-20:22:05.1470
***DiskUnmountApproval ('disk4', DAVolumePath = 'file:///Volumes/my%20backups/', DAVolumeKind = 'hfs', DAVolumeName = 'Time Machine Backups') Comment=Approving Time=20220108-20:22:09.9084
***DiskUnmountApproval ('disk4', DAVolumePath = 'file:///Volumes/my%20backups/', DAVolumeKind = 'hfs', DAVolumeName = 'Time Machine Backups') Comment=Approving Time=20220108-20:22:21.1909
***DiskDescriptionChanged ('disk4', DAVolumePath = '<null>') Time=20220108-20:22:21.5683
***DAIdle (no DADiskRef) Time=20220108-20:22:21.5684
***DiskDisappeared ('disk3', DAVolumePath = '<null>', DAVolumeKind = '<null>', DAVolumeName = '<null>') Time=20220108-20:22:29.6767
***DiskDisappeared ('disk4', DAVolumePath = '<null>', DAVolumeKind = 'hfs', DAVolumeName = 'Time Machine Backups') Time=20220108-20:22:29.6768
***DiskDisappeared ('disk3s3', DAVolumePath = '<null>', DAVolumeKind = 'hfs', DAVolumeName = 'Boot OS X') Time=20220108-20:22:29.6770
***DiskDisappeared ('disk3s2', DAVolumePath = '<null>', DAVolumeKind = '<null>', DAVolumeName = '<null>') Time=20220108-20:22:29.6772
***DiskDisappeared ('disk3s1', DAVolumePath = '<null>', DAVolumeKind = 'msdos', DAVolumeName = 'EFI') Time=20220108-20:22:29.6773
***DAIdle (no DADiskRef) Time=20220108-20:22:29.6774
***DiskPeek ('disk3s1') Time=20220108-20:22:35.8607
***DiskAppeared ('disk3s1', DAVolumePath = '<null>', DAVolumeKind = 'msdos', DAVolumeName = 'EFI') Time=20220108-20:22:35.8673
***DiskMountApproval ('disk3s1', DAVolumePath = '<null>', DAVolumeKind = 'msdos', DAVolumeName = 'EFI') Comment=Approving Time=20220108-20:22:35.8686
***DiskPeek ('disk3s3') Time=20220108-20:22:36.0009
***DiskPeek ('disk3s2') Time=20220108-20:22:36.0011
***DiskPeek ('disk3') Time=20220108-20:22:36.0014
***DiskAppeared ('disk3s3', DAVolumePath = '<null>', DAVolumeKind = 'hfs', DAVolumeName = 'Boot OS X') Time=20220108-20:22:36.0040
***DiskMountApproval ('disk3s3', DAVolumePath = '<null>', DAVolumeKind = 'hfs', DAVolumeName = 'Boot OS X') Comment=Approving Time=20220108-20:22:36.0065
***DiskAppeared ('disk3s2', DAVolumePath = '<null>', DAVolumeKind = '<null>', DAVolumeName = '<null>') Time=20220108-20:22:36.0116
***DiskAppeared ('disk3', DAVolumePath = '<null>', DAVolumeKind = '<null>', DAVolumeName = '<null>') Time=20220108-20:22:36.0118
***DAIdle (no DADiskRef) Time=20220108-20:22:36.0119
***DiskPeek ('disk4') Time=20220108-20:22:37.5920
***DiskAppeared ('disk4', DAVolumePath = '<null>', DAVolumeKind = 'hfs', DAVolumeName = 'Time Machine Backups') Time=20220108-20:22:37.5980
***DiskMountApproval ('disk4', DAVolumePath = '<null>', DAVolumeKind = 'hfs', DAVolumeName = 'Time Machine Backups') Comment=Approving Time=20220108-20:22:37.5985
***DiskDescriptionChanged ('disk4', DAVolumePath = 'file:///Volumes/my%20backups/') Time=20220108-20:22:41.5508
***DAIdle (no DADiskRef) Time=20220108-20:22:41.5509
***DiskUnmountApproval ('disk4', DAVolumePath = 'file:///Volumes/my%20backups/', DAVolumeKind = 'hfs', DAVolumeName = 'Time Machine Backups') Comment=Approving Time=20220108-20:22:58.3459
***DiskUnmountApproval ('disk4', DAVolumePath = 'file:///Volumes/my%20backups/', DAVolumeKind = 'hfs', DAVolumeName = 'Time Machine Backups') Comment=Approving Time=20220108-20:23:09.6402
***DiskDescriptionChanged ('disk4', DAVolumePath = '<null>') Time=20220108-20:23:10.0281
***DAIdle (no DADiskRef) Time=20220108-20:23:10.0282
***DiskDisappeared ('disk3', DAVolumePath = '<null>', DAVolumeKind = '<null>', DAVolumeName = '<null>') Time=20220108-20:26:42.3640
***DiskDisappeared ('disk4', DAVolumePath = '<null>', DAVolumeKind = 'hfs', DAVolumeName = 'Time Machine Backups') Time=20220108-20:26:42.3642
***DiskDisappeared ('disk3s3', DAVolumePath = '<null>', DAVolumeKind = 'hfs', DAVolumeName = 'Boot OS X') Time=20220108-20:26:42.3643
***DiskDisappeared ('disk3s2', DAVolumePath = '<null>', DAVolumeKind = '<null>', DAVolumeName = '<null>') Time=20220108-20:26:42.3645
***DiskDisappeared ('disk3s1', DAVolumePath = '<null>', DAVolumeKind = 'msdos', DAVolumeName = 'EFI') Time=20220108-20:26:42.3647
***DAIdle (no DADiskRef) Time=20220108-20:26:42.3647
***DiskPeek ('disk3s1') Time=20220108-20:26:48.0983
***DiskAppeared ('disk3s1', DAVolumePath = '<null>', DAVolumeKind = 'msdos', DAVolumeName = 'EFI') Time=20220108-20:26:48.1052
***DiskMountApproval ('disk3s1', DAVolumePath = '<null>', DAVolumeKind = 'msdos', DAVolumeName = 'EFI') Comment=Approving Time=20220108-20:26:48.1069
***DiskPeek ('disk3s3') Time=20220108-20:26:48.2289
***DiskPeek ('disk3s2') Time=20220108-20:26:48.2291
***DiskPeek ('disk3') Time=20220108-20:26:48.2292
***DiskAppeared ('disk3s3', DAVolumePath = '<null>', DAVolumeKind = 'hfs', DAVolumeName = 'Boot OS X') Time=20220108-20:26:48.2317
***DiskMountApproval ('disk3s3', DAVolumePath = '<null>', DAVolumeKind = 'hfs', DAVolumeName = 'Boot OS X') Comment=Approving Time=20220108-20:26:48.2345
***DiskAppeared ('disk3s2', DAVolumePath = '<null>', DAVolumeKind = '<null>', DAVolumeName = '<null>') Time=20220108-20:26:48.2418
***DiskAppeared ('disk3', DAVolumePath = '<null>', DAVolumeKind = '<null>', DAVolumeName = '<null>') Time=20220108-20:26:48.2419
***DAIdle (no DADiskRef) Time=20220108-20:26:48.2420
***DiskPeek ('disk4') Time=20220108-20:26:49.4535
***DiskAppeared ('disk4', DAVolumePath = '<null>', DAVolumeKind = 'hfs', DAVolumeName = 'Time Machine Backups') Time=20220108-20:26:49.4590
***DiskMountApproval ('disk4', DAVolumePath = '<null>', DAVolumeKind = 'hfs', DAVolumeName = 'Time Machine Backups') Comment=Approving Time=20220108-20:26:49.4594
***DiskDescriptionChanged ('disk4', DAVolumePath = 'file:///Volumes/my%20backups/') Time=20220108-20:26:52.7814
***DAIdle (no DADiskRef) Time=20220108-20:26:52.7814

```
</details>
