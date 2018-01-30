# Request for Comment: Memory v. Disk
There are two options for handling files. The first is to keep them stored on the disk and not load them into memory. The second is to load all files into memory. *This RfC is being discussed on its [issue page](https://github.com/milesmcc/ArmorLib/issues/2).*

## Disk approach
### Advantages
* Files of any size can be safely scanned without worrying about hogging memory.
* Potentially more secure—loading files into RAM makes them possibly accessible to other programs or the CPU for potential execution.

### Disadvantages
* Redundant file IO, as each scan module would have to reload the file from the disk.
* Slow.

## Memory approach
### Advantages
* Fast and not redundant.
* Orthodox.
* Does not assume that files will necessarily be stored on the disk.

### Disadvantages
* Could prevent large files from being processed.
* Poses a potential security risk.

---

## Comments
*Please respond to this RfC on its [issue page](https://github.com/milesmcc/ArmorLib/issues/2).*
