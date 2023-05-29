This module reads index/files.json to get a list of all vault files.

It then reads the files ending in `.md`, and collects the frontmatter yaml and the inline tags.
These two streams are combined into a `metadata` hashtable, and a new hashtable is created that maps
each file's relative path to the metadata hashtable of that file.

This hashtable is written to `<module_data_folder>/index/metadata.json`.

This metadata.json can then be used by other modules to filter out files from the `index/files.json` file based on metadata present in each file.