# A Simple Implementation for HeyStack in Rust

> https://www.usenix.org/legacy/event/osdi10/tech/full_papers/Beaver.pdf

HeyStack is a fast file storage system raising by FB. The target of this rep is to implement the easiest part of HeyStack, including:

+ Physical File
  + To store millions of files in one big file with ordering
+ Index File
  + To look for file as fast as possible
  + Sometimes, if the index file is lost, we need to rebuild it from physical file
+ HTTP Interface
  + Can get/put/post/delete files using ``curl`` etc.

Although there are many 3rd dependences in rust, we want to make this project simple and easy build.


## Usage

The project is based on ``rustc 1.51``

+ build: ``cargo build``
+ build-release: ``cargo build --release``
+ run test: ``cargo test``

+ Start Server: ``cargo run start``
+ Close Server:
  + Send http.delete /sync to sync all index into disk
  + Press Ctrl+c
  + If you forget send Delete /sync, you can run ``cargo run reload`` to rebuild the index file from physical file, however, it may cause much time.
