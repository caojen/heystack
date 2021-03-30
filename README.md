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

## API

+ Post A New File
  + POST /file
  + Request.body == file.content
  + Return JSON like:
```json
{
  "key": 12,
  "size": 102,
  "offset": 28377,
  "flag": true
}
```
  + Note that posting file only store the data, the ``Content-Type`` will be ignored.

+ Get A File With Key
  + GET /file/{key}
  + After post a file, you can use ``key`` to get this file
  + Return the file as the response.body
  + Note that the ``Content-Type`` will be ignored when posting a file, you need to store this file's ``Content-Type``

+ Delete A File With Key
  + DELETE /file/{key}

+ Update A File With Key
  + PUT /file/{key}
  + Request.body == newfile.content
  + Modify file will modify the key, so return JSON like:
```json
{
  "key": 19,
  "size": 102,
  "offset": 28377,
  "flag": true
}
```
  + After old file deleted, that ``key`` will be removed and cannot be used anymore. You may need to store the new ``key`` and update your storage.

## Testing
Testing is being operating, please wait.
Some basic operations on disk has been test, you can run ``cargo test`` for testing.
